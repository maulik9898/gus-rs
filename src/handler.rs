use crate::api;
use crate::utils;
use crate::Commands;
use crate::GusCli;
use crate::Profile;
use color_eyre::eyre::Context;
use color_eyre::Result;
use inquire::Select;
use sqlx::{Pool, Sqlite};

pub struct GusHandler {
    db: Pool<Sqlite>,
    cli: GusCli,
    curr_path: String,
}

impl GusHandler {
    pub fn new(db: Pool<Sqlite>, cli: GusCli, curr_path: &str) -> Self {
        Self {
            db,
            cli,
            curr_path: curr_path.to_string(),
        }
    }
    pub async fn handle_commands(&self) -> Result<()> {
        match &self.cli.command {
            Commands::List => self.list().await?,
            Commands::Add(profile) => self.add(profile).await?,
            Commands::Activate(args) => self.activate(&args.profile).await?,
            Commands::Delete => self.delete().await?,
        }
        Ok(())
    }

    pub async fn list(&self) -> Result<()> {
        let profiles = api::list_all_profiles(&self.db).await?;
        let ans = Select::new("Select any one profile", profiles).prompt()?;
        utils::update_or_create_gitconfig(&self.curr_path, &ans.name, &ans.email).await?;
        println!("Activated {} profile", ans);
        Ok(())
    }

    pub async fn add(&self, profile: &Profile) -> Result<()> {
        let _result = api::add_profile(&self.db, profile).await?;
        println!("Added {} profile", profile);
        Ok(())
    }

    pub async fn activate(&self, profile_name: &str) -> Result<()> {
        let profile = api::get_profile(&self.db, profile_name)
            .await
            .with_context(|| format!("No profile with name {} found", profile_name))?;
        utils::update_or_create_gitconfig(&self.curr_path, &profile.name, &profile.email).await?;
        println!("Activated {} profile", profile);
        Ok(())
    }

    pub async fn delete(&self) -> Result<()> {
        let profiles = api::list_all_profiles(&self.db).await?;
        let ans = Select::new("Select any one profile", profiles).prompt()?;
        api::delete_profile(&self.db, &ans.profile).await?;
        println!("Deleted {} profile", ans);
        Ok(())
    }
}
