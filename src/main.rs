mod purge_online;
mod util;

use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Select};
use eyre::Result;

#[tokio::main]
async fn main() {
    let args = util::Args::parse();
    let command = args.command;

    match command {
        Some(util::Command::PurgeOnline { path }) => {
            purge_online::handler(path).await.unwrap();
        }
        None => {
            show_menu().await.unwrap();
        }
    }
}

async fn show_menu() -> Result<()> {
    loop {
        println!();
        let menu_response: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick an option (use arrow keys to select, enter to confirm)")
            .items(&[
                "Purge online history items",
                "Open ShareX Website",
                "View source code (GitHub)",
                "Exit",
            ])
            .default(0)
            .interact()
            .unwrap();
        println!();

        handle_option(menu_response).await?;
    }
}

async fn handle_option(number: usize) -> Result<()> {
    match number {
        0 => {
            purge_online::handler(None).await?;
        }
        1 => {
            util::open_webpage(util::SHAREX_URL.to_string().as_str());
        }
        2 => {
            util::open_webpage(util::REPO_URL.to_string().as_str());
        }
        3 => {
            std::process::exit(0);
        }
        _ => {
            println!("Invalid option");
        }
    }

    Ok(())
}
