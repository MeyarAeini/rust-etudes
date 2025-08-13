pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::{collections::HashMap, env};

use crate::{
    models::{NewUser, User, Vote},
    schema::votes::{self, option_id, ordinal, user_id},
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

pub fn run_election() -> Vec<ElectionResult> {
    use itertools::Itertools;
    use rcir::{ElectionResult, MajorityMode, run_election};

    let mut conn = establish_connection();

    let mut all_votes = match votes::dsl::votes.select(Vote::as_select()).load(&mut conn) {
        Ok(value) => value,
        Err(_) => return Vec::new(), //TODO: return a proper error, for now consider error as no
                                     //election
    };

    let mut winners = HashMap::new();

    all_votes.sort_by_key(|vote| (vote.user_id, vote.ordinal));
    let mut rank = 0;
    loop {
        rank += 1;
        let ballots: Vec<Vec<i32>> = all_votes
            .clone()
            .into_iter()
            .chunk_by(|vote| vote.user_id)
            .into_iter()
            .map(|(_, user_votes)| {
                let mut user_votes: Vec<_> = user_votes.collect();
                user_votes.sort_by_key(|k| k.ordinal);

                user_votes
                    .into_iter()
                    .map(|v| v.option_id.clone())
                    .collect()
            })
            .collect();

        match run_election(&ballots, MajorityMode::CompleteMajority) {
            Ok(result) => match result {
                ElectionResult::Winner(winner) => {
                    winners.entry(winner.clone()).or_insert(rank);
                    all_votes.retain(|v| &v.option_id != winner);
                }
                ElectionResult::Tie(winners_together) => {
                    for winner in winners_together {
                        winners.entry(winner.clone()).or_insert(rank);
                        all_votes.retain(|v| &v.option_id != winner);
                    }
                }
            },
            Err(_error) => break, //exit the loop, the election is either finished or the election
                                  //library can not decide how to proceed
        }
    }

    let mut options: std::collections::HashMap<_, _> = get_options(&mut conn)
        .into_iter()
        .map(|o| (o.id, o))
        .collect();

    winners
        .into_iter()
        .map(|(w, rank)| {
            let option = options.remove(&w).unwrap();

            crate::ElectionResult {
                name: option.name,
                description: option.description,
                rank,
            }
        })
        .collect()
}

use serde::Serialize;

#[derive(Serialize)]
pub struct ElectionResult {
    pub name: String,
    pub description: String,
    pub rank: i32,
}
