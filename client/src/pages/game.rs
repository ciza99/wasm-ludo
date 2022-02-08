use std::rc::Rc;

use yew::prelude::*;

use crate::components::board::Board;
use crate::components::dialogs::winner_dialog::WinnerDialog;
use crate::components::player::{Player, PlayerButtonPosition};
use crate::context::dialog::context::{DialogContext, OpenDialogOptions};
use crate::context::game_context::context::GameContext;
use crate::models::color::Color;

#[function_component(Game)]
pub fn game() -> Html {
  let DialogContext { open } = use_context::<DialogContext>().expect("context not found");
  let GameContext { game, .. } = use_context::<GameContext>().expect("context not found");

  let winner_name = game
    .winner
    .as_ref()
    .and_then(|color| game.players.iter().find(|player| player.color == *color))
    .map(|player| player.name.clone());

  use_effect_with_deps(
    move |winner_name| {
      if let Some(winner_name) = winner_name.clone() {
        open.emit(OpenDialogOptions {
          content: Rc::new(
            move |close| html! { <WinnerDialog {close} winner_name={winner_name.clone()} /> },
          ),
        });
      }

      || {}
    },
    winner_name,
  );

  html! {
    <div class="py-4 flex">
      <div class="flex flex-col justify-between item-center p-4 max-w-md flex-grow">
        <Player position={PlayerButtonPosition::Bottom} color={Color::Yellow} />
        <Player position={PlayerButtonPosition::Top} color={Color::Green} />
      </div>
      <div class="flex-grow">
        <Board />
      </div>
      <div class="flex flex-col justify-between item-center p-4 max-w-md flex-grow">
        <Player position={PlayerButtonPosition::Bottom} color={Color::Blue} />
        <Player position={PlayerButtonPosition::Top} color={Color::Red} />
      </div>
    </div>
  }
}
