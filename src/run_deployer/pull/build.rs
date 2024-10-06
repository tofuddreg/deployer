// Parse services, search for "key-files", build the project.
// The "key-files" are files that are important for the project
// such as "package.json", "gleam.toml" or "Cargo.toml".

use crate::run_deployer::pull::{DateTime, Local};
use std::process::Command;
use std::{
    fmt::Display,
    io::{Error, ErrorKind, Result},
    path::Path,
    process::ExitStatus,
};
use walkdir::{DirEntry, WalkDir};
use crate::log;

enum KeyFile {
    Gleam,
    Rust,

    // what if user uses Bun which is also using package.json?
    // I mean... I should probably consider changing its name, eh?
    NodeJS,
}

impl KeyFile {
    fn value(&self) -> &str {
        match self {
            KeyFile::Gleam => "gleam.toml",
            KeyFile::Rust => "Cargo.toml",
            KeyFile::NodeJS => "package.json",
        }
    }
    fn cmp(&self, value: KeyFile) -> bool {
        if self.value() == value.value() {
            return true;
        }
        false
    }
}

impl Display for KeyFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// Build services looking at their `KeyFiles`
pub fn build(
    service_path: &Path,
    build_dir: &Path,
    service_name: String
) -> Result<()> {
    let key_file = list_directories(service_path)?;
    log!("Found a key file ({}) in {}", key_file.1, key_file.0.path().display());
    if key_file.1.cmp(KeyFile::Rust) {
        let path = key_file
            .0
            .path()
            .parent()
            .expect("Failed to get file's parent directory");
        let status = build_rust(path);
        log!(
            "Build command has finished with status: {}",
            status.expect("Failed to get exit status code")
        );

        let rs_build_path =
            format!("{}/target/release", path
                .to_str().expect("Failed to get rust build path"));

        move_build(Path::new(&rs_build_path), build_dir, service_name)?;
    } else if key_file.1.cmp(KeyFile::Gleam) {
        todo!();
    } else if key_file.1.cmp(KeyFile::NodeJS) {
        todo!();
    } else {
        return Err(Error::new(ErrorKind::Other, "Failed to compare KeyFile."));
    }
    Ok(())
}

/// Move built file to the specified directory
fn move_build(project: &Path, destination: &Path, service_name: String) -> Result<ExitStatus> {
    let tmp = format!("{}/{}", destination.to_str().unwrap(), service_name);
    let destination = Path::new(&tmp);
    if Path::exists(destination) {
        let mut cmd = Command::new("rm")
            .arg("-rf")
            .arg(destination)
            .spawn()
            .expect("Failed to remove existing service.");
        cmd.wait()?;
    }
    let mut cmd = Command::new("mv")
        .arg(project)
        .arg(destination)
        .spawn()
        .expect("Failed to move release");
    cmd.wait()
}

/// Panics if fails to spawn the CMD.
fn build_rust(path: &Path) -> Result<ExitStatus> {
    let mut cmd = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(path)
        .spawn()
        .expect("Failed to build Rust project");
    cmd.wait()
}

/// Search for supported "key-files"
fn list_directories(path: &Path) -> Result<(DirEntry, KeyFile)> {
    for entry in WalkDir::new(path).follow_links(true).into_iter() {
        let tmp = entry?;
        if tmp.path().is_file() {
            let file_name = tmp.path().file_name().unwrap().to_str().unwrap();
            match file_name {
                "package.json" => return Ok((tmp, KeyFile::NodeJS)),
                "Cargo.toml" => return Ok((tmp, KeyFile::Rust)),
                "gleam.toml" => return Ok((tmp, KeyFile::Gleam)),
                _ => continue,
            }
        }
    }
    Err(Error::new(
        ErrorKind::Other,
        "Couldn't find any supported key-file.",
    ))
}
