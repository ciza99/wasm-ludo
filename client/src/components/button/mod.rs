use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
  #[prop_or_default]
  pub children: Children,
  #[prop_or(false)]
  pub disabled: bool,
  pub onclick: Callback<MouseEvent>,
  #[prop_or_default]
  pub icon: Html,
  #[prop_or_default]
  pub class: String,
  #[prop_or(String::from("bg-primary-600"))]
  pub bg_color: String,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
  let ButtonProps {
    children,
    disabled,
    onclick,
    icon,
    class,
    bg_color,
  } = props.clone();

  let bg_color = if disabled {
    "bg-neutral-600".into()
  } else {
    bg_color
  };

  html! {
    <button {onclick} {disabled} class={classes!(String::from("rounded hover:brightness-90 text-white
      p-3 shadow-md font-semibold flex justify-center items-center gap-4"), bg_color, class)}
    >
      { icon }
      { for children.iter() }
    </button>
  }
}
