use anyhow::Context;
use diesel::prelude::*;
use diesel_async::{RunQueryDsl, AsyncPgConnection};
use crate::models::{NewPassword, Password};

pub async fn create_password(conn: &mut AsyncPgConnection, new_password: NewPassword) -> anyhow::Result<Password> {
    use crate::schema::passwords;

    let result = diesel::insert_into(passwords::table)
        .values(&new_password)
        .returning(Password::as_returning())
        .get_result(conn)
        .await
        .with_context(|| format!("Failed to create new password"))?;

    Ok(result)
}

pub async fn list_passwords(conn: &mut AsyncPgConnection) -> anyhow::Result<Vec<Password>> {
    use crate::schema::passwords;

    let passwords: Vec<Password> = passwords::table
        .select(Password::as_select())
        .load(conn)
        .await
        .with_context(|| format!("Failed to list passwords"))?;

    Ok(passwords)
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
            let new_password = NewPassword {
                name: "name".to_string(),
                value: "value".to_string()
            };
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

    #[tokio::test]
    async fn test_list_passwords() -> anyhow::Result<()> {
        use super::list_passwords;
        use crate::models::NewPassword;

        let mut conn = database_connection().await?;
        conn.test_transaction::<_, anyhow::Error, _>(|conn| async move {
            let new_passwords = vec![
                NewPassword {
                    name: "name1".to_string(),
                    value: "value1".to_string()
                },
                NewPassword {
                    name: "name2".to_string(),
                    value: "value2".to_string()
                }
            ];

            let expected_passwords = diesel::insert_into(passwords::table)
                .values(new_passwords)
                .returning(Password::as_returning())
                .get_results(conn)
                .await?;
            let actual_passwords: Vec<Password> = list_passwords(conn).await?;

            assert_eq!(expected_passwords, actual_passwords);

            Ok(())
        }.scope_boxed()).await;

        Ok(())
    }
}
