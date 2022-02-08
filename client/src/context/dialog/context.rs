use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone)]
pub struct OpenDialogOptions {
  pub content: Rc<dyn Fn(Callback<()>) -> Html>,
}

#[derive(Clone, PartialEq)]
pub struct DialogContext {
  pub open: Callback<OpenDialogOptions>,
}
