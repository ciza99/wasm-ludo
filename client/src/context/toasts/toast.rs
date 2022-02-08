use gloo::timers::callback::Timeout;
use stylist::css;
use yew::prelude::*;

use crate::{components::icon::Icon, context::toasts::context::ToastVariant};

use super::context::ToastOptions;

#[derive(Properties, PartialEq, Clone)]
pub struct ToastProps {
  pub options: ToastOptions,
  pub close: Callback<()>,
  pub closing: bool,
}

const TIMEOUT: u32 = 5000;

#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
  let ToastProps {
    options,
    close,
    closing,
  } = props.clone();
  let ToastOptions { message, variant } = options;

  {
    let close = close.clone();
    use_effect_with_deps(
      move |_| {
        let timeout = Timeout::new(TIMEOUT, move || {
          close.emit(());
        });

        || {
          drop(timeout);
        }
      },
      (),
    );
  }

  let variant_class = match variant {
    ToastVariant::Success => "text-green-600",
    ToastVariant::Warning => "text-yellow-600",
    ToastVariant::Error => "text-red-600",
  };

  let icon = match variant {
    ToastVariant::Success => html! { <Icon class="fas fa-check" /> },
    ToastVariant::Warning => html! { <Icon class="fas fa-exclamation" /> },
    ToastVariant::Error => html! { <Icon class="fas fa-bug" /> },
  };

  let anim = if closing {
    css!(
      r#"
      animation: fadeOut 500ms ease;
      @keyframes fadeOut {
        from {
          opacity: 1;
        }
        to {
          opacity: 0;
          transform: scale(0.5);
        }
      }
      "#
    )
  } else {
    css!(
      r#"
      animation: fadeIn 500ms ease;
      @keyframes fadeIn {
        from {
          opacity: 0;
          transform: scale(0.5);
        }
      }
      "#
    )
  };

  let onclick = Callback::from(move |_| {
    close.emit(());
  });

  html! {
    <div class={classes!(String::from("relative rounded border-2 shadow-2xl bg-neutral-50 border-neutral-300 p-4"), closing.then(|| "opacity-0"), anim)}>
      <div class="absolute top-0 right-2">
        <Icon class="text-md fas fa-times text-neutral-600" {onclick} />
      </div>
      <div class="flex items-center">
        <div class={variant_class}>{icon}</div>
        <span class="ml-4 text-lg font-semibold mr-6">{ message.clone() }</span>
      </div>
    </div>
  }
}
