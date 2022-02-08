use yew::events::InputEvent;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TextInputProps {
  pub label: String,
  pub value: String,
  pub onchange: Callback<InputEvent>,
}

#[function_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
  let TextInputProps {
    label,
    value,
    onchange,
  } = props.clone();

  html! {
    <div class="w-full">
      <p class="font-semibold text-neutral-600">{label}</p>
      <input class="w-full border border-neutral-300 p-3 focus:border-primary-600 rounded shadow focus:outline-none" oninput={ &onchange } {value} />
    </div>
  }
}
