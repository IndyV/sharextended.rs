use clap::Parser;

use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use std::path::PathBuf;
use std::{borrow::Cow, time::Duration};

lazy_static! {
    pub static ref SHAREX_URL: &'static str = "https://getsharex.com/";
    pub static ref REPO_URL: &'static str = "https://github.com/IndyV/sharextended";
}

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "IndyV", about = None, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(clap::Subcommand, Clone, Debug)]
pub enum Command {
    #[clap(about = "Delete all screenshots that have been uploaded to Imgur")]
    PurgeOnline {
        #[clap(short, long)]
        path: Option<PathBuf>,
    },
}

pub fn setup_spinner(msg: impl Into<Cow<'static, str>>) -> ProgressBar {
    let spinner: ProgressBar = ProgressBar::new_spinner().with_message(msg);
    spinner.enable_steady_tick(Duration::from_millis(1000));
    spinner
}

pub fn setup_progressbar(items: usize) -> ProgressBar {
    let progress_bar = ProgressBar::new(items as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("##-"),
    );
    progress_bar
}

pub fn open_webpage(url: &str) {
    open::that(url).unwrap_or_else(|_| panic!("Unable to open webpage {}", &url));
}

pub fn is_interactive() -> bool {
    let args = Args::parse();
    match args.command {
        None => true,
        _ => false,
    }
}
