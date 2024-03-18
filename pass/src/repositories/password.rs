use anyhow::Context;
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncPgConnection};
use crate::models::{NewPassword, Password};

pub async fn create_password<'a>(conn: &mut AsyncPgConnection, new_password: NewPassword<'a>) -> anyhow::Result<Password> {
    use crate::schema::passwords;

    let result = diesel::insert_into(passwords::table)
        .values(&new_password)
        .returning(Password::as_returning())
        .get_result(conn)
        .await
        .with_context(|| format!("Failed to create new password"))?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use diesel::prelude::*;
    use diesel_async::{RunQueryDsl, AsyncConnection};
    use scoped_futures::ScopedFutureExt;
    use crate::schema::passwords;
    use crate::models::Password;
    use crate::tests::database_connection;

    #[tokio::test]
    async fn test_create_password() -> anyhow::Result<()> {
        use super::create_password;
        use crate::models::NewPassword;

        let mut conn = database_connection().await?;
        conn.test_transaction::<_, anyhow::Error, _>(|conn| async move {
            let new_password = NewPassword { name: "name", value: "value" };
            let created_password = create_password(conn, new_password).await?;
            let all_passwords: Vec<Password> = passwords::table
                .select(Password::as_select())
                .load(conn)
                .await?;

            assert_eq!(vec![created_password], all_passwords);

            Ok(())
        }.scope_boxed()).await;

        Ok(())
    }
}
