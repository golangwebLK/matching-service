use std::io::Read;
use axum::{
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::fs::File;
use log::{Level, log};

#[derive(Deserialize, Serialize, Debug)]
struct Server{
    host: String,
    port: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Config{
    server: Server,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut file = File::open("config.yaml").expect("文件config.yaml打开失败");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let config: Config = serde_yaml::from_str(&contents).expect("yaml序列化失败");
    let addr = format!("{}:{}",config.server.host,config.server.port);

    let app = Router::new()
        .route("/", get(matching_service::root))
        .route("/matching", post(matching_service::matching));

    log!(Level::Info, "server start: {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



