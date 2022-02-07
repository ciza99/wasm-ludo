use anyhow::anyhow;
use mongodb::{
  bson::{self, doc, oid::ObjectId, Bson, Document},
  options::{FindOneAndUpdateOptions, ReturnDocument},
  Database,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::{game::Game, player::Player};

pub async fn create_game(db: &Arc<Mutex<Database>>) -> anyhow::Result<String> {
  let db_mutex = db.lock().await;
  let game_collection = db_mutex.collection::<Game>("games");
  let mock_game = Game::new();
  let res = game_collection.insert_one(mock_game, None).await;
  match res {
    Ok(result) => {
      if let Bson::ObjectId(id) = result.inserted_id {
        return Ok(format!("{}", id));
      } else {
        return Err(anyhow!("game id couldn't be parsed"));
      };
    }
    Err(e) => Err(anyhow!(e)),
  }
}

pub async fn add_player(
  db: &Arc<Mutex<Database>>,
  game_id: &str,
  new_player: Player,
) -> anyhow::Result<Game> {
  let serialized_player = bson::to_bson(&new_player)?;
  let update = doc! { "$push": { "players": serialized_player } };
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(anyhow!(err)),
  };
  let filter = doc! { "_id" : oid };
  return update_game(db, filter, update).await;
}

pub async fn find_game(db: &Arc<Mutex<Database>>, game_id: &str) -> anyhow::Result<Option<Game>> {
  let db_mutex = db.lock().await;
  let game_collection = db_mutex.collection::<Game>("games");
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(anyhow!(err)),
  };
  let filter = doc! { "_id" : oid };
  let found = game_collection.find_one(filter, None).await;
  match found {
    Ok(result) => Ok(result),
    Err(e) => Err(anyhow!(e)),
  }
}

pub async fn start_game(db: &Arc<Mutex<Database>>, game_id: &str) -> anyhow::Result<Game> {
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(anyhow!(err)),
  };
  let filter = doc! { "_id" : oid };
  let update = doc! { "$set": { "started" : true } };
  return update_game(db, filter, update).await;
}

pub async fn add_dice_roll(
  db: &Arc<Mutex<Database>>,
  game_id: &str,
  roll: usize,
) -> anyhow::Result<Game> {
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(anyhow!(err)),
  };
  let serialized_roll = bson::to_bson(&roll)?;
  let filter = doc! { "_id" : oid };
  let update = doc! { "$push": { "dice_throws": serialized_roll } };
  return update_game(db, filter, update).await;
}

async fn update_game(
  db: &Arc<Mutex<Database>>,
  filter: Document,
  update: Document,
) -> anyhow::Result<Game> {
  let db_mutex = db.lock().await;
  let game_collection = db_mutex.collection::<Game>("games");
  let option = FindOneAndUpdateOptions::builder()
    .return_document(ReturnDocument::After)
    .build();
  let res = game_collection
    .find_one_and_update(filter, update, option)
    .await;
  match res {
    Ok(Some(game)) => Ok(game),
    Ok(None) => Err(anyhow!("Game doesnt exits")),
    Err(e) => Err(anyhow!(e)),
  }
}

pub async fn update(
  db: &Arc<Mutex<Database>>,
  game_id: &str,
  update: Document,
) -> anyhow::Result<Game> {
  let oid = match ObjectId::parse_str(game_id) {
    Ok(res) => res,
    Err(err) => return Err(anyhow!(err)),
  };
  let filter = doc! { "_id" : oid };
  let res = update_game(db, filter, update).await;
  match res {
    Ok(game) => Ok(game),
    Err(e) => Err(anyhow!(e)),
  }
}

pub async fn update_game_state(
  db: &Arc<Mutex<Database>>,
  game_id: &str,
  game: &Game,
) -> anyhow::Result<Game> {
  let update_doc = match make_doc(game) {
    Ok(doc) => doc,
    _ => return Err(anyhow!("Failed to create document")),
  };
  update(db, game_id, update_doc).await
}

fn make_doc(game: &Game) -> anyhow::Result<Document> {
  let fields = bson::to_bson(&game.fields)?;
  let players = bson::to_bson(&game.players)?;
  let current_player = bson::to_bson(&game.current_player)?;
  let bson_dice_throws = bson::to_bson(&game.dice_throws)?;
  let phase = bson::to_bson(&game.round_phase)?;
  let doc = doc! { "$set": { "fields": fields, "players": players, "current_player": current_player, "dice_throws": &bson_dice_throws, "round_phase":phase } };
  Ok(doc)
}
