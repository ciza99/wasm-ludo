use gloo::dialogs::alert;
use reqwasm::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::content::Content;
use crate::components::icon::Icon;

use crate::routes::MainRoute;

#[derive(Properties, PartialEq, Clone)]
pub struct HomeProps {}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
  let history = use_history().unwrap();

  let onclick = Callback::from(move |_| {
    let history = history.clone();
    spawn_local(async move {
      let res = Request::post("http://127.0.0.1:8080/games").send().await;

      let resp = match res {
        Ok(resp) => resp,
        Err(_) => {
          alert("something failed");
          return;
        }
      };

      let id = match resp.text().await {
        Ok(id) => id,
        Err(_) => {
          alert("something failed");
          return;
        }
      };

      history.push(MainRoute::GameJoin { id })
    });
  });

  let create_icon = html! {
    <Icon class={classes!(String::from("fas fa-gamepad"))}/>
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
        <p class="text-2xl font-bold text-neutral-800">{ "Start by creating a new game lobby" }</p>
        <ol class="list-disc list-inside ml-4 my-12">
          <li class="text-lg font-semibold text-neutral-600">{"Click the create new game lobby button"}</li>
          <li class="text-lg font-semibold text-neutral-600">{"Share the link with your friends"}</li>
          <li class="text-lg font-semibold text-neutral-600">{"Invite up to 4 friends to play"}</li>
          <li class="text-lg font-semibold text-neutral-600">{"Start the game when ready!"}</li>
        </ol>
        <Button class="w-full" {onclick} icon={create_icon}>{"Create new game lobby"}</Button>
      </Card>
    </Content>
  }
}
