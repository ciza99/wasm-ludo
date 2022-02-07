use crate::components::game_server::services::move_bot::move_bot;
use crate::utils::enums::RoundPhase;
use crate::{
  components::{
    game::database,
    game_server::{
      actor::GameServerState,
      utils::{send_message, send_message_to_room},
    },
  },
  models::{actor_messages::ClientActorMessage, position::Position},
  utils::{
    enums::{MoveResult, MoveType, ServerMessage},
    game::play_round,
  },
};

pub async fn move_piece(state: GameServerState, msg: ClientActorMessage, position: Position) {
  let db_game = database::find_game(&state.db, &msg.room_id).await;
  let mut game = match db_game {
    Ok(Some(game)) => game,
    _ => {
      let message =
        serde_json::to_string(&ServerMessage::Error("Cannot find game".into())).unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
      return;
    }
  };
  if game.round_phase != RoundPhase::Moving {
    let message = serde_json::to_string(&ServerMessage::Error(
      "Moving a piece is not allowed now".into(),
    ))
    .unwrap();
    send_message(message.as_str(), state.sessions, &msg.player_id);
    return;
  }
  let current_player_id = game.get_current_player_id();
  if current_player_id != msg.player_id {
    let message =
      serde_json::to_string(&ServerMessage::Error("It is not your turn".into())).unwrap();
    send_message(message.as_str(), state.sessions, &msg.player_id);
    return;
  };
  let result = play_round(&mut game, MoveType::Move(position)).await;
  match result {
    MoveResult::Success(_) => {
      let mut game_state = database::update_game_state(&state.db, &msg.room_id, &game)
        .await
        .unwrap();
      let update_message =
        serde_json::to_string(&ServerMessage::GameUpdate(game_state.clone())).unwrap();
      send_message_to_room(
        update_message.as_str(),
        state.sessions.clone(),
        state.rooms.clone(),
        &msg.room_id,
      );

      // handle if next player is a bot
      move_bot(state.clone(), &msg, &mut game_state).await;
    }
    MoveResult::Winner(color) => {
      game.finish_game(color);
      let game_state = database::update_game_state(&state.db, &msg.room_id, &game)
        .await
        .unwrap();
      let update_message = serde_json::to_string(&ServerMessage::GameUpdate(game_state)).unwrap();
      send_message_to_room(
        update_message.as_str(),
        state.sessions.clone(),
        state.rooms.clone(),
        &msg.room_id,
      );
    }
    _ => {
      let message =
        serde_json::to_string(&ServerMessage::Error("Error executing move".into())).unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
    }
  }
}
