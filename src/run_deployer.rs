use crate::generate_conf::file_struct::{Commit, ConfigFile};
use chrono::{prelude::DateTime, Local};
use git2::Repository;
use reqwest::{Client, Error, Response};
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
    const INVALID_URL: &str = "Invalid repository URL!";
    let list: Vec<&str> = url.split('/').collect();
    if list.len() != 3 {
        panic!("{}", INVALID_URL);
    }

    let domain = list[0];
    let author = list[1];
    let repository = list[2];

    if domain != "github.com" {
        panic!("Invalid repository domain!");
    }

    if author.is_empty() || repository.is_empty() {
        panic!("{}", INVALID_URL);
    }

    if branch.is_empty() {
        panic!("No main branch specified!");
    }

    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        author, repository, branch
    );

    RepositoryInfo {
        url,
        author: author.to_owned(),
        name: repository.to_owned(),
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
        // Make request
        let token = format!("token {}", token);
        let res = send_request(&repository.url, &token, &client).await?;

        // Panic if some error
        if !res.status().is_success() {
            println!("Failed to fetch data: {}", res.status());
            continue;
        }

        let body = res.text().await?;
        let response: Commit = serde_json::from_str(&body).expect("Failed to parse response body");

        // Check for new commits
        if last_commit != response.sha {
            last_commit = response.sha.clone();
            let url = format!(
                "https://github.com/{}/{}.git",
                repository.author, repository.name
            );
            pull_repository(&url, root_dir).expect("Failed to fetch repository");
        }
        time::sleep(Duration::from_secs(60)).await;
    }
}

async fn send_request(url: &str, token: &str, client: &Client) -> Result<Response, Error> {
    let response = client
        .get(url)
        .header("Authorization", token)
        .header("User-Agent", "request")
        .send()
        .await?;
    Ok(response)
}

fn pull_repository(url: &str, root_dir: &str) -> Result<(), git2::Error> {
    let destination = append_time(&root_dir);

    // Pull repository
    match Repository::clone(url, &destination) {
        Ok(_) => {
            println!("Fetched from remote branch to {}", destination);
            Ok(())
        }
        Err(e) => match e.code() {
            git2::ErrorCode::Exists => {
                let new_dest = update_destination(true, destination, 1);
                println!("updated destination: {}", new_dest);
                Repository::clone(url, &new_dest).expect("Failed to fetch repository");
                Ok(())
            }
            _ => Err(e),
        },
    }
}

/// Recursive function. Checks if directory already exists
/// and appends available index to that folder so it is possible
/// to pull repository without issues.
///
/// Panics if fails to split
/// base path on underscores (_) into less than four pieces.
fn update_destination(exists: bool, mut from: String, index: i32) -> String {
    if !exists {
        return from;
    }

    let from_split = from.clone();
    let base_path: Vec<&str> = from_split.split("_").collect();

    if base_path.len() < 4 {
        panic!("Failed to format folder name. Try removing all repeating folders or try again in a minute.");
    }

    // assemble base of the destination path
    let index_str;
    from = format!(
        "{}_{}_{}_{}",
        base_path[0], base_path[1], base_path[2], base_path[3]
    );

    if index < 10 {
        index_str = format!("_0{}", index);
    } else {
        index_str = format!("_{}", index);
    }

    from.push_str(&index_str);
    update_destination(check_existance(&from), from, index + 1)
}

fn check_existance(path: &str) -> bool {
    let dir = path::Path::new(&path);
    dir.exists()
}

fn append_time(root: &str) -> String {
    let mut destination = String::from(root);
    let now: DateTime<Local> = Local::now();
    let formatted_date = now.format("%d_%b_%Y_%H%M").to_string();
    destination.push_str(&formatted_date);
    destination
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test destination modifier
    #[test]
    fn test_non_existent_path() {
        let non_existent_path = String::from("01_Sep_2024_1308");
        let result = update_destination(false, non_existent_path.clone(), 1);
        assert_eq!(result, non_existent_path);
    }

    #[test]
    #[should_panic(
        expected = "Failed to format folder name. Try removing all repeating folders or try again in a minute."
    )]
    fn test_invalid_path_format() {
        let invalid_path = String::from("01_Sep_2024");
        update_destination(true, invalid_path, 1);
    }

    #[test]
    fn test_valid_path_format() {
        let valid_path = String::from("01_Sep_2024_1307");
        let result = update_destination(true, valid_path, 1);

        // Ensure the format is correctly updated
        assert_eq!(result, "01_Sep_2024_1307_01");
    }

    // Test URL formatter
    #[test]
    fn test_valid_url_fmt() {
        let url = "github.com/tofuddreg/deployer";
        let branch = "master";
        let repository_info = url_fmt(url, branch);
        assert_eq!(
            repository_info.url,
            "https://api.github.com/repos/tofuddreg/deployer/commits/master"
        );
        assert_eq!(repository_info.author, "tofuddreg");
        assert_eq!(repository_info.name, "deployer");
    }

    #[test]
    #[should_panic(expected = "Invalid repository domain!")]
    fn test_invalid_domain_url_fmt() {
        let url = "gitlab.com/tofuddreg/deployer";
        let branch = "master";
        url_fmt(url, branch);
    }

    #[test]
    #[should_panic(expected = "Invalid repository URL!")]
    fn test_invalid_author_url_fmt() {
        let url = "github.com//deployer";
        let branch = "master";
        url_fmt(url, branch);
    }

    #[test]
    #[should_panic(expected = "Invalid repository URL!")]
    fn test_invalid_name_url_fmt() {
        let url = "github.com/tofuddreg/";
        let branch = "master";
        url_fmt(url, branch);
    }
}
