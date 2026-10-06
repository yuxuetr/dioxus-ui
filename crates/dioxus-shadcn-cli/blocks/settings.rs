use dioxus::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{
  Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::ui::field::{Field, FieldDescription, FieldError, FieldLabel};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::native_select::{NativeSelect, NativeSelectOption};
use crate::components::ui::switch::Switch;
use crate::components::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
use crate::components::ui::textarea::Textarea;

/// The settings a save submits.
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
  pub name: String,
  pub email: String,
  pub bio: String,
  pub language: String,
  pub product_updates: bool,
  pub security_alerts: bool,
  pub weekly_digest: bool,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      name: "Ada Lovelace".to_string(),
      email: "ada@acme.example".to_string(),
      bio: String::new(),
      language: "en".to_string(),
      product_updates: true,
      security_alerts: true,
      weekly_digest: false,
    }
  }
}

const LANGUAGES: [(&str, &str); 4] =
  [("en", "English"), ("de", "Deutsch"), ("ja", "日本語"), ("zh", "中文")];

/// A settings page with Profile and Notifications tabs. Save checks the
/// profile and calls `on_save` with the settings; Reset restores the last
/// saved ones. Start it from your stored settings with `initial`.
#[component]
pub fn SettingsBlock(
  #[props(default)] initial: Settings,
  #[props(default)] on_save: Option<EventHandler<Settings>>,
) -> Element {
  let mut saved = use_signal(|| initial.clone());
  let mut draft = use_signal(|| initial.clone());
  let mut tab = use_signal(|| "profile".to_string());
  let mut attempted = use_signal(|| false);
  let name_error = (attempted() && draft().name.trim().is_empty()).then_some("Enter your name.");
  let email_error =
    (attempted() && !draft().email.contains('@')).then_some("Enter an email address.");
  let changed = draft() != saved();

  let toggle = |label: &'static str, id: &'static str, checked: bool, description: &'static str| {
    (label, id, checked, description)
  };
  let notifications = [
    toggle(
      "Product updates",
      "settings-updates",
      draft().product_updates,
      "News about features and releases.",
    ),
    toggle(
      "Security alerts",
      "settings-security",
      draft().security_alerts,
      "Sign-ins from new devices and password changes.",
    ),
    toggle(
      "Weekly digest",
      "settings-digest",
      draft().weekly_digest,
      "A summary of your workspace every Monday.",
    ),
  ];

  rsx! {
    div { class: "mx-auto grid w-full max-w-2xl gap-6 p-4 md:p-8",
      div {
        h1 { class: "text-2xl font-semibold", "Settings" }
        p { class: "mt-1 text-sm text-muted-foreground", "Manage your profile and how we contact you." }
      }
      Tabs { value: tab(), on_value_change: move |value: String| tab.set(value),
        TabsList { "aria-label": "Settings sections",
          TabsTrigger { value: "profile", "Profile" }
          TabsTrigger { value: "notifications", "Notifications" }
        }
        TabsContent { value: "profile",
          Card {
            CardHeader {
              CardTitle { "Profile" }
              CardDescription { "How you appear to your team." }
            }
            CardContent { class: "grid gap-4",
              Field { invalid: name_error.is_some(),
                FieldLabel { r#for: "settings-name", "Name" }
                Input {
                  id: "settings-name",
                  autocomplete: "name",
                  value: draft().name,
                  invalid: name_error.is_some(),
                  "aria-describedby": name_error.map(|_| "settings-name-error"),
                  on_value_change: move |value| draft.with_mut(|draft| draft.name = value),
                }
                if let Some(message) = name_error {
                  FieldError { id: "settings-name-error", "{message}" }
                }
              }
              Field { invalid: email_error.is_some(),
                FieldLabel { r#for: "settings-email", "Email" }
                Input {
                  id: "settings-email",
                  r#type: "email",
                  autocomplete: "email",
                  value: draft().email,
                  invalid: email_error.is_some(),
                  "aria-describedby": email_error.map(|_| "settings-email-error").unwrap_or("settings-email-hint"),
                  on_value_change: move |value| draft.with_mut(|draft| draft.email = value),
                }
                if let Some(message) = email_error {
                  FieldError { id: "settings-email-error", "{message}" }
                } else {
                  FieldDescription { id: "settings-email-hint", "Sign-in and notification address." }
                }
              }
              Field {
                FieldLabel { r#for: "settings-bio", "Bio" }
                Textarea {
                  id: "settings-bio",
                  placeholder: "A sentence about you",
                  value: draft().bio,
                  on_value_change: move |value| draft.with_mut(|draft| draft.bio = value),
                }
              }
              Field {
                FieldLabel { r#for: "settings-language", "Language" }
                NativeSelect {
                  id: "settings-language",
                  on_value_change: move |value| draft.with_mut(|draft| draft.language = value),
                  for (code, label) in LANGUAGES {
                    NativeSelectOption { key: "{code}", value: code, selected: draft().language == code, "{label}" }
                  }
                }
              }
            }
          }
        }
        TabsContent { value: "notifications",
          Card {
            CardHeader {
              CardTitle { "Notifications" }
              CardDescription { "Choose the email we send you." }
            }
            CardContent { class: "grid gap-4",
              for (label, id, checked, description) in notifications {
                div { key: "{id}", class: "flex items-start justify-between gap-4",
                  div { class: "grid gap-1",
                    Label { r#for: id, "{label}" }
                    p { id: "{id}-description", class: "text-sm text-muted-foreground", "{description}" }
                  }
                  Switch {
                    id,
                    checked,
                    "aria-describedby": "{id}-description",
                    on_checked_change: move |next| {
                      draft.with_mut(|draft| match id {
                        "settings-updates" => draft.product_updates = next,
                        "settings-security" => draft.security_alerts = next,
                        _ => draft.weekly_digest = next,
                      })
                    },
                  }
                }
              }
            }
          }
        }
      }
      Card {
        CardFooter { class: "justify-end gap-2 pt-6",
          span { class: "me-auto text-sm text-muted-foreground", role: "status",
            if changed { "Unsaved changes" } else { "All changes saved" }
          }
          Button {
            variant: ButtonVariant::Outline,
            disabled: !changed,
            onclick: move |_| {
              draft.set(saved());
              attempted.set(false);
            },
            "Reset"
          }
          Button {
            disabled: !changed,
            onclick: move |_| {
              attempted.set(true);
              let current = draft();
              if current.name.trim().is_empty() || !current.email.contains('@') {
                tab.set("profile".to_string());
                return;
              }
              saved.set(current.clone());
              attempted.set(false);
              if let Some(handler) = on_save {
                handler.call(current);
              }
            },
            "Save"
          }
        }
      }
    }
  }
}
