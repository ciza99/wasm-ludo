use actix::Recipient;
use std::collections::{HashMap, HashSet};

use crate::models::actor_messages::WsMessage;

pub fn send_message(message: &str, sessions: HashMap<String, Recipient<WsMessage>>, id_to: &str) {
  if let Some(session) = sessions.get(id_to) {
    session.do_send(WsMessage(message.to_owned()));
  } else {
    println!("attempting to send message but couldn't find session with given id.");
  }
}

pub fn send_message_to_room(
  message: &str,
  sessions: HashMap<String, Recipient<WsMessage>>,
  rooms: HashMap<String, HashSet<String>>,
  room_id: &str,
) {
  if let Some(sessions_ids) = rooms.get(room_id) {
    for session_id in sessions_ids {
      send_message(message, sessions.clone(), session_id);
    }
  }
}
