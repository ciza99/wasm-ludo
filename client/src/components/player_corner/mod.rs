use yew::prelude::*;
use stylist::{style, Style};

use crate::models::color::Color;
use crate::components::pawn::Pawn;
use crate::utils::{clamp, resolve_color_class};

#[derive(Properties, PartialEq, Clone)]
pub struct PlayerCornerProps {
  pub color: Color,
  pub pawn_count: u32,
}

#[function_component(PlayerCorner)]
pub fn player_corner(props: &PlayerCornerProps) -> Html {
  let PlayerCornerProps { color, pawn_count } = props.clone();

  let pawn_count = clamp(pawn_count, 0, 4);

  let color_class = resolve_color_class(&color);

  html! {
    <div class={classes!(String::from("h-full w-full grid place-items-center drop-shadow-lg"), color_class)}>
      <div class="w-1/2 h-1/2 rounded-full bg-neutral-100 grid grid-cols-2 grid-rows-2 p-4">
        {
          (0..pawn_count).map(|index| html! {<Pawn key={index} color={color.clone()}/>}).collect::<Vec<Html>>()
        }
      </div>
    </div>
  }
}