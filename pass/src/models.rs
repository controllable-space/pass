use diesel::prelude::*;
use crate::schema::passwords;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = passwords)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Password {
    pub id: i32,
    pub name: String,
    pub value: String,
}

#[derive(Insertable)]
#[diesel(table_name = passwords)]
pub struct NewPassword<'a> {
    pub name: &'a str,
    pub value: &'a str,
}
