use super::super::actor::GameServerState;
use super::move_bot::move_bot;
use crate::components::game_server::services::utils::{send_roll_message, skip_player};
use crate::{
  components::{game::database, game_server::utils::send_message},
  models::actor_messages::ClientActorMessage,
  utils::{
    dice::get_dice_value,
    enums::{RoundPhase, ServerMessage},
    player::get_available_positions,
  },
};

pub async fn roll_dice(state: GameServerState, msg: ClientActorMessage) {
  let roll = get_dice_value();
  let db_game = database::find_game(&state.db, &msg.room_id).await;
  let game = match db_game {
    Ok(Some(game)) => game,
    _ => {
      let message =
        serde_json::to_string(&ServerMessage::Error("Cannot find game".into())).unwrap();
      send_message(message.as_str(), state.sessions, &msg.player_id);
      return;
    }
  };
  if game.round_phase != RoundPhase::Rolling {
    let message =
      serde_json::to_string(&ServerMessage::Error("Rolling is not allowed now".into())).unwrap();
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

  let res = database::add_dice_roll(&state.db, &msg.room_id, roll).await;

  if res.is_err() {
    let message =
      serde_json::to_string(&ServerMessage::Error("Error while rolling dice".into())).unwrap();
    send_message(message.as_str(), state.sessions, &msg.player_id);
    return;
  };
  let mut game = res.unwrap();
  let can_roll_again = roll == 6 && game.dice_throws.len() < 3;
  send_roll_message(state.clone(), &msg, roll, can_roll_again).await;

  // just informed players about roll
  if can_roll_again {
    return;
  }

  let rolls_sum: usize = game.dice_throws.clone().iter().sum();
  if rolls_sum == 18 {
    let _ = skip_player(state.clone(), &msg, &mut game).await;
    move_bot(state.clone(), &msg, &mut game).await;
    return;
  }

  // shouldn't be necessary, since there should be no available positions anyway,
  // but is faster since it doesn't need to compute the available positions
  // can be OR'd with 'rolls_sum == 18' condition
  if rolls_sum < 6
    && game.get_current_player().pawns_at_start + game.get_current_player().pawns_at_finish == 4
  {
    let _ = skip_player(state.clone(), &msg, &mut game).await;
    move_bot(state.clone(), &msg, &mut game).await;
    return;
  }

  let available_positions = get_available_positions(&game, rolls_sum);
  if no_available_positions(&available_positions) {
    let _ = skip_player(state.clone(), &msg, &mut game).await;
    move_bot(state.clone(), &msg, &mut game).await;
  } else {
    // send available positions to player (he should choose one of the positions / promote) and update round_phase
    let roll_results_message = serde_json::to_string(&ServerMessage::AvailablePositions(
      available_positions.0,
      available_positions.1,
      available_positions.2,
    ))
    .unwrap();

    game.round_phase = RoundPhase::Moving;
    let _ = database::update_game_state(&state.db, &msg.room_id, &game).await;

    send_message(
      roll_results_message.as_str(),
      state.sessions.clone(),
      &msg.player_id,
    );
  }
}

fn no_available_positions(available_positions: &(Vec<usize>, Vec<usize>, bool)) -> bool {
  let (positions_in_fields, positions_in_home, can_promote) = available_positions;
  positions_in_fields.is_empty() && positions_in_home.is_empty() && !(*can_promote)
}
