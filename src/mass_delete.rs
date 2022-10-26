use crate::util;

use ansi_term::{self, Colour};
use clap::Parser;
use directories::{BaseDirs, UserDirs};
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::thread;
use std::{
    io::Read,
    path::{Path, PathBuf},
    time::Duration,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HistoryItem {
    #[serde(rename = "FileName")]
    pub file_name: String,
    #[serde(rename = "FilePath")]
    pub file_path: Option<String>,
    #[serde(rename = "DateTime")]
    pub date_time: String,
    #[serde(rename = "Type")]
    pub type_field: String,
    #[serde(rename = "Host")]
    pub host: String,
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "ThumbnailURL")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "DeletionURL")]
    pub deletion_url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tags {
    #[serde(rename = "WindowTitle")]
    pub window_title: Option<String>,
    #[serde(rename = "ProcessName")]
    pub process_name: String,
}

pub async fn handler(pathflag: Option<PathBuf>) -> Result<()> {
    // TODO
    // If a pathflag is Some() it means it's passed via a flag, use that path and skip the prompting
    // Otherwise, prompt the user to select a default path (depending on type of installation), or tinyfiledialog for a path, or manual input

    let path = match pathflag {
        Some(pathflag) => pathflag,
        None => get_path_input(),
    };

    let file = prompt_history_file(&path);

    if !Path::new(&file).exists() {
        eprintln!(
            "{} The directory does not exist. Given path: {:?}",
            Colour::Red.paint("Error:"),
            &file
        );
        std::process::exit(1);
    }

    let history_urls = get_history_urls(&file);

    delete_urls(history_urls).await?;

    Ok(())
}

fn prompt_history_file(path: &Path) -> PathBuf {
    // TODO: Use dialoguer select to prompt the user to select the history file, either with tinyfiledialogs or manual input

    return tinyfiledialogs::open_file_dialog(
        "Choose where sharex history is stored",
        path.to_str().unwrap(),
        Some((&["History.json", "*.json"], "History.json")),
    )
    .map_or_else(
        || {
            eprintln!("No file selected, exiting...");
            std::process::exit(1);
        },
        PathBuf::from,
    );
}

fn get_history_urls(path: &PathBuf) -> Result<Vec<String>> {
    let spinner = util::setup_spinner("Reading and parsing JSON...");

    let history_json = read_history_json(path)?;
    let history_items = parse_history_json(&history_json)?;
    let deletion_urls = get_deletion_urls(&history_items);

    // spinner.finish_and_clear();
    spinner.finish_with_message(format!("Done! {} items found", deletion_urls.len()));
    Ok(deletion_urls)
}

fn get_path_input() -> PathBuf {
    let args = util::Args::parse();
    let path = match args.command {
        Some(util::Command::MassDelete { path }) => path,
        None => None,
    };

    let default_history_path = get_default_history_path();

    match path {
        Some(path) => path,
        None => default_history_path,
    }
}

fn get_default_history_path() -> PathBuf {
    let document_directory: PathBuf = UserDirs::new().map_or_else(
        || BaseDirs::new().unwrap().home_dir().join("Documents"),
        |user_dirs| user_dirs.document_dir().unwrap().to_path_buf(),
    );
    let default_history_path: PathBuf = document_directory.join("ShareX").join("History.json");

    default_history_path
}

fn read_history_json(path: &PathBuf) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Since ShareX history is invalid JSON we add brackets to make it valid JSON
    contents = format!("[{}]", contents);
    Ok(contents)
}

fn parse_history_json(json: &str) -> Result<Vec<HistoryItem>, serde_json::Error> {
    let history_items: Vec<HistoryItem> = serde_json::from_str(json)?;
    Ok(history_items)
}

fn get_deletion_urls(items: &[HistoryItem]) -> Vec<String> {
    items
        .iter()
        .filter(|item| item.deletion_url.is_some() && item.deletion_url != Some("".to_string()))
        .map(|item| item.deletion_url.clone().unwrap())
        .collect()
}

async fn delete_urls(deletion_urls: Result<Vec<String>>) -> Result<()> {
    let deletion_urls = deletion_urls?;
    let progress_bar = util::setup_progressbar(deletion_urls.len());
    // progress_bar.enable_steady_tick(Duration::from_millis(500)); // This only visually updates the ticker once every 500ms instead of when the tick occurs

    // ? Maybe use Rayon to parallelize the requests and run them through public proxies to prevent rate limiting?
    for url in deletion_urls {
        let (remaining, limit, reset) = send_deletion(&url).await?;
        println!(
            "Remaining: {} Limit: {} Reset: {}",
            Colour::Green.paint(remaining),
            Colour::Yellow.paint(limit),
            Colour::Red.paint(reset)
        );
        progress_bar.inc(1);
        thread::sleep(Duration::from_millis(100));
    }
    // let client = reqwest::Client::new();

    // let mut futures = Vec::new();

    // for url in deletion_urls {
    //     let future = client.delete(&url).send();
    //     futures.push(future);
    // }

    // let results = futures::future::join_all(futures).await;

    // for result in results {
    //     match result {
    //         Ok(response) => {
    //             if response.status().is_success() {
    //                 println!("Deleted {}", response.url());
    //             } else {
    //                 eprintln!("Failed to delete {}", response.url());
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to delete {}", e);
    //         }
    //     }
    // }
    progress_bar.finish_with_message("Done!");

    Ok(())
}

async fn send_deletion(url: &str) -> Result<(String, String, String)> {
    let client = reqwest::Client::new();
    let params = [("confirm", true)];
    let resp = client.post(url).form(&params).send().await?;

    println!("{:#?}", resp);

    match resp.status() {
        reqwest::StatusCode::OK => {
            println!("OK");
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            println!("TOO MANY REQUESTS");
        }
        reqwest::StatusCode::BAD_GATEWAY => {
            println!("BAD GATEWAY");
        }
        _ => {
            println!("Not OK");
        }
    }

    // I don't understand Rust enough so the stuff below looks kinda cursed
    let headers = resp.headers().clone();
    let remaining = headers
        .get("x-post-rate-limit-remaining")
        .unwrap()
        .to_str()?
        .to_owned();
    let limit = headers
        .get("x-post-rate-limit-limit")
        .unwrap()
        .to_str()?
        .to_owned();
    let reset = headers
        .get("x-post-rate-limit-Reset")
        .unwrap()
        .to_str()?
        .to_owned();

    println!(
        "Remaining: {} Limit: {} Reset: {}",
        Colour::Green.paint(&remaining),
        Colour::Yellow.paint(&limit),
        Colour::Red.paint(&reset)
    );

    print!("{:?}", headers);

    Ok((remaining, limit, reset))
}
