use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::passwords;

#[derive(Queryable, Selectable, Debug, PartialEq, Serialize)]
#[diesel(table_name = passwords)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Password {
    pub id: i32,
    pub name: String,
    pub value: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = passwords)]
pub struct NewPassword {
    pub name: String,
    pub value: String,
}
