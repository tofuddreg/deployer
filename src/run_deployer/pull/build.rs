// Parse services', search for "key-files", build the project.
// The "key-files" are file that are important for the project
// such as "package.json", "gleam.toml" or "Cargo.toml".

use crate::generate_conf::file_struct::Service;
use std::{fmt::Display, io::{Error, ErrorKind, Result}, path::Path};
use walkdir::{DirEntry, WalkDir};
use std::process::Command;

enum KeyFile {
    Gleam,
    Rust,

    // what if user uses Bun which is also using package.json?
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
            return true
        }
        false
    }
}

impl Display for KeyFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

pub fn build(_services: &[Service]) -> Result<()> {
    let path = Path::new("/Users/killer-whale/Desktop/test-destination/15_Sep_2024_1021");
    //for service in services {}
    let key_file = list_directories(&path)?;
    println!("Found a key file ({}) in {}", key_file.1, key_file.0.path().display());

    // todo: replace with `match`
    if key_file.1.cmp(KeyFile::Rust) {
        println!("key_file dir: {}", key_file.0.path()
            .parent().unwrap().display());
        let mut cmd = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .current_dir(&key_file.0.path().parent().unwrap())
            .spawn()
            .expect("Failed to build the Rust project");
        let status = cmd.wait().unwrap();
        println!("Build command has finished with status: {}", status);
    } else if key_file.1.cmp(KeyFile::Gleam) {
        todo!();
    } else if key_file.1.cmp(KeyFile::NodeJS) {
        todo!();
    } else {
        return Err(Error::new(ErrorKind::Other, "Failed to compare KeyFile."))
    }
    Ok(())
}

// Search for supported "key-files"
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
    Err(Error::new(ErrorKind::Other, "Couldn't find any supported key-file."))
}

