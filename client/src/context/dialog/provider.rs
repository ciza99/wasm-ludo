use stylist::css;
use yew::prelude::*;

use crate::bindings::set_body_overflow;
use crate::context::dialog::hook::UseDialogValues;

use super::{context::DialogContext, hook::use_dialog};

#[derive(Properties, PartialEq, Clone)]
pub struct DialogProviderProps {
  pub children: Children,
}

#[function_component(DialogProvider)]
pub fn dialog_provider(props: &DialogProviderProps) -> Html {
  let UseDialogValues {
    content,
    open,
    close,
    is_open,
  } = use_dialog();

  let context = DialogContext { open };
  let content_html = content(close);

  let backdrop_class = css!("background-color: rgba(0, 0, 0, .5);");

  use_effect_with_deps(
    |is_open| {
      let value = if *is_open { "hidden" } else { "unset" };
      set_body_overflow(value.into());

      || {}
    },
    is_open,
  );

  html! {
    <ContextProvider<DialogContext> {context}>
      {for props.children.iter()}
      <div class={classes!(String::from("z-50 fixed top-0 left-0 w-screen h-screen grid place-items-center"), backdrop_class, (!is_open).then(|| "hidden"))}>
        <div class="rounded border border-neutral-300 w-full max-w-md p-4 bg-white shadow-lg">
          { content_html }
        </div>
      </div>
    </ContextProvider<DialogContext>>
  }
}
