use color_eyre::{
    eyre::{bail, Context, ContextCompat},
    Result,
};
use ini::Ini;
use inquire::Select;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use std::path::PathBuf;
use tokio::fs;

use crate::{api, Profile};

pub async fn get_sqlite_path() -> Result<PathBuf> {
    let mut config_path = dirs::config_dir().with_context(|| "No config dir found")?;
    config_path.push("gus");
    if !config_path.exists() {
        fs::create_dir(&config_path)
            .await
            .with_context(|| format!("Error creating Dir {:?}", config_path))?;
    }
    config_path.push("sqlite");
    config_path.set_extension("db");
    Ok(config_path)
}

pub async fn get_db_pool(path: &str) -> Result<Pool<Sqlite>> {
    let path = format!("sqlite://{}", path);
    if !Sqlite::database_exists(&path).await.unwrap_or(false) {
        Sqlite::create_database(&path)
            .await
            .with_context(|| format!("Error creating database {}", path))?;
    }

    let instance = SqlitePool::connect(&path)
        .await
        .with_context(|| format!("Error connecting sqlite db {}", path))?;
    sqlx::migrate!()
        .run(&instance)
        .await
        .with_context(|| "Error running migrations")?;
    Ok(instance)
}

pub async fn update_or_create_gitconfig(curr_path: &str, name: &str, email: &str) -> Result<()> {
    let mut curr_path = PathBuf::from(curr_path);
    curr_path.push(".git");
    if !curr_path.exists() {
        bail!(".git Path does not exists. Try to run this from root of git repository.");
    }
    curr_path.push("config");
    let mut config = {
        if curr_path.exists() {
            Ini::load_from_file(curr_path.clone())?
        } else {
            println!(
                "config file not exists. Creating new file at {:?}",
                curr_path
            );
            Ini::new()
        }
    };

    config
        .with_section(Some("user"))
        .set("name", name)
        .set("email", email);

    config.write_to_file(curr_path)?;

    Ok(())
}

pub async fn show_list(db: &SqlitePool) -> Result<Profile> {
    let profiles = api::list_all_profiles(db).await?;
    let ans = Select::new("Select any one profile", profiles).prompt()?;
    Ok(ans)
}
