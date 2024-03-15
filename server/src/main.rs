use dotenv::dotenv;
use config::{Config, Environment};
use serde::{Serialize, Deserialize};
use actix_web::{App, HttpServer};

mod handlers;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub server_addr: String
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::builder()
        .add_source(Environment::default().separator("__"))
        .build()
        .unwrap();

    let config: ServerConfig = config.try_deserialize().unwrap();

    HttpServer::new(move || {
            App::new()
                .service(handlers::test)
        })
        .workers(12)
        .bind(config.server_addr)?
        .run()
        .await
}
