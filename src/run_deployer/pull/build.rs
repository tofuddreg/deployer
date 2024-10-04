// Parse services, search for "key-files", build the project.
// The "key-files" are files that are important for the project
// such as "package.json", "gleam.toml" or "Cargo.toml".

use crate::generate_conf::file_struct::Service;
use std::process::Command;
use std::{
    fmt::Display,
    io::{Error, ErrorKind, Result},
    path::Path,
    process::ExitStatus,
};
use walkdir::{DirEntry, WalkDir};

enum KeyFile {
    Gleam,
    Rust,

    // what if user uses Bun which is also using package.json?
    // I mean... I should probably consider changing its name, maybe?
    NodeJS,
}

impl KeyFile {
    fn value<'a>(&self) -> &'a str {
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
pub fn build(_services: &[Service]) -> Result<()> {
    let path = Path::new("/Users/killer-whale/Desktop/test-destination/15_Sep_2024_1021");
    // for service in services {}
    let key_file = list_directories(&path)?;
    println!(
        "Found a key file ({}) in {}",
        key_file.1,
        key_file.0.path().display()
    );

    // todo: replace with `match` (maybe)
    if key_file.1.cmp(KeyFile::Rust) {
        let path = key_file
            .0
            .path()
            .parent()
            .expect("Failed to get file's parent directory");
        let status = build_rust(path);
        println!(
            "Build command has finished with status: {}",
            status.expect("Failed to get exit status code")
        );

        // todo: Change hardcoded
        let build = Path::new("/Users/killer-whale/Desktop/test-destination/15_Sep_2024_1021/target/release");
        let dest = Path::new("/Users/killer-whale/Desktop/test-destination/15_Sep_2024_1021__build");
        move_build(build, dest).expect("Failed to move project to the dest. dir.");
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
fn move_build(project: &Path, destination: &Path) -> Result<ExitStatus> {
    let mut cmd = Command::new("mv")
        .arg(project)
        .arg(destination)
        .current_dir(project)
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
