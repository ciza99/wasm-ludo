use actix_web::{get, post, put, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use futures::stream::TryStreamExt;
use serde::Deserialize;
use uuid::Uuid;

use crate::models::{app_data::AppData, color::Color, game::Game, player::Player};

use super::super::session::actor::GameSession;
use super::database;

#[post("")]
pub async fn create_new_game(data: web::Data<AppData>) -> HttpResponse {
  let game_res = database::create_game(&data.db).await;
  match game_res {
    Ok(id) => HttpResponse::Ok().body(id),
    Err(_) => HttpResponse::InternalServerError().body("Failed to create new game"),
  }
}

#[derive(Deserialize)]
pub struct JoinGameBody {
  pub name: String,
}

#[derive(Deserialize)]

pub struct JoinGamePath {
  pub room: String,
}

#[put("/{room}")]
pub async fn join_game(
  body: web::Json<JoinGameBody>,
  path: web::Path<JoinGamePath>,
  data: web::Data<AppData>,
) -> HttpResponse {
  let room = path.room.as_str();
  let game_res = database::find_game(&data.db, room).await;
  let game = match game_res {
    Ok(Some(game)) => game,
    _ => return HttpResponse::InternalServerError().body("Failed to join game"),
  };
  let used_colors = game
    .players
    .iter()
    .map(|player| player.color)
    .collect::<Vec<_>>();
  let free_colors = Color::ordered()
    .into_iter()
    .filter(|color| !used_colors.contains(color))
    .collect::<Vec<_>>();

  let color = match free_colors.first() {
    Some(color) => color,
    None => return HttpResponse::Conflict().body("Game is full"),
  };

  let player_id = Uuid::new_v4().to_string();

  let new_player = Player::new(player_id.clone(), body.name.clone(), *color, false);
  let res = database::add_player(&data.db, room, new_player).await;

  if res.is_err() {
    return HttpResponse::InternalServerError().body("Couldn't add you as a player");
  }

  HttpResponse::Ok().body(player_id)
}

#[derive(Deserialize)]
pub struct WebsocketPath {
  pub room: String,
  pub player_id: String,
}

#[get("/websocket/{room}/{player_id}")]
pub async fn init_websocket(
  req: HttpRequest,
  stream: web::Payload,
  path: web::Path<WebsocketPath>,
  data: web::Data<AppData>,
) -> HttpResponse {
  let game_id = &path.room;
  let player_id = &path.player_id;

  let result = database::find_game(&data.db, game_id).await;

  let game = match result {
    Ok(Some(game)) => game,
    _ => return HttpResponse::Forbidden().body("Game does not exist"),
  };

  if !game.players.iter().any(|player| &player.id == player_id) {
    return HttpResponse::Forbidden().body("You haven't joined this game");
  };

  let session = GameSession::new(
    player_id.clone(),
    game_id.clone(),
    data.game_server_addr.clone(),
  );
  let resp = ws::start(session, &req, stream);
  println!("{:?}", resp);
  resp.unwrap_or_else(|_| HttpResponse::InternalServerError().body("Whoops"))
}

#[get("")]
pub async fn get_games(data: web::Data<AppData>) -> HttpResponse {
  let db = &data.db.lock().await;
  let game_collection = db.collection::<Game>("games");

  let mut cursor = game_collection.find(None, None).await.unwrap();

  let mut games: Vec<Game> = Vec::new();
  while let Ok(Some(game)) = cursor.try_next().await {
    games.push(game);
  }

  HttpResponse::Ok().json(games)
}
