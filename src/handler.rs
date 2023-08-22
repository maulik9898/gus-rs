use crate::api;
use crate::utils;
use crate::Commands;
use crate::GusCli;
use crate::Profile;
use color_eyre::eyre::Context;
use color_eyre::Result;
use inquire::{Confirm, Text};
use sqlx::{Pool, Sqlite};
use std::io;
use std::io::Write;

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
            Commands::A(profile) => self.add(profile).await?,
            Commands::Ac(args) => self.activate(&args.profile).await?,
            Commands::Add => self.add_interactively().await?,
            Commands::Delete => self.delete().await?,
            Commands::Edit => self.edit().await?,
        }
        Ok(())
    }

    pub async fn add_interactively(&self) -> Result<()> {
        loop {
            let name = Text::new("Enter user name ").prompt()?;

            let email = Text::new("Enter email").prompt()?;

            let profile = Text::new("Enter profile name").prompt()?;

            let profile = Profile {
                name,
                email,
                profile,
            };

            api::add_profile(&self.db, &profile).await?;
            println!("Added {} profile", profile);

            let ans = Confirm::new("Do you want to enter new profile")
                .with_default(false)
                .prompt()?;
            utils::clear_last_propmt(5)?;
            if !ans {
                break;
            }
        }
        Ok(())
    }

    pub async fn list(&self) -> Result<()> {
        let ans = utils::show_list(&self.db).await?;
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
        let ans = utils::show_list_multiple(&self.db).await?;
        api::delete_profiles(&self.db, &ans).await?;
        for profile in ans {
            println!("Deleted {} profile", { profile })
        }
        Ok(())
    }

    pub async fn edit(&self) -> Result<()> {
        let ans = utils::show_list(&self.db).await?;
        let name = Text::new("Enter user name ")
            .with_default(&ans.name)
            .prompt()?;

        let email = Text::new("Enter email").with_default(&ans.email).prompt()?;
        let profile = Profile {
            profile: ans.profile,
            name,
            email,
        };
        api::update_profile(&self.db, &profile).await?;
        println!("Updated {} profile", profile);
        Ok(())
    }
}
