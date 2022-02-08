mod clamp;
pub mod color_to_name;
mod resolve_color;
pub mod get_host;
pub use clamp::clamp;

pub use resolve_color::resolve_bg_color_class;
pub use resolve_color::resolve_text_color_class;
