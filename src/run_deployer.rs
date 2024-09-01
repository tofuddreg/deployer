use crate::generate_conf::file_struct::{Commit, ConfigFile};
use chrono::{prelude::DateTime, Local};
use git2::Repository;
use reqwest::Error;
use serde_json;
use std::{fs::File, io::Read, path};
use tokio::time::{self, Duration};

/// Local struct. Used to pass
/// these three fields across functions.
struct RepositoryInfo {
    pub url: String,
    pub author: String,
    pub name: String,
}

/// Function that starts Deployer. It makes
/// request to GitHub's REST API every 60 seconds.
/// As an argument it takes path to the config file.
///
/// Panics if directory does not exist, no services
/// specified, token/repository/branch is not specified or
/// repository link is invalid.
pub async fn run(path: &str) {
    let config = deserialise(path);

    if config.token == "" || config.token == "YOUR-GITHUB-TOKEN-HERE" {
        panic!("Github token is not specified!");
    }

    if config.repository == "" || config.repository == "https://github.com/your-repository/link" {
        panic!("Github repository is not specified!");
    }

    let repository = url_fmt(&config.repository, &config.branch);

    // NOTE: Only global directories are valid yet
    validate_dir(&config.services_dir);
    validate_dir(&config.destination);

    if config.services.len() == 0 {
        panic!("No services specified!");
    }

    ping(&config.token, &config.destination, &repository)
        .await
        .unwrap();
}

/// Formats URL from `github.com/author/their-repo` to
/// `https://api.github.com/repos/author/their-repo/commits`.
/// Panics if URL is badly formatted.
fn url_fmt(url: &str, branch: &str) -> RepositoryInfo {
    let list: Vec<&str> = url.split('/').collect();

    if list[0] != "github.com" || list.len() != 3 {
        panic!("Bad repository URL!");
    }

    if list[1] == "" || list[2] == "" {
        panic!("Bad repository URL!");
    }

    if branch == "" {
        panic!("No main branch specified!");
    }

    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        list[list.len() - 2],
        list[list.len() - 1],
        branch
    );

    RepositoryInfo {
        url,
        author: list[list.len() - 2].to_owned(),
        name: list[list.len() - 1].to_owned(),
    }
}

/// Check if specified directory exists.
/// Panics if it does not.
fn validate_dir(dir: &str) {
    let path = path::Path::new(dir);
    if !path.exists() {
        panic!("Path \"{}\" does not exist!", dir);
    }
}

/// Converts JSON data from the config file into
/// `ConfigFile` struct. Panics if fails to
/// either open the config file or to read it.
fn deserialise(path: &str) -> ConfigFile {
    let mut buf = Vec::new();
    let mut file = File::open(path).expect("Failed to open the config file");
    file.read_to_end(&mut buf)
        .expect("Failed to read the config file");

    serde_json::from_slice(&buf).expect("Failed to parse json config")
}

/// This function makes request to the GitHub's REST API.
/// Returns error if fails to send request or to parse response body.
async fn ping(token: &str, root_dir: &str, repository: &RepositoryInfo) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let mut last_commit = String::from("");
    loop {
        let res = client
            .get(&repository.url)
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "request")
            .send()
            .await?;

        if !res.status().is_success() {
            println!("Failed to fetch data: {}", res.status());
            continue;
        }

        let body = res.text().await?;
        let response: Commit = serde_json::from_str(&body).expect("Failed to parse response body");

        if last_commit != response.sha {
            println!("New commit found!");
            last_commit = response.sha.clone();

            // `git pull` logic here:
            let url = format!(
                "https://github.com/{}/{}.git",
                repository.author, repository.name
            );

            let destination = append_time(&root_dir);

            match Repository::clone(&url, &destination) {
                Ok(_) => println!("Fetched from remote branch to {}", destination),
                Err(e) => match e.code() {
                    git2::ErrorCode::Exists => {
                        let new_dest = update_destination(&destination, 1);
                        println!("NEW DESTINATION: {}", new_dest);
                        Repository::clone(&url, &new_dest).unwrap();
                    }
                    _ => panic!("Failed to fetch repository: {e}"),
                },
            }
        }
        time::sleep(Duration::from_secs(60)).await;
    }
}

// TODO: fix this function.
//   The problem is that it appends _2, _3, ...
//   to its original name (such as 01_Sep_0938_1_2_3)
fn update_destination(from: &str, index: i32) -> String {
    let path = path::Path::new(from);
    if !path.exists() {
        return from.to_owned();
    }

    let mut dest = String::from(from);
    let index_str = format!("_{}", index);
    dest.push_str(&index_str);
    update_destination(&dest, index + 1)
}

fn append_time(root: &str) -> String {
    let mut destination = String::from(root);
    let now: DateTime<Local> = Local::now();
    let formatted_date = now.format("%d_%b_%Y_%H%M").to_string();
    destination.push_str(&formatted_date);
    destination
}
