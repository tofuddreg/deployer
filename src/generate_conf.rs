use serde_derive::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Error, Write},
};

#[derive(Serialize, Deserialize)]
struct ServiceFile {
    unit: Option<Vec<String>>,
    service: Option<Vec<String>>,
    install: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct Service {
    name: String,
    root_dir: String,
    build_commands: Vec<String>,
    environment: Option<Vec<String>>,
    service_executable: String,
    overwrite: bool,
    service_file: Option<ServiceFile>,
}

#[derive(Serialize, Deserialize)]
struct ConfigFile {
    repository: String,
    services_dir: String,
    destination: String,
    services: Vec<Service>,
}

impl Default for ServiceFile {
    fn default() -> Self {
        ServiceFile {
            unit: Some(vec!["Description=service-name".to_owned()]),
            service: None,
            install: None,
        }
    }
}

impl Default for Service {
    fn default() -> Self {
        Service {
            name: "service-name".to_owned(),
            root_dir: "/var/www/your_repository/backend".to_owned(),
            build_commands: vec![
                "gleam build".to_owned(),
                "mv ${root_dir}/build/erlang-shipment ${destination}".to_owned(),
            ],
            environment: Some(vec![
                "ENVIRONMENT=production".to_owned(),
                "SECRET=some_secret_key".to_owned(),
                "PORT=4020".to_owned(),
            ]),
            service_executable: "entrypoint.sh".to_owned(),
            overwrite: true,
            service_file: Some(ServiceFile::default()),
        }
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            repository: "https://github.com/your-repository/link".to_owned(),
            services_dir: "/lib/systemd/system".to_owned(),
            destination: "/var/www".to_owned(),
            services: vec![Service::default()],
        }
    }
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
