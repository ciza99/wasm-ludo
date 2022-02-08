use gloo::{console::log, timers::callback::Timeout};
use yew::prelude::*;

use super::{
  context::ToastOptions,
  reducer::{ToastsAction, ToastsState},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
  pub id: u64,
  pub options: ToastOptions,
  pub closing: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UseToasts {
  pub open: Callback<ToastOptions>,
  pub close: Callback<u64>,
  pub toasts: Vec<Toast>,
}

pub fn use_toasts() -> UseToasts {
  let key = use_mut_ref(|| 0_u64);
  let toasts = use_reducer(ToastsState::default);

  let close = {
    let toasts = toasts.clone();
    Callback::from(move |id: u64| {
      toasts.dispatch(ToastsAction::Closing(id));

      let toasts = toasts.clone();
      Timeout::new(500, move || {
        toasts.dispatch(ToastsAction::Remove(id));
      })
      .forget();
    })
  };

  let open = {
    let toasts = toasts.clone();
    Callback::from(move |toast_opts: ToastOptions| {
      let mut key = key.borrow_mut();
      let id = *key;
      *key = id + 1;
      log!(format!("here ist like this {:?}", *toasts));

      let toast = Toast {
        id,
        options: toast_opts,
        closing: false,
      };
      toasts.dispatch(ToastsAction::Add(toast));
    })
  };

  UseToasts {
    open,
    close,
    toasts: toasts.toasts.clone(),
  }
}
