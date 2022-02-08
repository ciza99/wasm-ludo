use std::rc::Rc;
use yew::Reducible;

use super::hook::Toast;

pub enum ToastsAction {
  Add(Toast),
  Remove(u64),
  Closing(u64),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ToastsState {
  pub toasts: Vec<Toast>,
}

impl Default for ToastsState {
  fn default() -> Self {
    Self { toasts: Vec::new() }
  }
}

impl Reducible for ToastsState {
  type Action = ToastsAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    let toasts = match action {
      ToastsAction::Add(toast) => {
        let mut new_toasts = self.toasts.clone();
        new_toasts.push(toast);
        new_toasts
      }
      ToastsAction::Remove(id) => self.toasts.iter().cloned().filter(|t| t.id != id).collect(),
      ToastsAction::Closing(id) => self
        .toasts
        .iter()
        .cloned()
        .map(|toast| {
          if toast.id == id {
            Toast {
              closing: true,
              ..toast
            }
          } else {
            toast
          }
        })
        .collect(),
    };

    Self { toasts }.into()
  }
}
