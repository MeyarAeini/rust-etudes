use axum::{
    Json, Router,
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use minijinja::{Environment, context};
use serde::Deserialize;
use std::{collections::HashSet, sync::Arc};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;
use voting::*;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening to {}", listener.local_addr().unwrap());

    let mut env = Environment::new();
    env.add_template("layout", include_str!("../../templates/layout.jinja"))
        .unwrap();
    env.add_template("home", include_str!("../../templates/home.jinja"))
        .unwrap();

    env.add_template("election", include_str!("../../templates/election.jinja"))
        .unwrap();

    let state = std::sync::Arc::new(AppState { env });

    let app = Router::new()
        .route("/", get(home).post(wants_to_vote))
        .route("/logout", post(logout))
        .route("/election", get(election))
        .route("/submit-votes", post(submit_votes))
        .layer(CookieManagerLayer::new())
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}

struct AppState {
    env: Environment<'static>,
}

async fn home(
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let html = state.env.get_template("home").unwrap();

    let current_user = cookies
        .get("username")
        .map(|cookie| cookie.value().to_owned());
    let mut has_user = true;
    let username = current_user.unwrap_or_else(|| {
        has_user = false;
        String::new()
    });

    let mut conn = establish_connection();

    let mut options = get_options(&mut conn);
    let user_options = get_user_options(&mut conn, &username);
    let set :HashSet<i32>= HashSet::from_iter(user_options.iter().map(|o| o.id));
    options.retain(|o| !set.contains(&o.id));
    let rendered = html
        .render(context! {
        current_user => has_user,
        title=>"home",
        welcome_text=>format!("hello {}",username),
        options=>options,
        votes=>user_options,
            })
        .unwrap();

    Ok(Html(rendered))
}

async fn election(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let html = state.env.get_template("election").unwrap();
    let election_result = run_election();
    let rendered = html
        .render(context! {
        title=>"election result",
        election_result=>election_result
            })
        .unwrap();

    Ok(Html(rendered))
}

#[derive(Deserialize, Debug)]
struct User {
    name: String,
}
async fn wants_to_vote(cookies: Cookies, Form(user): Form<User>) -> impl IntoResponse {
    if !user.name.is_empty() {
        let mut conn = establish_connection();
        create_user(&mut conn, &user.name);
        cookies.add(Cookie::new("username", user.name));
    }

    Redirect::to("/")
}

async fn logout(cookies: Cookies) -> impl IntoResponse {
    cookies.remove(Cookie::new("username", ""));

    Redirect::to("/")
}

#[derive(Deserialize, Debug)]
struct UserVote {
    id: i32,
    order: i32,
}

async fn submit_votes(cookies: Cookies, Json(votes): Json<Vec<UserVote>>) {
    let current_user = cookies
        .get("username")
        .map(|cookie| cookie.value().to_owned())
        .unwrap_or(String::new());
    let mut conn = establish_connection();
    let mut votes = votes;
    votes.sort_by_key(|v| v.order);
    let ordered_votes = votes.iter().map(|v| v.id).collect();
    save_votes(&mut conn, &current_user, ordered_votes);
}
