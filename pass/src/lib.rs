pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncPgConnection};
use self::models::{NewPassword, Password};

pub async fn create_password(conn: &mut AsyncPgConnection, name: &str, value: &str) -> Password {
    use crate::schema::passwords;

    let new_password = NewPassword { name, value };

    diesel::insert_into(passwords::table)
        .values(&new_password)
        .returning(Password::as_returning())
        .get_result(conn)
        .await
        .expect("Error saving new password")
}
