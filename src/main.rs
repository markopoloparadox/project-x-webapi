use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;

mod database;
mod server;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let db = database::Database::new();
    let result = db.query("GET ALL FROM test WHERE family = Ana");
    println!("{:?}", result);

    /*
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap(); */
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
