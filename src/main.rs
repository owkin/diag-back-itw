pub mod app;
pub mod config;
pub mod error;
pub mod image_proc;
use clap::{arg, Command};

#[tokio::main]
async fn main() {
    let matches = Command::new("Backend")
        .arg(arg!(--port <VALUE>).required(true))
        .get_matches();

    let app = app::router::app();

    // run it with hyper
    let port = matches.get_one::<String>("port").expect("required");
    let addr = format!("{}:{}", "0.0.0.0", port.to_string());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
