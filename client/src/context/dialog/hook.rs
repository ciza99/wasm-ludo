use std::rc::Rc;
use yew::prelude::*;

use super::context::OpenDialogOptions;

pub struct UseDialogValues {
  pub is_open: bool,
  pub close: Callback<()>,
  pub open: Callback<OpenDialogOptions>,
  pub content: Rc<dyn Fn(Callback<()>) -> Html>,
}

#[function_component(DefaultDialog)]
fn default_dialog() -> Html {
  html! {}
}

pub fn use_dialog() -> UseDialogValues {
  let is_open = use_state(|| false);
  let content = use_state::<Rc<dyn Fn(Callback<()>) -> Html>, _>(|| Rc::new(|_| html! {}));

  let close = {
    let is_open = is_open.clone();
    Callback::from(move |_| {
      is_open.set(false);
    })
  };

  let open = {
    let is_open = is_open.clone();
    let content = content.clone();
    Callback::from(move |open_props: OpenDialogOptions| {
      content.set(open_props.content);
      is_open.set(true);
    })
  };

  UseDialogValues {
    content: (*content).clone(),
    is_open: *is_open,
    close,
    open,
  }
}
