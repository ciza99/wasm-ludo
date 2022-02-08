use yew::prelude::*;

use super::toast::Toast;
use crate::context::toasts::context::ToastsContext;

use super::hook::{use_toasts, UseToasts};

#[derive(Properties, PartialEq, Clone)]
pub struct SnackbarProviderProps {
  #[prop_or_default]
  pub children: Children,
}

#[function_component(SnackbarProvider)]
pub fn snackbar_provider(props: &SnackbarProviderProps) -> Html {
  let UseToasts {
    open,
    close,
    toasts,
  } = use_toasts();
  let context = ToastsContext { open };

  let toasts = toasts.iter().cloned().map(|toast| {
    let close = close.clone();

    let id = toast.id;
    let close = Callback::from(move |_| {
      close.emit(id);
    });

    html! {
      <Toast key={id.to_string()} {close} options={toast.options} closing={toast.closing} />
    }
  });

  html! {
    <ContextProvider<ToastsContext> context={context}>
      { for props.children.iter() }
      <div class="fixed left-4 bottom-4 flex flex-col gap-2 items-start">
        { for toasts }
      </div>
    </ContextProvider<ToastsContext>>
  }
}
