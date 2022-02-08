use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};

use crate::{components::button::Button, routes::MainRoute};

#[derive(Properties, PartialEq, Clone)]
pub struct WinnerDialogProps {
  pub close: Callback<()>,
  pub winner_name: String,
}

#[function_component(WinnerDialog)]
pub fn winner_dialog(props: &WinnerDialogProps) -> Html {
  let history = use_history().unwrap();
  let WinnerDialogProps { close, winner_name } = props.clone();

  let onclick = Callback::from(move |_| {
    close.emit(());
    history.push(MainRoute::Home);
  });

  html! {
    <div class="flex flex-col gap-6 text-neutral-600">
      <span class="text-3xl font-bold">{ "Game Over !" }</span>
      <p class="mb-2 text-xl">{"The winner is:"}
        <span class="ml-4 font-semibold text-green-600">{winner_name}</span>
      </p>
      <Button class="w-full" {onclick}>
        {"Go back to homepage"}
      </Button>
    </div>
  }
}
