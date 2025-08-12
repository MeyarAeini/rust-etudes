pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::{
    models::{NewUser, User, Vote},
    schema::votes::{option_id, ordinal, user_id},
};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("SQLITE_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Failed to connect, error: {e}"))
}

pub fn create_user(conn: &mut SqliteConnection, name: &str) -> User {
    if let Some(user) = get_user(conn, name) {
        return user;
    }

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

pub fn get_user(conn: &mut SqliteConnection, username: &str) -> Option<User> {
    use crate::schema::users::dsl::*;

    if let Ok(result) = users
        .filter(crate::schema::users::dsl::name.eq(username))
        .select(crate::models::User::as_select())
        .first(conn)
    {
        Some(result)
    } else {
        None
    }
}

pub fn save_votes(conn: &mut SqliteConnection, username: &str, ordered_choises: Vec<i32>) {
    use crate::schema::votes;

    if let Some(user) = get_user(conn, username) {
        if let Ok(_) = diesel::delete(votes::table)
            .filter(user_id.eq(user.id))
            .execute(conn)
        {
            for (index, item) in ordered_choises.iter().enumerate() {
                diesel::insert_into(votes::table)
                    .values((
                        user_id.eq(user.id),
                        option_id.eq(item),
                        ordinal.eq(index as i32 + 1),
                    ))
                    .on_conflict((user_id, option_id))
                    .do_update()
                    .set(ordinal.eq(index as i32 + 1))
                    .execute(conn)
                    .expect("error on saving a new vote");
            }
        }
    }
}
