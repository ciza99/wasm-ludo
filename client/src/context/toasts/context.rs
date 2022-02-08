use yew::prelude::*;

#[allow(dead_code)]
#[derive(PartialEq, Clone, Debug)]
pub enum ToastVariant {
  Success,
  Warning,
  Error,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ToastOptions {
  pub message: String,
  pub variant: ToastVariant,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ToastsContext {
  pub open: Callback<ToastOptions>,
}
