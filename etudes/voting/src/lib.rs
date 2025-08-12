pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::{NewUser, User};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("SQLITE_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Failed to connect, error: {e}"))
}

pub fn create_user(conn: &mut SqliteConnection, name: &str) -> User {
    let new_user = NewUser { name };

    use crate::schema::users;

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("save new user")
}

pub fn get_options(conn: &mut SqliteConnection) -> Vec<crate::models::Option> {
    use crate::schema::options::dsl::*;

    options
        .select(crate::models::Option::as_select())
        .load(conn)
        .expect("error loading options")
}
