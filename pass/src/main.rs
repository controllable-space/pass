use std::env;
use anyhow::Context;
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection, AsyncPgConnection};
use controllable_pass::schema::passwords;
use controllable_pass::models::Password;
use controllable_pass::create_password;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let database_url = env::var("DATABASE_URL")
        .with_context(|| format!("please set DATABASE_URL environment variable"))?;
    let mut conn = AsyncPgConnection::establish(&database_url)
        .await
        .with_context(|| format!("Failed to connect to database. Please make sure DATABASE_URL is correct.") )?;

    let passwords: Vec<Password> = passwords::table
        .select(Password::as_select())
        .load(&mut conn)
        .await
        .with_context(|| format!("Failed to query for passwords"))?;

    if passwords.len() == 0 {
        create_password(&mut conn, "example", "example").await?;
    }

    dbg!(passwords);

    Ok(())
}
