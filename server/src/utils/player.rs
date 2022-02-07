use crate::models::game::Game;
use crate::utils::enums::MoveResult;

use super::enums::MoveType;

pub fn make_a_move(game: &mut Game, player_move: MoveType) -> MoveResult {
  let dice_value = game.dice_throws.iter().sum();
  match player_move {
    MoveType::Promote => game.promote_piece(dice_value),
    MoveType::Move(position) => game.execute_move(position.position, dice_value, position.is_home),
  }
}

pub fn get_available_positions(game: &Game, dice_value: usize) -> (Vec<usize>, Vec<usize>, bool) {
  let positions = game.get_players_pieces_positions(game.current_player);
  let player = game.get_current_player();

  let mut positions_on_board: Vec<usize> = positions
    .clone()
    .into_iter()
    .filter(|position| game.can_jump(*position, dice_value))
    .collect();

  let mut piece_positions_to_jump_home: Vec<usize> = positions
    .clone()
    .into_iter()
    .filter(|position| game.can_jump_to_home(*position, dice_value))
    .collect();

  let mut piece_positions_to_jump_to_finish: Vec<usize> = positions
    .into_iter()
    .filter(|position| game.can_jump_to_finish(*position, dice_value))
    .collect();

  positions_on_board.append(&mut piece_positions_to_jump_home);
  positions_on_board.append(&mut piece_positions_to_jump_to_finish);

  let can_promote = player.pawns_at_start > 0 && game.can_promote_piece(dice_value);

  let piece_positions_in_home_row: Vec<usize> =
    game.get_players_pieces_positions_in_home(player.color);

  let piece_positions_in_home_row = piece_positions_in_home_row
    .into_iter()
    .filter(|&position| game.can_jump_from_home(position, dice_value))
    .collect();

  (positions_on_board, piece_positions_in_home_row, can_promote)
}
