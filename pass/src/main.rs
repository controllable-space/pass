use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncConnection, AsyncPgConnection};
use std::env;
use controllable_pass::schema::passwords;
use controllable_pass::models::Password;
use controllable_pass::create_password;

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = AsyncPgConnection::establish(&database_url).await.expect("Failed to connect");

    let passwords: Vec<Password> = passwords::table
        .select(Password::as_select())
        .load(&mut conn)
        .await
        .expect("Failed to query for passwords");

    if passwords.len() == 0 {
        create_password(&mut conn, "example", "example").await;
    }

    dbg!(passwords);
}
