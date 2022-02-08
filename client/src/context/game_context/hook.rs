use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use gloo::console::log;
use gloo::storage::{SessionStorage, Storage};
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::Message;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::context::{GameContext, MsgSender};
use super::game_reducer::GameState;
use crate::context::toasts::context::{ToastOptions, ToastVariant, ToastsContext};
use crate::models::messages::{ClientMessage, ServerMessage};
use crate::utils::get_host::WS_STRING;

#[derive(Properties, PartialEq, Clone)]
pub struct UseGameProps {
  pub game_id: String,
}

pub fn use_game(props: &UseGameProps) -> GameContext {
  let ToastsContext { open } = use_context::<ToastsContext>().expect("context not found");
  let game_state = use_reducer(GameState::default);
  let sender = use_state(|| None);
  let game_id = props.game_id.clone();
  let event_handler = use_state::<Option<Callback<ServerMessage>>, _>(|| None);

  let handle_message = {
    let game_state = game_state.clone();
    Callback::from(move |message: ServerMessage| {
      match message.clone() {
        ServerMessage::DiceValue(value, _) => {
          open.emit(ToastOptions {
            message: format!("Player rolled {}", value),
            variant: ToastVariant::Warning,
          });
        }
        ServerMessage::PlayerCountChange(count) => {
          open.emit(ToastOptions {
            message: format!("Players connected {}", count),
            variant: ToastVariant::Warning,
          });
        }
        ServerMessage::GameStarted(_) => {
          open.emit(ToastOptions {
            message: "Game started, good luck!".into(),
            variant: ToastVariant::Success,
          });
        }
        ServerMessage::Error(message) => {
          open.emit(ToastOptions {
            message,
            variant: ToastVariant::Error,
          });
        }
        _ => {}
      };
      game_state.dispatch(message);
    })
  };

  {
    let sender = sender.clone();
    let event_handler = event_handler.clone();
    use_effect_with_deps::<_, Box<dyn FnOnce()>, _>(
      move |callback| {
        let callback = (**callback).clone();
        let handle_message = handle_message.clone();
        let player_id: String = SessionStorage::get("player_id").unwrap();
        let ws = WebSocket::open(
          format!(
            "{}/games/websocket/{}/{}",
            WS_STRING, game_id, player_id
          )
          .as_str(),
        )
        .unwrap();

        let (mut write, mut read) = ws.split();
        let (tx, mut rx) = mpsc::channel::<ClientMessage>(1000);
        sender.set(Some(MsgSender(tx)));

        spawn_local(async move {
          // TODO: handle errors as well
          while let Some(Ok(Message::Text(text))) = read.next().await {
            log!(text.clone());
            if let Ok(message) = serde_json::from_str::<ServerMessage>(text.as_str()) {
              handle_message.emit(message.clone());
              if let Some(callback) = callback.clone() {
                callback.emit(message.clone());
              };
            } else {
              log!("Parsing of message failed:\n", text);
            }
          }
        });

        spawn_local(async move {
          while let Some(msg) = rx.next().await {
            let json = serde_json::to_string(&msg).unwrap();
            write.send(Message::Text(json)).await.unwrap();
          }
        });

        Box::new(|| {})
      },
      event_handler,
    );
  }

  let subscribe = {
    Callback::from(move |function: Callback<ServerMessage>| {
      event_handler.set(Some(function));
    })
  };

  GameContext {
    game: game_state.game.clone(),
    player_color: game_state.player_color.clone(),
    player_count: 0,
    subscribe,
    sender: (*sender).clone(),
    current_player: game_state.game.current_player.clone(),
    dice_info: game_state.dice_info.clone(),
  }
}
