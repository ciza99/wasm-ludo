use gloo::storage::{SessionStorage, Storage};
use reqwasm::http::Request;
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;

use crate::components::button::Button;
use crate::components::card::Card;
use crate::components::content::Content;
use crate::components::text_input::TextInput;
use crate::context::toasts::context::{ToastOptions, ToastVariant, ToastsContext};
use crate::routes::GameRoute;
use crate::utils::get_host::HTTP_STRING;

#[derive(Properties, PartialEq, Clone)]
pub struct GameJoinProps {
  pub id: String,
}

#[derive(serde::Serialize, Deserialize)]
pub struct JoinGameBody {
  pub name: String,
}

#[function_component(GameJoin)]
pub fn game_join(props: &GameJoinProps) -> Html {
  let GameJoinProps { id } = props.clone();
  let ToastsContext { open } = use_context().expect("context not found");
  let history = use_history().unwrap();
  let nickname = use_state::<String, _>(|| "".into());

  // use_effect_with_deps(|_| {

  // }, []);

  let onchange = {
    let nickname = nickname.clone();
    Callback::from(move |event: InputEvent| {
      let target = event.target();
      let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

      if let Some(element) = input {
        nickname.set(element.value());
      }
    })
  };

  let onclick = {
    let nickname = nickname.clone();
    Callback::from(move |_| {
      let open = open.clone();
      let nickname = nickname.clone();
      let id = id.clone();
      let history = history.clone();
      spawn_local(async move {
        let body = JoinGameBody {
          name: (*nickname).clone(),
        };
        let body_json = serde_json::to_string(&body).unwrap();
        let res = Request::put(format!("{}/games/{}",HTTP_STRING, id).as_str())
          .header("Content-Type", "application/json")
          .body(body_json)
          .send()
          .await;

        let resp = match res {
          Ok(resp) => resp,
          Err(e) => {
            open.emit(ToastOptions {
              message: e.to_string(),
              variant: ToastVariant::Error,
            });
            return;
          }
        };

        if !resp.ok() {
          open.emit(ToastOptions {
            message: "Couldn't join game".into(),
            variant: ToastVariant::Error,
          });
          return;
        };

        let player_id = match resp.text().await {
          Ok(player_id) => player_id,
          Err(e) => {
            open.emit(ToastOptions {
              message: e.to_string(),
              variant: ToastVariant::Error,
            });
            return;
          }
        };

        if SessionStorage::set("player_id", player_id).is_err() {
          open.emit(ToastOptions {
            message: "Failed to set your player id".into(),
            variant: ToastVariant::Error,
          });
          return;
        };

        history.push(GameRoute::GameLobby { id });
      });
    })
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
        <p class="text-2xl mb-8 text-neutral-600 font-semibold">{"Enter your nickname bellow"}</p>
        <TextInput value={(*nickname).clone()} label={"Nickname:".to_string()} {onchange} />
        <Button class="w-full mt-8" {onclick} disabled={(*nickname).is_empty()}>{"Join the game!"}</Button>
      </Card>
    </Content>
  }
}
