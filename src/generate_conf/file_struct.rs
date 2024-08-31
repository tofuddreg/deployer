use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceFile<'a> {
    #[serde(borrow)]
    unit: Option<Vec<&'a str>>,
    service: Option<Vec<&'a str>>,
    install: Option<Vec<&'a str>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service<'a> {
    name: &'a str,
    root_dir: &'a str,
    build_commands: Vec<&'a str>,
    environment: Option<Vec<&'a str>>,
    service_executable: &'a str,
    overwrite: bool,
    service_file: Option<ServiceFile<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile<'a> {
    repository: &'a str,
    services_dir: &'a str,
    destination: &'a str,
    services: Vec<Service<'a>>,
}

impl<'a> Default for ServiceFile<'a> {
    fn default() -> Self {
        ServiceFile {
            unit: Some(vec!["Description=service-name"]),
            service: None,
            install: None,
        }
    }
}

impl<'a> Default for Service<'a> {
    fn default() -> Self {
        Service {
            name: "service-name",
            root_dir: "/var/www/your_repository/backend",
            build_commands: vec![
                "gleam build",
                "mv ${root_dir}/build/erlang-shipment ${destination}",
            ],
            environment: Some(vec![
                "ENVIRONMENT=production",
                "SECRET=some_secret_key",
                "PORT=4020",
            ]),
            service_executable: "entrypoint.sh",
            overwrite: true,
            service_file: Some(ServiceFile::default()),
        }
    }
}

impl<'a> Default for ConfigFile<'a> {
    fn default() -> Self {
        ConfigFile {
            repository: "https://github.com/your-repository/link",
            services_dir: "/lib/systemd/system",
            destination: "/var/www",
            services: vec![Service::default()],
        }
    }
}
