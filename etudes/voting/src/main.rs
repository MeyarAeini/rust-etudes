use axum::{
    Router,
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use minijinja::{Environment, context};
use serde::Deserialize;
use std::sync::Arc;
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening to {}", listener.local_addr().unwrap());

    let mut env = Environment::new();
    env.add_template("layout", include_str!("../templates/layout.jinja"))
        .unwrap();
    env.add_template("home", include_str!("../templates/home.jinja"))
        .unwrap();

    let state = std::sync::Arc::new(AppState(env));

    let app = Router::new()
        .route("/", get(home).post(wants_to_vote))
        .route("/logout", post(logout))
        .layer(CookieManagerLayer::new())
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}

struct AppState(Environment<'static>);

async fn home(
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let html = state.0.get_template("home").unwrap();

    let current_user = cookies
        .get("username")
        .map(|cookie| cookie.value().to_owned());

    let options = vec!["name1", "name2", "name3", "name4", "name5"];

    let rendered = html
        .render(context! {
        current_user => current_user.is_some(),
        title=>"home",
        welcome_text=>format!("hello {}",current_user.unwrap_or(String::new())),
        options=>options
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
        cookies.add(Cookie::new("username", user.name));
    }

    Redirect::to("/")
}

async fn logout(cookies: Cookies) -> impl IntoResponse {
    cookies.remove(Cookie::new("username", ""));

    Redirect::to("/")
}
