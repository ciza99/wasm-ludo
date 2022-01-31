use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::models::color::Color;
use crate::types::Field;
use crate::utils::enums::MoveResult;

use super::player::Player;
use crate::utils::game::initialize_players;

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub started_at: DateTime,
    pub finished_at: Option<DateTime>,
    pub fields: Vec<Field>,
    pub players: Vec<Player>,
    pub current_player: Color
}


impl Game {
    pub fn new(player_names: Vec<String>) -> Self {
        Game{
            id: "".to_string(), // TODO
            started_at: mongodb::bson::DateTime::now(),
            finished_at: None,
            fields: vec![None; 48], // TODO check if this number is correct
            players: initialize_players(player_names),
            current_player: Color::Red,
        }
    }

    // there should be at most one winner at a time, therefore we take the first
    //   player that meets the winning condition
    pub fn check_winner(&self) -> Option<Color> {
        for player in &self.players {
            if player.check_winner() {
                return Some(player.color)
            }
        }
        None
    }

    pub fn field_size(&self) -> usize {
        self.fields.len()
    }

    pub fn update_current_player(&mut self) {
        self.current_player = match self.current_player {
            Color::Yellow => Color::Blue,
            Color::Blue => Color::Red,
            Color::Red => Color::Green,
            Color::Green => Color::Yellow
        }
    }

    // how many steps we need to make to reach the first field of player's home
    // e.g. curr_pos = 0, end_pos = 39 => distance = 40 (need to throw 40 to get to home)
    // max(end_pos + field_size) = 39, max(curr_pos) = 39
    pub fn distance_from_home(&self, current_position: usize) -> usize {
        // position of the field right in front of home
        let end_position = self.get_end_position();
        // +1 to get to the first home field
        (end_position + self.field_size() - current_position) % self.field_size() + 1
    }

    // how far away is the starting position (where we place pieces after throwing 6)
    //   from the ending position (= the last field before home)
    fn start_end_position_difference(&self) -> usize {
        // 1
        2
    }

    // we can use this 'modulo trick' to deal with different offsets and looping (pos 39 -> 0)
    // e.g. start_pos = 0 => end_pos = 39
    pub fn get_end_position(&self) -> usize {
        (self.get_starting_position() + self.fields.len() - self.start_end_position_difference()) % self.fields.len()
    }

    // returns size of the home column (finish)
    pub fn get_home_size(&self) -> usize {
        match self.players.get(0) {
            Some(player) => player.home.len(),
            None => 4
        }
    }

    pub fn is_occupied_by(&self, field: &Field, color: &Color) -> bool {
        match field {
            None => false,
            Some(_color) => _color == color
        }
    }

    pub fn get_players_pieces_positions(&self, color: &Color) -> Vec<usize> {
        self.fields
            .iter()
            .enumerate()
            .filter(|&(_position, field)| self.is_occupied_by(field, color))
            .map(|(position, _field)| position)
            .collect()
    }

    // we assume home_offset is valid
    pub fn get_home_field(&self, home_offset: usize) -> &Field {
        let player = self.get_current_player();
        // let player = self.players.iter().filter(|&player| player.color == player_color).next().unwrap();
        &player.home[home_offset]
    }

    pub fn is_in_bounds(&self, position: usize) -> bool {
        position < self.fields.len()
    }

    // there is a clock-wise ordering: Yellow, Blue, Red, Green
    // TODO: move to a utility, pass attr 'color' to replace 'self.current_player'
    pub fn get_offset(&self) -> usize {
        let offset = (self.fields.len() / 4) as usize;
        match self.current_player {
            Color::Yellow => 0,
            Color::Blue => offset,
            Color::Red => offset * 2,
            Color::Green => offset * 3
        }
    }

    // position of the field where we put pieces after throwing 6
    pub fn get_starting_position(&self) -> usize {
        self.get_offset() + 8
    }

    // if we land on opponent at 'position', we remove his piece (we can't jump on our own piece)
    pub fn clear_field(&mut self, position: usize) {
        match &self.fields[position] {
            None => (),
            Some(color) => self.remove_players_piece(*color)
        }
    }

    pub fn remove_players_piece(&mut self, color: Color) {
        let mut player = self.get_player_mut(color);
        player.return_piece_to_start();
    }

    // add check for player.pawns_at_start > 0 ?
    // can jump to starting position
    /// check if starting position is empty
    pub fn is_start_empty(&self) -> bool {
        self.is_available_field(self.get_starting_position())
    }

    pub fn promote_piece(&mut self) {
        let position = self.get_starting_position();
        self.clear_field(position);
        let mut player = self.get_player_mut(self.current_player);
        player.promote_piece();
        self.fields[position] = Some(self.current_player)
    }


    // we can jump to a field, if it's either empty or occupied by opponent,
    // i.e. it's not occupied by us
    pub fn is_available_field(&self, position: usize) -> bool {
        self.is_in_bounds(position) && !self.is_current_players_piece(position)
    }

    pub fn opponent_at_field(&self, position: usize) -> bool {
        self.is_in_bounds(position) && !self.is_current_players_piece(position) && self.is_opponents_piece(position)
    }

    pub fn get_new_position(&self, position: usize, dice_value: usize) -> usize {
        (position + dice_value) % self.fields.len()
    }

    // if we can make a move/jump within main board/field (not reaching home)
    pub fn can_jump(&self, position: usize, dice_value: usize) -> bool {
        dice_value < self.distance_from_home(position) &&
            self.is_available_field(self.get_new_position(position, dice_value))
    }

    pub fn will_remove_enemy(&self, position: usize, dice_value: usize) -> bool {
        dice_value < self.distance_from_home(position) &&
            self.is_opponents_piece(self.get_new_position(position, dice_value))
    }

    pub fn is_in_bounds_home(&self, home_offset: usize) -> bool {
        home_offset < self.get_home_size()
    }

    pub fn is_available_home_field(&self, home_offset: usize) -> bool {
        self.is_in_bounds_home(home_offset) && !self.is_home_field_occupied(home_offset)
    }

    /// check if piece can reach `safe zone`
    pub fn can_jump_to_home(&self, position: usize, dice_value: usize) -> bool {
        // let distance_from_home = self.distance_from_home(position);
        // let will_reach_home = dice_value >= distance_from_home;
        // let will_not_overjump_home = dice_value < distance_from_home + self.get_home_size();
        // match will_reach_home && will_not_overjump_home {
        //     true => {
        //         let home_offset = dice_value - distance_from_home;
        //         self.is_available_home_field(home_offset)
        //     },
        //     false => false
        // }

        match self.can_reach_home(position, dice_value) && !self.would_overjump_home(position, dice_value) {
            true => self.is_available_home_field(self.get_home_offset(position, dice_value)),
            false => false
        }
    }

    pub fn jump(&mut self, old_position: usize, new_position: usize) {
        self.fields[old_position] = None;
        self.clear_field(new_position);
        self.fields[new_position] = Some(self.current_player)
    }

    // we assume we jump from 'main fields' to player's home
    // this currently doesn't allow moving pieces within home itself - we would just have to
    //    distinguish between old_position in self.fields and in home, so that we can clear
    //    the correct field
    pub fn jump_home(&mut self, old_position: usize, home_offset: usize) {
        self.fields[old_position] = None;
        let color = self.current_player;
        let mut home = self.get_home_mut();
        home[home_offset] = Some(color);
    }

    // if we move 'dice_value' fields, we will reach beyond the main board/field
    pub fn can_reach_home(&self, position: usize, dice_value: usize) -> bool {
        dice_value >= self.distance_from_home(position)
    }

    // if we move 'dice_value' fields, we will reach beyond the main board/field
    pub fn can_reach_finish(&self, position: usize, dice_value: usize) -> bool {
        dice_value == self.distance_from_home(position) + self.get_home_size()
    }

    // distance_from_home gets you already to the first home field, that's why '>=' and not only '>'
    pub fn would_overjump_home(&self, position: usize, dice_value: usize) -> bool {
        dice_value >= self.distance_from_home(position) + self.get_home_size()
    }

    // returns position/index of field in player's home column where we will jump,
    // i.e. offset in player's home column
    // e.g. if piece is right in front of home => distance = 1, and if we throw a 1,
    //      we would reach the first home field (home_offset = 0)
    pub fn get_home_offset(&self, position: usize, dice_value: usize) -> usize {
        dice_value - self.distance_from_home(position)
    }

    pub fn get_home(&self) -> &Vec<Field> {
        let player = self.get_current_player();
        &player.home
    }

    pub fn get_home_mut(&mut self) -> &mut Vec<Field> {
        let mut player = self.get_current_player_mut();
        &mut player.home
    }

    pub fn is_home_field_occupied(&self, home_offset: usize) -> bool {
        match self.get_home_field(home_offset) {
            None => false,
            Some(_) => true
        }
    }

    // jump from home column (1 of 5 home fields) to finish
    pub fn jump_from_home_to_finish(&mut self, home_offset: usize) {
        let home = self.get_home_mut();
        home[home_offset] = None;
        let mut player = self.get_current_player_mut();
        player.pawns_at_finish += 1;
    }

    // jump from main field to finish
    pub fn jump_to_finish(&mut self, position: usize) {
        self.fields[position] = None;
        let mut player = self.get_current_player_mut();
        player.pawns_at_finish += 1;
    }

    pub fn jump_from_home(&mut self, old_home_offset: usize, new_home_offset: usize) {
        let color = self.current_player;
        let home = self.get_home_mut();
        home[old_home_offset] = None;
        home[new_home_offset] = Some(color)
    }

    // when we are trying to move piece in home column (1 out of 5 home fields)
    fn execute_move_from_home(&mut self, home_offset: usize, dice_value: usize) -> MoveResult {

        let distance_from_home = self.get_home_size() - home_offset;
        match dice_value == distance_from_home {
            true => {
                self.jump_from_home_to_finish(home_offset);
                MoveResult::Success(String::from("Move successful."))
            },
            false => match dice_value > distance_from_home {
                true => MoveResult::Error(String::from("Would overjump home.")),
                false => {
                    let new_home_offset = home_offset + dice_value;
                    match self.is_available_home_field(new_home_offset) {
                        true => {
                            self.jump_from_home(home_offset, new_home_offset);
                            MoveResult::Success(String::from("Move successful."))
                        },
                        false => MoveResult::Error(String::from("Home field is occupied.")),
                    }
                }
            }
        }
    }

    // as of now, we assume we can only move pieces from 'main fields', not home
    pub fn execute_move(&mut self, position: usize, dice_value: usize, home_column: bool) -> MoveResult {

        // dice_value = 0 means player threw 3x6, therefore he gets skipped (should we create a message ?)
        // MoveResult::SkipPlayer
        if dice_value == 0 {
            return MoveResult::Success(String::from("Throwing 3x6 means you have to wait a round."));
        }

        //TODO we need to get message if player wants to promote his piece (how?)
        if position == 100 {
            if dice_value < 6 {
                return MoveResult::Error(String::from("We can't promote - did not roll 6."));
            }
            match self.is_start_empty() {
                false => return MoveResult::Error(String::from("We can't promote - starting field is occupied by our piece.")),
                true => {
                    self.promote_piece();
                    return MoveResult::Success(String::from("Your piece has been promoted!"));
                }
            }
        }

        // we are trying to moving piece from
        if home_column {
            return self.execute_move_from_home(position, dice_value)
        }

        if self.can_reach_finish(position, dice_value) {
            self.jump_to_finish(position);
            return MoveResult::Success(String::from("Jumped to finish!"))
        }

        match self.can_reach_home(position, dice_value) {
            true => {
                // first we check a situation where we overjump home
                if self.would_overjump_home(position, dice_value) {
                    return MoveResult::Error(String::from("Can't move - would overjump home."))
                }

                // offset/position in player's home column
                let home_offset = self.get_home_offset(position, dice_value);
                match self.is_available_home_field(home_offset) {
                    false => MoveResult::Error(String::from("Can't move - home field is already occupied.")),
                    true => {
                        self.jump_home(position, home_offset);
                        MoveResult::Success(String::from("Successfully moved a piece to home!"))
                    }
                }
            },
            false => {
                let new_position = self.get_new_position(position, dice_value);
                match self.is_available_field(new_position) {
                    false => MoveResult::Error(String::from("Can't move - field is occupied by our piece.")),
                    true => {
                        self.jump(position, new_position);
                        MoveResult::Success(String::from("Moved to a new position."))
                    }
                }
            }
        }


    }

    // returns whether a field specified by <position> is is occupied by a piece with <color>
    pub fn is_players_piece(&self, position: usize, player_color: &Color) -> bool {
        match self.fields.get(position) {
            Some(field) => match field {
                Some(color) => color == player_color,
                None => false
            }
            None => false
        }
    }

    pub fn is_opponents_piece(&self, position: usize) -> bool {
        match self.fields.get(position) {
            Some(field) => match field {
                Some(color) => color != &self.current_player,
                None => false
            }
            None => false
        }
    }

    pub fn is_current_players_piece(&self, position: usize) -> bool {
        match self.fields.get(position) {
            Some(field) => match field {
                Some(color) => color == &self.current_player,
                None => false
            }
            None => false
        }
    }

    // returns whether a field is empty
    pub fn is_field_empty(&self, position: usize) -> bool {
        match self.fields.get(position) {
            Some(field) => match field {
                Some(_) => false,
                None => true
            }
            None => false
        }
    }

    pub fn get_player(&self, player_color: Color) -> &Player {
        self.players.iter().filter(|&player| player.color == player_color).next().unwrap()
    }

    pub fn get_player_mut(&mut self, player_color: Color) -> &mut Player {
       self.players.iter_mut().filter(|player| player.color == player_color).next().unwrap()
    }

    pub fn get_current_player(&self) -> &Player {
        self.get_player(self.current_player)
    }

    pub fn get_current_player_mut(&mut self) -> &mut Player {
        self.get_player_mut(self.current_player)
    }

    pub fn is_player_ai(&self, player_color: Color) -> bool {
        let bots: Vec<Color> = self.players
            .iter()
            .filter(|&player| player.is_bot)
            .map(|player| player.color)
            .collect();
        bots.contains(&player_color)
    }

    pub fn is_current_player_ai(&self) -> bool {
        self.is_player_ai(self.current_player)
    }
}
