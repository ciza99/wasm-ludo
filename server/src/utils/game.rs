use crate::models::color::Color;
use crate::models::game::Game;
use crate::models::player::Player;
use crate::utils::bot::create_bot_name;
use crate::utils::enums::MoveResult;
use crate::utils::player::make_a_move;

use super::enums::MoveType;

pub fn fill_with_bots(players: Vec<Player>) -> Vec<Player> {
  let colors = [Color::Red, Color::Green, Color::Blue, Color::Yellow];
  colors.iter().fold(Vec::new(), |mut acc, color| {
    if let Some(player) = players
      .iter()
      .cloned()
      .find(|player| player.color == *color)
    {
      acc.push(player);
    } else {
      acc.push(Player::new(
        "0".to_string(),
        create_bot_name(),
        *color,
        true,
      ));
    }
    acc
  })
}

/// called upon receiving either PromotePiece or MovePiece(position, Option<Color>)
pub async fn play_round(game: &mut Game, move_type: MoveType) -> MoveResult {
  let mut move_result = make_a_move(game, move_type);

  if let Some(winner) = game.check_winner() {
    move_result = MoveResult::Winner(winner);
    game.finish_game(winner);
  }

  if let MoveResult::Success(_) = move_result {
    game.update_current_player();
    game.dice_throws.clear();
  }

  move_result
}
