use crate::components::game_server::actor::GameServerState;
use crate::components::game_server::services::utils::{
  send_game_update_message, send_roll_message, skip_player,
};
use crate::models::actor_messages::ClientActorMessage;
use crate::models::game::Game;
use crate::utils::dice::get_dice_value;
use crate::utils::enums::MoveResult;
use tokio::time::{sleep, Duration};

fn move_result_update_game(game: &mut Game, move_result: MoveResult) {
  match move_result {
    MoveResult::Success(_) => {
      game.update_current_player();
      game.dice_throws.clear();
    }
    MoveResult::Winner(winner) => game.finish_game(winner),
    MoveResult::Error(msg) => {
      game.update_current_player();
      game.dice_throws.clear();
      println!("move_result_update_game - MoveResult::Error: {}", msg);
    }
  }
}

/// Bot move algorithm:
/// 0. throw dice, skip turn if 18, otherwise
/// 1. jump to finish (from main field), if possible, otherwise
/// 2. jump to home (from main field),
/// 3. add new piece if only 1 is in main field
/// 4. remove enemy's piece
/// 5. add new piece to game
/// 6. move any piece (on main field)
/// 7. jump from home to finish
/// 8. move piece forward in home
/// 9. If none of the above possible, there are no valid moves, skip turn.

pub async fn move_bot(state: GameServerState, msg: &ClientActorMessage, game: &mut Game) {
  let mut game = game.clone();
  while game.is_current_player_ai() {
    sleep(Duration::from_millis(3000)).await;

    let throw_sum = throw_dice_bot_messages(state.clone(), msg).await;
    // skip bot's move
    if throw_sum == 18 {
      game = skip_player(state.clone(), msg, &mut game).await;
      continue;
    }

    // ----------[ handles jumping from main field ]----------

    let player = game.get_current_player();
    let positions = game.get_players_pieces_positions(player.color);

    // -----[ 1. jump to finish ]-----

    let piece_positions_to_jump_to_finish: Vec<usize> = positions
      .clone()
      .into_iter()
      .filter(|position| game.can_jump_to_finish(*position, throw_sum))
      .collect();

    if !piece_positions_to_jump_to_finish.is_empty() {
      let move_result = game.execute_move(piece_positions_to_jump_to_finish[0], throw_sum, false);
      game = update_game_bot(state.clone(), msg, &mut game, move_result.clone()).await;
      if let MoveResult::Winner(_) = move_result.clone() {
        return;
      }
      continue;
    }

    // -----[ 2. jump to home ]-----

    let piece_positions_to_jump_home: Vec<usize> = positions
      .clone()
      .into_iter()
      .filter(|position| game.can_jump_to_home(*position, throw_sum))
      .collect();

    if !piece_positions_to_jump_home.is_empty() {
      let move_result = game.execute_move(piece_positions_to_jump_home[0], throw_sum, false);
      game = update_game_bot(state.clone(), msg, &mut game, move_result.clone()).await;
      // shouldn't be necessary - player/bot should never become a winner in this branch
      if let MoveResult::Winner(_) = move_result.clone() {
        return;
      }
      continue;
    }

    // -----[ 3. add new piece if only 1 is in main field ]-----

    if player.pawns_at_start + player.pawns_at_finish >= 3 && game.can_promote_piece(throw_sum) {
      let move_result = game.promote_piece(throw_sum);
      game = update_game_bot(state.clone(), msg, &mut game, move_result).await;
      continue;
    }

    // -----[ 4. remove enemy's piece ]-----

    let piece_positions_to_remove_enemy: Vec<usize> = positions
      .clone()
      .into_iter()
      .filter(|position| game.will_remove_enemy(*position, throw_sum))
      .collect();

    if !piece_positions_to_remove_enemy.is_empty() {
      let move_result = game.execute_move(piece_positions_to_remove_enemy[0], throw_sum, false);
      game = update_game_bot(state.clone(), msg, &mut game, move_result.clone()).await;
      continue;
    }

    // -----[ 5. add new piece to game ]-----

    if game.can_promote_piece(throw_sum) {
      let move_result = game.promote_piece(throw_sum);
      game = update_game_bot(state.clone(), msg, &mut game, move_result.clone()).await;
      continue;
    }

    // -----[ 6. move any piece (on main field) ]-----

    let piece_positions_to_move: Vec<usize> = positions
      .into_iter()
      .filter(|position| game.can_jump(*position, throw_sum))
      .collect();

    if !piece_positions_to_move.is_empty() {
      let move_result =
        game.execute_move(*piece_positions_to_move.last().unwrap(), throw_sum, false);
      game = update_game_bot(state.clone(), msg, &mut game, move_result.clone()).await;

      // shouldn't be necessary, since we can only move in board
      if let MoveResult::Winner(_) = move_result.clone() {
        return;
      }
      continue;
    }

    // ----------[ handles jumping from home ]----------

    let piece_positions_in_home: Vec<usize> =
      game.get_players_pieces_positions_in_home(player.color);

    let piece_positions_in_home_to_jump = piece_positions_in_home
      .clone()
      .into_iter()
      .filter(|position| game.can_jump_from_home(*position, throw_sum))
      .collect::<Vec<usize>>();

    // -----[ 7. jump from home to finish ]-----

    let piece_positions_in_home_to_jump_to_finish = piece_positions_in_home_to_jump
      .clone()
      .into_iter()
      .filter(|position| game.can_jump_from_home_to_finish(*position, throw_sum))
      .collect::<Vec<usize>>();

    if !piece_positions_in_home_to_jump_to_finish.is_empty() {
      let move_result = game.execute_move(
        *piece_positions_in_home_to_jump_to_finish.last().unwrap(),
        throw_sum,
        true,
      );
      game = update_game_bot(state.clone(), msg, &mut game, move_result.clone()).await;
      if let MoveResult::Winner(_) = move_result.clone() {
        return;
      }
      continue;
    }

    // -----[ 8. move piece forward in home ]-----

    if !piece_positions_in_home_to_jump.is_empty() {
      let move_result = game.execute_move(
        *piece_positions_in_home_to_jump.last().unwrap(),
        throw_sum,
        true,
      );
      game = update_game_bot(state.clone(), msg, &mut game, move_result.clone()).await;

      // shouldn't be necessary, since we can only move forward in home
      if let MoveResult::Winner(_) = move_result.clone() {
        return;
      }
      continue;
    }

    // -----[ 9. no valid moves available, skip turn ]-----
    game = skip_player(state.clone(), msg, &mut game).await;
  }
}

/// updates game based on move_result (set winner / change current player and empty dice_throws)
pub async fn update_game_bot(
  state: GameServerState,
  msg: &ClientActorMessage,
  game: &mut Game,
  move_result: MoveResult,
) -> Game {
  move_result_update_game(game, move_result);
  send_game_update_message(state.clone(), msg, game).await
}

/// inform player about value on dice after each roll
/// we don't need to keep updating game, since we perform rolling and move in the function / 'time frame'
pub async fn throw_dice_bot_messages(state: GameServerState, msg: &ClientActorMessage) -> usize {
  let mut throw_sum: usize = 0;

  let roll = get_dice_value();
  // can_roll_again is irrelevant
  send_roll_message(state.clone(), msg, roll, false).await;
  throw_sum += roll;

  if roll < 6 {
    return throw_sum;
  }

  let roll = get_dice_value();
  send_roll_message(state.clone(), msg, roll, false).await;
  throw_sum += roll;

  if roll < 6 {
    return throw_sum;
  }

  let roll = get_dice_value();
  send_roll_message(state.clone(), msg, roll, false).await;
  throw_sum += roll;

  throw_sum
}
