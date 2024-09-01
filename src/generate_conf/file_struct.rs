use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub sha: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceFile {
    pub unit: Option<Vec<String>>,
    pub service: Option<Vec<String>>,
    pub install: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub root_dir: String,
    pub build_commands: Vec<String>,
    pub environment: Option<Vec<String>>,
    pub service_executable: String,
    pub overwrite: bool,
    pub service_file: Option<ServiceFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub repository: String,
    pub branch: String,
    pub token: String,
    pub services_dir: String,
    pub destination: String,
    pub services: Vec<Service>,
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
            branch: "main".to_owned(),
            repository: "https://github.com/your-repository/link".to_owned(),
            token: "YOUR-GITHUB-TOKEN-HERE".to_owned(),
            services_dir: "/lib/systemd/system".to_owned(),
            destination: "/var/www".to_owned(),
            services: vec![Service::default()],
        }
    }
}
