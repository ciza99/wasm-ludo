// use futures::channel::oneshot::channel;
use futures::SinkExt;
use gloo::timers::callback::Interval;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::content::Content;
use crate::components::copy_bar::CopyBar;
use crate::components::icon::Icon;
use crate::components::outlined_item::OutlinedItem;
use crate::context::game_context::context::GameContext;
use crate::models::messages::{ClientMessage, ServerMessage};
use crate::routes::{GameRoute, MainRoute};
use crate::utils::get_host::JOIN_STRING;

#[derive(Properties, PartialEq, Clone)]
pub struct GameLobbyProps {
  pub id: String,
}

#[function_component(GameLobby)]
pub fn game_lobby(props: &GameLobbyProps) -> Html {
  let GameLobbyProps { id } = props.clone();
  let GameContext {
    subscribe, sender, ..
  } = use_context::<GameContext>().expect("provider is not a parent");
  let history = use_history().unwrap();
  let player_count = use_state(|| 0);
  let seconds = use_state(|| 0);

  {
    let id = id.clone();
    let history = history.clone();
    let player_count = player_count.clone();
    use_effect_with_deps(
      move |_: &[u32; 0]| {
        subscribe.emit(Callback::from(
          move |message: ServerMessage| match message {
            ServerMessage::PlayerCountChange(count) => player_count.set(count),
            ServerMessage::GameStarted(_) => {
              history.push(GameRoute::Game { id: id.clone() });
            }
            _ => {}
          },
        ));

        || {}
      },
      [],
    );
  }

  let on_start = {
    Callback::from(move |_| {
      let sender = sender.clone();
      spawn_local(async move {
        if let Some(mut sender) = sender.clone() {
          sender.0.send(ClientMessage::StartGame).await.ok();
        };
      });
    })
  };

  let redirect_to_home = Callback::from(move |_| {
    history.push(MainRoute::Home);
  });

  {
    let seconds = seconds.clone();
    use_effect(move || {
      let interval = Interval::new(1000, move || seconds.set(*seconds + 1));

      move || {
        drop(interval);
      }
    });
  }

  let start_icon = html! {
    <Icon class="fas fa-play"/>
  };

  let leave_icon = html! {
    <Icon class="fas fa-sign-out-alt"/>
  };

  let players_item = html! {
    {format!("{} / 4", *player_count)}
  };

  let time_item = html! {
    {format!("{} seconds", *seconds)}
  };

  html! {
    <Content class="py-12 h-full">
      <div class="flex items-center mb-6 w-full">
        <div class="flex flex-col gap-2 w-full justify-between">
          <p class="text-5xl font-bold">{"Ludo"}</p>
          <p class="text-2xl text-neutral-600 font-bold">{"Board game for up to 4 players online"}</p>
        </div>
        <img class="h-28" src="/assets/ludo.svg" alt="" />
      </div>
      <Card class="w-full px-8 py-14 lg:px-40">
        <p class="text-xl text-neutral-600 font-bold">{"Share the link with your friends and start the game"}</p>
        <CopyBar content={ format!("{}/games/{}/join",JOIN_STRING, id) } />
        <div class="flex items-center gap-3 text-neutral-600 mt-16">
          <Icon class="fas fa-info-circle" />
          <p class="text-xl font-bold">{"Starting the game without all 4 players will fill the remaining spots with
            bots"}</p>
        </div>
        <div class="flex flex-col gap-3">
          <OutlinedItem label="Players connected" item={players_item} />
          <OutlinedItem label="Time in lobby" item={time_item} />
        </div>
        <div class="w-full flex justify-end">
          <span>{"Waiting for other players to join"}</span>
        </div>
        <div class="flex items-center gap-3 mt-16">
          <Button class="w-full" onclick={on_start} icon={start_icon}>{"Start the game!"}</Button>
          <Button class="w-full" bg_color="bg-red-700" onclick={redirect_to_home} icon={leave_icon}>{"Leave the lobby"}</Button>
        </div>
      </Card>
    </Content>
  }
}
