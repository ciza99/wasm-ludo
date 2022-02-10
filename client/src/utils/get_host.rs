#[cfg(debug_assertions)]
pub const HTTP_STRING: &str =  "http://127.0.0.1:8080";

#[cfg(not(debug_assertions))]
pub const HTTP_STRING: &str = "https://ludo-be.herokuapp.com";

#[cfg(debug_assertions)]
pub const JOIN_STRING: &str =  "http://localhost:3000";

#[cfg(not(debug_assertions))]
pub const JOIN_STRING: &str = "https://ludo-fe.herokuapp.com";

#[cfg(debug_assertions)]
pub const WS_STRING: &str =   "ws://127.0.0.1:8080";

#[cfg(not(debug_assertions))]
pub const WS_STRING: &str = "wss://ludo-be.herokuapp.com";