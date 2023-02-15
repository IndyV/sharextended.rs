use crate::util;

use ansi_term::{self, Colour};
use chrono::prelude::*;
use chrono::Duration;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use directories::{BaseDirs, UserDirs};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{io::Read, path::PathBuf};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct HistoryItem {
    pub file_name: String,
    pub file_path: Option<String>,
    pub date_time: DateTime<Local>,
    #[serde(rename = "Type")]
    pub type_field: String,
    pub host: String,
    pub tags: Option<Tags>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "ThumbnailURL")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "DeletionURL")]
    pub deletion_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Tags {
    pub window_title: Option<String>,
    pub process_name: String,
}

pub async fn handler(pathflag: Option<PathBuf>) -> Result<()> {
    let path = match pathflag {
        Some(pathflag) => Some(pathflag),
        None => prompt_history_file(),
    };

    let history_file = match path {
        Some(path) => path,
        None => {
            return Ok(());
        }
    };

    if !PathBuf::from(&history_file).exists() {
        eprintln!("A valid path was not specified. Please try again.");
        return Ok(());
    }

    let history_urls = get_history_urls(history_file);

    delete_urls(history_urls).await?;

    Ok(())
}

fn prompt_history_file() -> Option<PathBuf> {
    let default_path = get_default_history_path();

    println!();
    let menu_response: usize = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick an option")
        .items(&[
            "Use default path",
            "Use file picker",
            "Manual input",
            "Cancel",
        ])
        .default(0)
        .interact()
        .unwrap();
    println!();

    match menu_response {
        0 => Some(default_path.clone()),
        1 => Some(
            tinyfiledialogs::open_file_dialog(
                "Choose where sharex history is stored",
                default_path.to_str().unwrap(),
                Some((&["History.json", "*.json"], "History.json")),
            )
            .map_or_else(
                || {
                    eprintln!("No file selected, exiting...");
                    std::process::exit(1);
                },
                PathBuf::from,
            ),
        ),
        2 => loop {
            let input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter path to history file")
                .default(default_path.to_str()?.to_string())
                .interact()
                .unwrap();
            let path = PathBuf::from(input);
            if path.exists() {
                return Some(path);
            } else {
                println!("Invalid path given. Please try again.");
            }
        },
        3 => {
            println!("Canceling operation...");
            None
        }
        _ => {
            println!("Invalid option");
            std::process::exit(1);
        }
    }
}

fn get_history_urls(path: PathBuf) -> Result<Vec<String>> {
    let spinner = util::setup_spinner("Reading and parsing JSON...");

    let history_json = read_history_json(path)?;
    let history_items = parse_history_json(&history_json)?;
    let deletion_urls = filter_deletion_urls(&history_items, None);

    spinner.finish_with_message(format!("Done! {} items found", deletion_urls.len()));

    Ok(deletion_urls)
}

fn get_default_history_path() -> PathBuf {
    let document_directory: PathBuf = UserDirs::new().map_or_else(
        || BaseDirs::new().unwrap().home_dir().join("Documents"),
        |user_dirs| user_dirs.document_dir().unwrap().to_path_buf(),
    );

    document_directory.join("ShareX").join("History.json")
}

fn read_history_json(path: PathBuf) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Since ShareX history is invalid JSON we add brackets to make it valid JSON
    contents = format!("[{}]", contents);
    Ok(contents)
}

fn parse_history_json(json: &str) -> Result<Vec<HistoryItem>> {
    let history_items: Vec<HistoryItem> = serde_json::from_str(json)?;
    Ok(history_items)
}

fn filter_deletion_urls(items: &[HistoryItem], from_date: Option<DateTime<Local>>) -> Vec<String> {
    /*
      TODO: Filter out items that don't have a deletion url but return the type like &[HistoryItem]
      This is because we need to match on HistoryItem Host when calling send_deletion
    */

    items
        .iter()
        .filter(|item| {
            item.deletion_url.is_some()
                && item.deletion_url != Some("".to_string())
                && item.host == *"Imgur"
                && item.date_time > from_date.unwrap_or_else(|| Local::now() - Duration::days(1))
        })
        .map(|item| item.deletion_url.clone().unwrap())
        .collect()
}

async fn delete_urls(deletion_urls: Result<Vec<String>>) -> Result<()> {
    let deletion_urls = deletion_urls?;
    if deletion_urls.is_empty() {
        println!("{}", Colour::Yellow.bold().paint("No items to delete!"));
        return Ok(());
    }

    if deletion_urls.len() > 1250 {
        println!(
            "{}",
            Colour::Yellow
                .bold()
                .paint("Amount of items to delete is too high for Imgur API, canceling...")
        );
        return Ok(());
    }

    let progress_bar = util::setup_progressbar(deletion_urls.len());

    let client = reqwest::Client::new();

    let mut futures = Vec::new();

    // Maybe limit size to 1,250 POST requests per hour
    // https://api.imgur.com/#limits
    for url in deletion_urls {
        //    let future = client.delete(&url).send();
        let params = [("confirm", true)];
        let future = client.post(&url).form(&params).send();
        futures.push(future);
    }

    let results = futures::future::join_all(futures).await;

    //     // I don't understand Rust enough so the stuff below looks kinda cursed
    //     let headers = resp.headers().clone();
    //     let remaining = headers
    //         .get("x-post-rate-limit-remaining")
    //         .unwrap()
    //         .to_str()?
    //         .to_owned();
    //     let limit = headers
    //         .get("x-post-rate-limit-limit")
    //         .unwrap()
    //         .to_str()?
    //         .to_owned();
    //     let reset = headers
    //         .get("x-post-rate-limit-Reset")
    //         .unwrap()
    //         .to_str()?
    //         .to_owned();

    for result in results {
        progress_bar.inc(1);
        // Check the headers here for rate limits?
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    println!("Deleted {}", response.url());
                } else {
                    eprintln!("Failed to delete {}", response.url());
                }
            }
            Err(e) => {
                eprintln!("Failed to delete {}", e);
            }
        }
    }
    progress_bar.finish_with_message("Done!");

    Ok(())
}
