use crate::generate_conf::file_struct::Commit;
use chrono::{prelude::DateTime, Local};
use git2::Repository;
use reqwest::{Client, Error, Response};
use std::path::Path;
use tokio::time::{self, Duration};

/// Local struct. Used to pass
/// these three fields across functions.
pub struct RepositoryInfo {
    pub url: String,
    pub author: String,
    pub name: String,
}

/// This function makes request to the GitHub's REST API.
/// Returns error if fails to send request or to parse response body.
pub async fn ping(token: &str, root_dir: &str, repository: &RepositoryInfo) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let mut last_commit = String::from("");
    loop {
        // Make request
        let res = send_request(&repository.url, token, &client).await?;

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
pub fn update_destination(exists: bool, mut path: String, index: i32) -> String {
    if !exists {
        return path;
    }

    let from_split = path.clone();
    let base_path: Vec<&str> = from_split.split("_").collect();
    destination_fmt(&base_path, &mut path, index);
    update_destination(check_existance(&path), path, index + 1)
}

/// External parameter `exists: bool` in `update_destination`
/// needed in function's tests. Therefore I had to make this
/// function for `update_destination`'s internal use.
fn check_existance(path: &str) -> bool {
    let dir = Path::new(&path);
    dir.exists()
}

/// Append suffix to the path (example: `01_Jan_2025_1046_01`).
///
/// Panics if fails to split
/// base path on underscores (_) into less than four pieces.
fn destination_fmt(base_path: &[&str], path: &mut String, index: i32) {
    if base_path.len() < 4 {
        panic!("Failed to format folder name. Try removing all repeating folders or try again in a minute.");
    }
    // assemble base of the destination path
    // "01" "Jan" "2025" "1046"
    *path = format!(
        "{}_{}_{}_{}",
        base_path[0], base_path[1], base_path[2], base_path[3]
    );
    path.push_str("_");
    if index < 10 {
        path.push('0');
    }
    path.push_str(&index.to_string());
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
}
