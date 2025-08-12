use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::options)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Option {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name=crate::schema::votes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Vote {
    pub user_id: i32,
    pub option_id: i32,
    pub ordinal: i32,
}

#[derive(Insertable)]
#[diesel(table_name=crate::schema::users)]
pub struct NewUser<'a> {
    pub name: &'a str,
}
