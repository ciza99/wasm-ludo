use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use components::game_server::actor::GameServer;
use dotenv::dotenv;
use env_logger::Env;
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

mod components;
mod models;
mod types;
mod utils;

use models::app_data::AppData;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  env_logger::init_from_env(Env::default().default_filter_or("info"));
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable is not set");
  let port = env::var("PORT").unwrap_or("8080".to_string());
  
  let mut client_options = ClientOptions::parse(database_url).await?;
  client_options.app_name = Some("Ludo".to_string());

  let client = Client::with_options(client_options)?;
  let db = Arc::new(Mutex::new(client.database("main")));

  let game_server_addr = GameServer::new(db.clone()).start();

  let app_data = web::Data::new(AppData {
    game_server_addr,
    db: db.clone(),
  });

  HttpServer::new(move || {
    App::new()
      .wrap(
        Cors::default()
          .allow_any_header()
          .allow_any_origin()
          .allow_any_method(),
      )
      .app_data(app_data.clone())
      .wrap(middleware::Logger::default())
      .configure(components::game::routes::attach_routes)
  })
  .bind(format!("0.0.0.0:{}",port))?
  .run()
  .await?;

  Ok(())
}
