use serde_derive::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, Write},
};

#[derive(Default, Serialize, Deserialize)]
struct ServiceFile {
    unit: Option<Vec<String>>,
    service: Option<Vec<String>>,
    install: Option<Vec<String>>,
}

#[derive(Default, Serialize, Deserialize)]
struct Service<'a> {
    name: &'a str,
    root_dit: &'a str,
    build_commands: Vec<&'a str>,
    environment: Option<Vec<&'a str>>,
    service_executable: &'a str,
    overwrite: bool,
    service_file: Option<ServiceFile>,
}

#[derive(Default, Serialize, Deserialize)]
struct ConfigFile<'a> {
    repository: &'a str,
    services_dir: &'a str,
    destination: &'a str,
    services: Vec<Service<'a>>,
}

pub fn generate(user_path: &str) -> Result<String, Error> {
    let mut path = String::from(user_path);
    validate_path(&mut path);
    match File::create_new(&path) {
        Ok(mut file) => {
            write_config(&mut file)?;
            Ok(path)
        }
        Err(e) => return Err(e),
    }
}

fn validate_path(path: &mut String) {
    if !path.contains("/") {
        path.push_str("/");
    }
    if !path.ends_with("deployer-config.json") {
        path.push_str("deployer-config.json");
    }
}

fn write_config(file: &mut File) -> Result<(), Error> {
    let config_file = ConfigFile::default();
    let data = serde_json::to_string_pretty(&config_file)?;
    writeln!(file, "{}", data)?;
    Ok(())
}
