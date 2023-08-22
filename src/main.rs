use std::fmt::Display;

use clap::{Args, Parser, Subcommand};
use color_eyre::{eyre::ContextCompat, Result};

pub mod api;
pub mod handler;
pub mod utils;

#[derive(Parser, Debug)]
#[command(author = "Maulik Patel", version, about = "Very simple git user switcher", long_about = None)]
#[command(propagate_version = true)]
pub struct GusCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all profile and select
    List,
    /// Add new profile
    A(Profile),

    /// Add new profile interactively
    Add,

    /// Activate profile
    Ac(ActivateArgs),
    /// Delete profile
    Delete,
    /// Edit profile
    Edit,
}

#[derive(Args, Debug)]
pub struct Profile {
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    email: String,
    #[arg(short, long)]
    profile: String,
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{} , {}]", self.profile, self.name, self.email)
    }
}
#[derive(Args, Debug)]
pub struct ActivateArgs {
    profile: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let curr_path = std::env::current_dir()?;
    let curr_path = curr_path
        .to_str()
        .with_context(|| "Error getting string from PathBuf")?;
    let config_path = utils::get_sqlite_path().await?;
    let config_path = config_path
        .to_str()
        .with_context(|| "Error getting string from PathBuf")?;
    let db = utils::get_db_pool(&config_path).await?;
    let cli = GusCli::parse();
    let gus_handler = handler::GusHandler::new(db, cli, curr_path);
    gus_handler.handle_commands().await
}
