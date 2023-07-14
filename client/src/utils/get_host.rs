#[cfg(debug_assertions)]
pub const HTTP_STRING: &str =  "http://127.0.0.1:8080";

#[cfg(not(debug_assertions))]
pub const HTTP_STRING: &str = "http://3.78.187.83:8080";

#[cfg(debug_assertions)]
pub const JOIN_STRING: &str =  "http://localhost:3000";

#[cfg(not(debug_assertions))]
pub const JOIN_STRING: &str = "http://3.121.215.195";

#[cfg(debug_assertions)]
pub const WS_STRING: &str =   "ws://127.0.0.1:8080";

#[cfg(not(debug_assertions))]
pub const WS_STRING: &str = "ws://3.78.187.83:8080";
