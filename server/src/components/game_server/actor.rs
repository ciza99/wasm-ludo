use actix::prelude::{Actor, Context, Handler, Recipient};
use mongodb::Database;
use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};
use tokio::sync::Mutex;

use super::{
  services::{
    connect_client::connect_client, move_piece::move_piece, promote_piece::promote_piece,
    roll_die::roll_dice, start_game::start_game,
  },
  utils::send_message_to_room,
};
use crate::models::{
  actor_messages::{ClientActorMessage, Connect, Disconnect, WsMessage},
  position::Position,
};
use crate::utils::enums::ClientMessage;
use crate::utils::enums::ServerMessage;

type Session = Recipient<WsMessage>;

#[derive(Clone)]
pub struct GameServerState {
  pub db: Arc<Mutex<Database>>,
  pub sessions: HashMap<String, Session>,
  pub rooms: HashMap<String, HashSet<String>>,
}

/// GameServer actor which keeps track of all the sessions and game rooms (each game room has up to 4 sessions)
pub struct GameServer {
  db: Arc<Mutex<Database>>,
  sessions: HashMap<String, Session>, // player_id => Address to send messages
  rooms: HashMap<String, HashSet<String>>, // room_id / game_id => player_id
}

impl GameServer {
  pub fn new(db: Arc<Mutex<Database>>) -> Self {
    GameServer {
      db,
      sessions: HashMap::new(),
      rooms: HashMap::new(),
    }
  }

  pub fn get_state(&self) -> GameServerState {
    GameServerState {
      db: self.db.clone(),
      sessions: self.sessions.clone(),
      rooms: self.rooms.clone(),
    }
  }
}

// Make the game server an actor so it can receive and send messages to sessions
impl Actor for GameServer {
  type Context = Context<Self>;
}

// Connect a session to the GameServer
impl Handler<Connect> for GameServer {
  type Result = ();

  fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
    println!("connected player with id: {}", msg.player_id);
    self
      .sessions
      .insert(msg.player_id.clone(), msg.address.clone());
    self
      .rooms
      .entry(msg.room_id.clone())
      .or_insert_with(HashSet::new)
      .insert(msg.player_id.clone());

    let count = self.sessions.len();
    let server_msg = ServerMessage::PlayerCountChange(count);
    let json = serde_json::to_string(&server_msg).unwrap();

    send_message_to_room(
      json.as_str(),
      self.sessions.clone(),
      self.rooms.clone(),
      msg.room_id.as_str(),
    );

    let state = self.get_state();
    actix_web::rt::spawn(async move {
      println!(
        "responded with connect message to player: {}",
        msg.player_id
      );
      connect_client(state, &msg).await;
    });
  }
}

// Handler for session message to disconnect from the GameServer
impl Handler<Disconnect> for GameServer {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
    println!("Someone left the game");

    let mut rooms: Vec<String> = Vec::new();

    if self.sessions.remove(&msg.player_id).is_some() {
      for (game_id, sessions) in &mut self.rooms {
        if sessions.remove(&msg.player_id) {
          rooms.push(game_id.to_owned());
        }
      }
    }

    let server_msg = ServerMessage::PlayerCountChange(self.sessions.len());
    let json = serde_json::to_string(&server_msg).unwrap();

    for room in rooms {
      send_message_to_room(
        json.as_str(),
        self.sessions.clone(),
        self.rooms.clone(),
        room.as_str(),
      );
    }
  }
}

impl Handler<ClientActorMessage> for GameServer {
  type Result = ();

  fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) {
    let result = serde_json::from_str::<ClientMessage>(msg.content.as_str());

    let message = match result {
      Ok(message) => message,
      // TODO: handle errors [send back Error(String) message ??]
      Err(_) => return,
    };

    let state = self.get_state();
    actix_web::rt::spawn(async move {
      match message {
        ClientMessage::ThrowDice => roll_dice(state, msg).await,
        ClientMessage::MoveFigure(position, color) => {
          move_piece(
            state,
            msg,
            Position {
              position,
              is_home: color.is_some(),
            },
          )
          .await
        }
        ClientMessage::PromotePiece => promote_piece(state, msg).await,
        ClientMessage::StartGame => start_game(state, msg).await,
      };
    });
  }
}
