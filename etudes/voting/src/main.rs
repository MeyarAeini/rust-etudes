use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening to {}", listener.local_addr().unwrap());

    let app = Router::new().route("/",get(home));
    
    axum::serve(listener,app).await.unwrap();
}

async fn home() -> Html<&'static str>{
    Html("<p1>hello web</p1>")
}
