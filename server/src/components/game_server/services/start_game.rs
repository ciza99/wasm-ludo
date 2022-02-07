use super::super::actor::GameServerState;
use crate::{
  components::{
    game::database,
    game_server::utils::{send_message, send_message_to_room},
  },
  models::actor_messages::ClientActorMessage,
  utils::{enums::ServerMessage, game::fill_with_bots},
};

pub async fn start_game(state: GameServerState, msg: ClientActorMessage) {
  let start_res = database::start_game(&state.db, &msg.room_id).await;
  let mut game = match start_res {
    Ok(game) => game,
    Err(_) => {
      let message =
        serde_json::to_string(&ServerMessage::Error("Cannot start the game".into())).unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
      return;
    }
  };

  game.players = fill_with_bots(game.players);
  let update_res = database::update_game_state(&state.db, &msg.room_id, &game).await;

  let game = match update_res {
    Ok(game) => game,
    Err(_) => {
      let message =
        serde_json::to_string(&ServerMessage::Error("Cannot update the game".into())).unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
      return;
    }
  };

  let message = serde_json::to_string(&ServerMessage::GameStarted(game)).unwrap();

  send_message_to_room(message.as_str(), state.sessions, state.rooms, &msg.room_id);
}
