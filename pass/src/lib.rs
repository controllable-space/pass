pub mod models;
pub mod schema;

use anyhow::Context;
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncPgConnection};
use self::models::{NewPassword, Password};

pub async fn create_password(conn: &mut AsyncPgConnection, name: &str, value: &str) -> anyhow::Result<Password> {
    use crate::schema::passwords;

    let new_password = NewPassword { name, value };
    let result = diesel::insert_into(passwords::table)
        .values(&new_password)
        .returning(Password::as_returning())
        .get_result(conn)
        .await
        .with_context(|| format!("Failed to create new password"))?;

    Ok(result)
}
