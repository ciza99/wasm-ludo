use actix::{
  fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
  Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use std::time::Instant;

use crate::components::game_server::actor::GameServer;
use crate::models::actor_messages::{ClientActorMessage, Connect, Disconnect, WsMessage};

/// Game session actor (for each connected client)
/// Sends messages to the GameServer actor who coordinates all connected sessions
pub struct GameSession {
  id: String,
  room: String,
  game_server: Addr<GameServer>,
  heartbeat: Instant,
}

impl GameSession {
  pub fn new(id: String, room: String, game_server: Addr<GameServer>) -> Self {
    println!("created game session");
    GameSession {
      id,
      room,
      heartbeat: Instant::now(),
      game_server,
    }
  }
}

// implementing lifecycle methods for a session
impl Actor for GameSession {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    println!("session started");

    let address = ctx.address();
    self
      .game_server
      .send(Connect {
        address: address.recipient(),
        player_id: self.id.clone(),
        room_id: self.room.clone(),
      })
      .into_actor(self)
      .then(|res, _, ctx| {
        if res.is_err() {
          ctx.stop();
        }
        fut::ready(())
      })
      .wait(ctx);
  }

  fn stopping(&mut self, _: &mut Self::Context) -> Running {
    println!("stoppping");
    self.game_server.do_send(Disconnect {
      room_id: self.room.clone(),
      player_id: self.id.clone(),
    });
    Running::Stop
  }
}

/// Handler for messages coming from the client
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSession {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => {
        self.heartbeat = Instant::now();
        ctx.pong(&msg);
      }
      // we recieved a pong message, client is still active so we can reset the heartbeat
      Ok(ws::Message::Pong(_msg)) => {
        self.heartbeat = Instant::now();
      }
      Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
      Ok(ws::Message::Close(reason)) => {
        ctx.close(reason);
        ctx.stop();
      }
      Ok(ws::Message::Continuation(_)) => {
        // message is too large so its sent in continuation
        // we are not handling large data
        ctx.stop();
      }
      Ok(ws::Message::Nop) => {}
      Ok(ws::Message::Text(s)) => self.game_server.do_send(ClientActorMessage {
        player_id: self.id.clone(),
        content: s.to_string(),
        room_id: self.room.clone(),
      }),
      Err(e) => panic!("{}", e),
    }
  }
}

// Sending a message back to the client from the session actor
impl Handler<WsMessage> for GameSession {
  type Result = ();

  fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
    // sending text to the client
    ctx.text(msg.0);
  }
}
