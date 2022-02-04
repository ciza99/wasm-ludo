use super::{super::actor::GameServerState, move_bot::move_bot};
use crate::{
  components::{
    game::database,
    game_server::utils::{send_message, send_message_to_room},
  },
  models::actor_messages::ClientActorMessage,
  utils::{dice::get_dice_value, enums::{ServerMessage, RoundPhase}, player::get_available_positions},
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
  let current_player_id = game
    .players
    .iter()
    .find(|player| player.color == game.current_player)
    .unwrap()
    .id
    .clone(); //TODO probably shouldn't unwrap - game.get_player() uses unwrap() as well
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
  let can_roll_again = roll == 6 && game.dice_throws.iter().len() != 3;
  let roll_message =
    serde_json::to_string(&ServerMessage::DiceValue(roll, can_roll_again)).unwrap();
  send_message_to_room(
    roll_message.as_str(),
    state.sessions.clone(),
    state.rooms.clone(),
    &msg.room_id,
  );
  let rolls_sum: usize = game.dice_throws.clone().iter().sum();

  if rolls_sum == 18 || (rolls_sum < 6 && game.get_current_player().pawns_at_start + game.get_current_player().pawns_at_finish ==4) {
    game.update_current_player();
    game.dice_throws.clear();
    let mut game_state = database::update_game_state(&state.db, &msg.room_id, &game)
      .await
      .unwrap(); //TODO handle errors

    let skip_message = serde_json::to_string(&ServerMessage::SkipPlayer).unwrap();
    send_message_to_room(
      skip_message.as_str(),
      state.sessions.clone(),
      state.rooms.clone(),
      &msg.room_id,
    );

    let update_message = serde_json::to_string(&ServerMessage::GameUpdate(game_state.clone())).unwrap();
    send_message_to_room(
      update_message.as_str(),
      state.sessions.clone(),
      state.rooms.clone(),
      &msg.room_id,
    );
    move_bot(state.clone(), &msg, &mut game_state).await;
  } else if !can_roll_again {
    println!("Sum: {}", rolls_sum);
    let possible_moves = get_available_positions(&game, rolls_sum);
    let roll_results_message = serde_json::to_string(&ServerMessage::AvailablePositions(
      possible_moves.0,
      possible_moves.1,
      possible_moves.2,
    ))
    .unwrap();//TODO handle no available moves
    game.round_phase = RoundPhase::Moving;
    let _ =database::update_game_state(&state.db, &msg.room_id, &game).await;
    send_message_to_room(
      roll_results_message.as_str(),
      state.sessions.clone(),
      state.rooms.clone(),
      &msg.room_id,
    );
  }
}
