use dioxus::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{
  Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::field::{Field, FieldError, FieldLabel};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::separator::Separator;

/// What a sign-in submits.
#[derive(Clone, Debug, PartialEq)]
pub struct SignIn {
  pub email: String,
  pub password: String,
  pub remember: bool,
}

/// A sign-in page: email and password checked when submitted, a remember-me
/// checkbox, and a second provider button. A valid submit calls `on_sign_in`;
/// connect it to your authentication.
#[component]
pub fn LoginBlock(
  #[props(default)] on_sign_in: Option<EventHandler<SignIn>>,
  #[props(default)] on_provider: Option<EventHandler<()>>,
) -> Element {
  let mut email = use_signal(String::new);
  let mut password = use_signal(String::new);
  let mut remember = use_signal(|| true);
  let mut submitted = use_signal(|| false);
  let email_error = (submitted() && !email().contains('@')).then_some("Enter your email address.");
  let password_error =
    (submitted() && password().chars().count() < 8).then_some("Use at least 8 characters.");

  rsx! {
    div { class: "flex min-h-screen items-center justify-center bg-muted p-4",
      Card { class: "w-full max-w-sm",
        CardHeader {
          CardTitle { "Sign in" }
          CardDescription { "Enter your email and password to continue." }
        }
        CardContent {
          form {
            class: "grid gap-4",
            novalidate: true,
            onsubmit: move |event| {
              event.prevent_default();
              submitted.set(true);
              let valid = email().contains('@') && password().chars().count() >= 8;
              if let Some(handler) = on_sign_in.filter(|_| valid) {
                handler.call(SignIn { email: email(), password: password(), remember: remember() });
              }
            },
            Field { invalid: email_error.is_some(),
              FieldLabel { r#for: "login-email", "Email" }
              Input {
                id: "login-email",
                r#type: "email",
                autocomplete: "email",
                placeholder: "you@example.com",
                value: email(),
                invalid: email_error.is_some(),
                "aria-describedby": email_error.map(|_| "login-email-error"),
                on_value_change: move |value| email.set(value),
              }
              if let Some(message) = email_error {
                FieldError { id: "login-email-error", "{message}" }
              }
            }
            Field { invalid: password_error.is_some(),
              FieldLabel { r#for: "login-password", "Password" }
              Input {
                id: "login-password",
                r#type: "password",
                autocomplete: "current-password",
                value: password(),
                invalid: password_error.is_some(),
                "aria-describedby": password_error.map(|_| "login-password-error"),
                on_value_change: move |value| password.set(value),
              }
              if let Some(message) = password_error {
                FieldError { id: "login-password-error", "{message}" }
              }
            }
            div { class: "flex items-center gap-2",
              Checkbox {
                id: "login-remember",
                checked: remember(),
                on_checked_change: move |checked| remember.set(checked),
              }
              Label { r#for: "login-remember", "Remember me" }
            }
            Button { class: "w-full", r#type: "submit", "Sign in" }
          }
          div { class: "my-4 flex items-center gap-3 text-xs text-muted-foreground",
            Separator { class: "flex-1", decorative: true }
            "or"
            Separator { class: "flex-1", decorative: true }
          }
          Button {
            class: "w-full",
            variant: ButtonVariant::Outline,
            onclick: move |_| {
              if let Some(handler) = on_provider {
                handler.call(());
              }
            },
            "Continue with SSO"
          }
        }
        CardFooter { class: "justify-center gap-1 text-sm text-muted-foreground",
          "No account?"
          a { class: "font-medium text-foreground underline underline-offset-4", href: "#", "Sign up" }
        }
      }
    }
  }
}
