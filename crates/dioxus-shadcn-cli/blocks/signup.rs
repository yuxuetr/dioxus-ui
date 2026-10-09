use dioxus::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::card::{
  Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::ui::checkbox::Checkbox;
use crate::components::ui::field::{Field, FieldDescription, FieldError, FieldLabel};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::progress::Progress;

/// What an account creation submits.
#[derive(Clone, Debug, PartialEq)]
pub struct SignUp {
  pub name: String,
  pub email: String,
  pub password: String,
}

/// Points out of 5: 8 and 12 characters, mixed case, a digit, and a symbol.
fn password_strength(password: &str) -> u8 {
  let length = password.chars().count();
  let checks = [
    length >= 8,
    length >= 12,
    password.chars().any(char::is_lowercase) && password.chars().any(char::is_uppercase),
    password.chars().any(|c| c.is_ascii_digit()),
    password.chars().any(|c| !c.is_alphanumeric()),
  ];
  checks.into_iter().filter(|passed| *passed).count() as u8
}

fn strength_label(strength: u8) -> &'static str {
  match strength {
    0 | 1 => "Weak",
    2 => "Fair",
    3 => "Good",
    _ => "Strong",
  }
}

/// An account creation page: name, email, and a password whose strength meter
/// follows each keystroke, plus a terms checkbox. Submit checks every field and
/// accepts a password from "Fair" up; a valid submit calls `on_sign_up`.
#[component]
pub fn SignupBlock(#[props(default)] on_sign_up: Option<EventHandler<SignUp>>) -> Element {
  let mut name = use_signal(String::new);
  let mut email = use_signal(String::new);
  let mut password = use_signal(String::new);
  let mut terms = use_signal(|| false);
  let mut submitted = use_signal(|| false);
  let strength = password_strength(&password());
  let strong_enough = password().chars().count() >= 8 && strength >= 2;
  let name_error = (submitted() && name().trim().is_empty()).then_some("Enter your name.");
  let email_error = (submitted() && !email().contains('@')).then_some("Enter your email address.");
  let password_error =
    (submitted() && !strong_enough).then_some("Use 8 or more characters with a mix of types.");
  let terms_error = (submitted() && !terms()).then_some("Accept the terms to continue.");

  rsx! {
    div { class: "flex min-h-screen items-center justify-center bg-muted p-4",
      Card { class: "w-full max-w-sm",
        CardHeader {
          CardTitle { "Create an account" }
          CardDescription { "It takes a minute. You can change these details later." }
        }
        CardContent {
          form {
            class: "grid gap-4",
            novalidate: true,
            onsubmit: move |event| {
              event.prevent_default();
              submitted.set(true);
              let valid = !name().trim().is_empty() && email().contains('@') && strong_enough && terms();
              if let Some(handler) = on_sign_up.filter(|_| valid) {
                handler.call(SignUp { name: name().trim().to_string(), email: email(), password: password() });
              }
            },
            Field { invalid: name_error.is_some(),
              FieldLabel { r#for: "signup-name", "Name" }
              Input {
                id: "signup-name",
                autocomplete: "name",
                value: name(),
                invalid: name_error.is_some(),
                "aria-describedby": name_error.map(|_| "signup-name-error"),
                on_value_change: move |value| name.set(value),
              }
              if let Some(message) = name_error {
                FieldError { id: "signup-name-error", "{message}" }
              }
            }
            Field { invalid: email_error.is_some(),
              FieldLabel { r#for: "signup-email", "Email" }
              Input {
                id: "signup-email",
                r#type: "email",
                autocomplete: "email",
                placeholder: "you@example.com",
                value: email(),
                invalid: email_error.is_some(),
                "aria-describedby": email_error.map(|_| "signup-email-error"),
                on_value_change: move |value| email.set(value),
              }
              if let Some(message) = email_error {
                FieldError { id: "signup-email-error", "{message}" }
              }
            }
            Field { invalid: password_error.is_some(),
              FieldLabel { r#for: "signup-password", "Password" }
              Input {
                id: "signup-password",
                r#type: "password",
                autocomplete: "new-password",
                value: password(),
                invalid: password_error.is_some(),
                "aria-describedby": if password_error.is_some() { "signup-password-strength signup-password-error" } else { "signup-password-strength" },
                on_value_change: move |value| password.set(value),
              }
              Progress {
                class: "h-1.5",
                value: f32::from(strength),
                max: 5.0,
                "aria-label": "Password strength",
                "aria-valuetext": strength_label(strength),
              }
              FieldDescription { id: "signup-password-strength", "Strength: {strength_label(strength)}" }
              if let Some(message) = password_error {
                FieldError { id: "signup-password-error", "{message}" }
              }
            }
            div { class: "grid gap-1",
              div { class: "flex items-center gap-2",
                Checkbox {
                  id: "signup-terms",
                  checked: terms(),
                  "aria-invalid": terms_error.is_some().to_string(),
                  "aria-describedby": terms_error.map(|_| "signup-terms-error"),
                  on_checked_change: move |checked| terms.set(checked),
                }
                Label { r#for: "signup-terms", "I accept the terms and privacy policy" }
              }
              if let Some(message) = terms_error {
                FieldError { id: "signup-terms-error", "{message}" }
              }
            }
            Button { class: "w-full", r#type: "submit", "Create account" }
          }
        }
        CardFooter { class: "justify-center gap-1 text-sm text-muted-foreground",
          "Have an account?"
          a { class: "font-medium text-foreground underline underline-offset-4", href: "#", "Sign in" }
        }
      }
    }
  }
}
