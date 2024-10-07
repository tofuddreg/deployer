use chrono::{DateTime, Local};
use std::fs::OpenOptions;
use crate::log;
use std::{
    fs::File,
    io::{Result, Write},
};

pub mod file_struct;

/// Generate config file if it does not exist.
/// Any `user_path` is valid but blank.
///
/// Fails if file already exists.
/// # Example usage:
/// ```Rust
/// let path: &str = ".";
/// generate(path).unwrap();
/// ```
pub fn generate(user_path: &str) -> Result<()> {
    let mut path = String::from(user_path);
    validate_path(&mut path);
    match create_file(&path) {
        Ok(mut file) => {
            write_config(&mut file)?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn create_file(path: &str) -> Result<File> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    Ok(file)
}

/// Used in `generate(user_path: &str)` function
/// to format path to the file.
pub fn validate_path(path: &mut String) {
    if !path.ends_with("deployer-config.jsonc") {
        if !path.ends_with("/") {
            path.push_str("/");
        }
        path.push_str("deployer-config.jsonc");
    }
    log!("path: {}", path);
}

/// This function is for generating default deployer
/// configuration file structure. Used when deployer
/// generates configuration file for the first time.
///
/// The function will return an error if either fails
/// to serialise structure into json or to write the data
/// into the file.
fn write_config(file: &mut File) -> Result<()> {
    let config_file = file_struct::ConfigFile::default();
    let data = serde_json::to_string_pretty(&config_file)?;
    writeln!(file, "{}", data)?;
    file.flush()?;
    Ok(())
}
