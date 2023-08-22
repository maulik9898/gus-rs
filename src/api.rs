use color_eyre::Result;
use sqlx::{Pool, Sqlite, SqlitePool};

use crate::Profile;

pub async fn list_all_profiles(db: &Pool<Sqlite>) -> Result<Vec<Profile>> {
    let profiles = sqlx::query_as!(Profile, "Select * from profiles ")
        .fetch_all(db)
        .await?;
    Ok(profiles)
}

pub async fn add_profile(db: &SqlitePool, profile: &Profile) -> Result<()> {
    let _profile = sqlx::query!(
        r#"INSERT INTO profiles (profile, name, email) VALUES (?1,?2, ?3) "#,
        profile.profile,
        profile.name,
        profile.email
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn get_profile(db: &SqlitePool, profile: &str) -> Result<Profile> {
    let profile = sqlx::query_as!(
        Profile,
        r#"SELECT * FROM profiles WHERE profile = ?1"#,
        profile
    )
    .fetch_one(db)
    .await?;
    Ok(profile)
}

pub async fn delete_profile(db: &SqlitePool, profile_name: &str) -> Result<()> {
    sqlx::query!(r#"DELETE FROM profiles WHERE profile = ?1"#, profile_name)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn delete_profiles(db: &SqlitePool, profiles: &Vec<Profile>) -> Result<()> {
    for profile in profiles {
        sqlx::query!(
            r#"DELETE FROM profiles WHERE profile = ?1"#,
            profile.profile
        )
        .execute(db)
        .await?;
    }
    Ok(())
}

pub async fn update_profile(db: &SqlitePool, profile: &Profile) -> Result<()> {
    sqlx::query!(
        "UPDATE profiles SET name = ?1 , email = ?2 WHERE profile = ?3",
        profile.name,
        profile.email,
        profile.profile
    )
    .execute(db)
    .await?;
    Ok(())
}
