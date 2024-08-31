use crate::generate_conf::file_struct::ConfigFile;
use reqwest::Error;
use serde_json;
use std::{fs::File, io::Read};
use tokio::time::{self, Duration};

/// Function that starts Deployer. It makes
/// request to GitHub's REST API every 60 seconds.
/// As an argument it takes path to the config file.
pub async fn run(path: &str) {
    let config = serialise(path);
    dbg!(&config);
    println!("token {}", config.token);

    let api_url: String = url_fmt(&config.repository);
    ping(&api_url, &config.token).await.unwrap();
}

/// Formats URL from `github.com/author/their-repo` to
/// `https://api.github.com/repos/author/their-repo`.
/// Panics if URL is badly formatted.
fn url_fmt(url: &str) -> String {
    let base: &str = url;
    let base_list: Vec<&str> = base.split('/').collect();
    if base_list.len() < 3 {
        panic!("Bad repository URL.");
    }

    format!(
        "https://api.github.com/repos/{}/{}",
        base_list[base_list.len() - 2],
        base_list[base_list.len() - 1]
    )
}

/// Converts JSON data from the config file into
/// `ConfigFile` struct. Panics if fails to
/// either open the config file or to read it.
fn serialise(path: &str) -> ConfigFile {
    let mut buf: Vec<u8> = Vec::new();
    let mut file = File::open(path).expect("Failed to open the config file.");
    let size: usize = file
        .read_to_end(&mut buf)
        .expect("Failed to read the config file.");

    println!("size: {:?}", size);
    serde_json::from_slice(&buf).expect("Failed to parse json config.")
}

/// This function makes request to the GitHub's REST API.
/// Returns error if fails to send request or to parse response body.
async fn ping(api_url: &str, token: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();
    loop {
        let res = client
            .get(api_url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "request")
            .send()
            .await?;
        if res.status().is_success() {
            let body = res.text().await?;
            println!("Response: {}", body);
        } else {
            println!("Failed to fetch data: {}", res.status());
        }
        time::sleep(Duration::from_secs(60)).await;
    }
}
