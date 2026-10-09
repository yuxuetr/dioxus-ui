use dioxus::prelude::*;

use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardHeader, CardTitle};
use crate::components::ui::field::{Field, FieldError, FieldLabel};
use crate::components::ui::input::Input;
use crate::components::ui::label::Label;
use crate::components::ui::native_select::{NativeSelect, NativeSelectOption};
use crate::components::ui::radio_group::{RadioGroup, RadioGroupItem};
use crate::components::ui::separator::Separator;

/// What placing an order submits. The card details stay out of it: send them
/// to your payment provider from the form, not through your server.
#[derive(Clone, Debug, PartialEq)]
pub struct Order {
  pub email: String,
  pub name: String,
  pub address: String,
  pub city: String,
  pub postal_code: String,
  pub country: String,
  pub shipping: String,
  pub total_cents: u32,
}

const ITEMS: [(&str, u32, u32); 2] = [("Linen shirt", 1, 4_800), ("Canvas tote", 2, 1_800)];
const COUNTRIES: [(&str, &str); 4] =
  [("", "Choose a country"), ("CA", "Canada"), ("DE", "Germany"), ("JP", "Japan")];
const SHIPPING: [(&str, &str, u32); 2] =
  [("standard", "Standard, 5 to 7 days", 0), ("express", "Express, 1 to 2 days", 1_200)];

fn money(cents: u32) -> String {
  format!("${}.{:02}", cents / 100, cents % 100)
}

fn digits(text: &str) -> String {
  text.chars().filter(|c| !c.is_whitespace()).collect()
}

/// A checkout page: contact, shipping address, shipping method, and card
/// fields beside an order summary whose total follows the shipping choice.
/// Pay checks every field; a valid order calls `on_place_order` and shows a
/// confirmation. Replace `ITEMS`, `COUNTRIES`, and `SHIPPING` with your own.
#[component]
pub fn CheckoutBlock(#[props(default)] on_place_order: Option<EventHandler<Order>>) -> Element {
  let mut email = use_signal(String::new);
  let mut name = use_signal(String::new);
  let mut address = use_signal(String::new);
  let mut city = use_signal(String::new);
  let mut postal_code = use_signal(String::new);
  let mut country = use_signal(String::new);
  let mut shipping = use_signal(|| "standard".to_string());
  let mut card = use_signal(String::new);
  let mut expiry = use_signal(String::new);
  let mut cvc = use_signal(String::new);
  let mut submitted = use_signal(|| false);
  let mut placed = use_signal(|| false);

  let subtotal: u32 = ITEMS.iter().map(|(_, quantity, cents)| quantity * cents).sum();
  let shipping_cents =
    SHIPPING.iter().find(|(id, _, _)| *id == shipping()).map_or(0, |(_, _, cents)| *cents);
  let total = subtotal + shipping_cents;
  let card_digits = digits(&card());
  let expiry_ok = {
    let text = expiry();
    let parts: Vec<&str> = text.split('/').collect();
    matches!(parts.as_slice(), [month, year]
      if month.len() == 2 && year.len() == 2
        && month.parse::<u8>().is_ok_and(|month| (1..=12).contains(&month))
        && year.chars().all(|c| c.is_ascii_digit()))
  };
  let check = |failed: bool, message: &'static str| (submitted() && failed).then_some(message);
  let email_error = check(!email().contains('@'), "Enter your email address.");
  let name_error = check(name().trim().is_empty(), "Enter the recipient's name.");
  let address_error = check(address().trim().is_empty(), "Enter a street address.");
  let city_error = check(city().trim().is_empty(), "Enter a city.");
  let postal_error = check(postal_code().trim().is_empty(), "Enter a postal code.");
  let country_error = check(country().is_empty(), "Choose a country.");
  let card_error = check(
    !(12..=19).contains(&card_digits.len()) || !card_digits.chars().all(|c| c.is_ascii_digit()),
    "Enter the 12 to 19 digit card number.",
  );
  let expiry_error = check(!expiry_ok, "Use MM/YY.");
  let cvc_error = check(
    !(3..=4).contains(&cvc().len()) || !cvc().chars().all(|c| c.is_ascii_digit()),
    "Enter the 3 or 4 digit code.",
  );

  rsx! {
    div { class: "min-h-screen bg-muted/40 px-4 py-10 text-foreground",
      div { class: "mx-auto grid max-w-5xl gap-6 lg:grid-cols-[1fr_22rem]",
        if placed() {
          div { class: "lg:col-span-2",
            Card {
              CardContent { class: "grid gap-2 pt-6 text-center",
                h1 { class: "text-2xl font-semibold", "Order placed" }
                p { role: "status", class: "text-muted-foreground",
                  "Thanks, {name().trim()}. We sent the receipt to {email()}."
                }
              }
            }
          }
        } else {
          form {
            class: "grid content-start gap-6",
            "aria-label": "Checkout",
            novalidate: true,
            onsubmit: move |event| {
              event.prevent_default();
              submitted.set(true);
              let card_digits = digits(&card());
              let valid = email().contains('@')
                && !name().trim().is_empty()
                && !address().trim().is_empty()
                && !city().trim().is_empty()
                && !postal_code().trim().is_empty()
                && !country().is_empty()
                && (12..=19).contains(&card_digits.len())
                && card_digits.chars().all(|c| c.is_ascii_digit())
                && expiry_ok
                && (3..=4).contains(&cvc().len())
                && cvc().chars().all(|c| c.is_ascii_digit());
              if !valid {
                return;
              }
              placed.set(true);
              if let Some(handler) = on_place_order {
                handler.call(Order {
                  email: email(),
                  name: name().trim().to_string(),
                  address: address().trim().to_string(),
                  city: city().trim().to_string(),
                  postal_code: postal_code().trim().to_string(),
                  country: country(),
                  shipping: shipping(),
                  total_cents: total,
                });
              }
            },
            Card {
              CardHeader {
                CardTitle {
                  h2 { "Contact" }
                }
              }
              CardContent {
                Field { invalid: email_error.is_some(),
                  FieldLabel { r#for: "checkout-email", "Email" }
                  Input {
                    id: "checkout-email",
                    r#type: "email",
                    autocomplete: "email",
                    value: email(),
                    invalid: email_error.is_some(),
                    "aria-describedby": email_error.map(|_| "checkout-email-error"),
                    on_value_change: move |value| email.set(value),
                  }
                  if let Some(message) = email_error {
                    FieldError { id: "checkout-email-error", "{message}" }
                  }
                }
              }
            }
            Card {
              CardHeader {
                CardTitle {
                  h2 { "Shipping address" }
                }
              }
              CardContent { class: "grid gap-4 sm:grid-cols-2",
                Field { class: "sm:col-span-2", invalid: name_error.is_some(),
                  FieldLabel { r#for: "checkout-name", "Full name" }
                  Input {
                    id: "checkout-name",
                    autocomplete: "shipping name",
                    value: name(),
                    invalid: name_error.is_some(),
                    "aria-describedby": name_error.map(|_| "checkout-name-error"),
                    on_value_change: move |value| name.set(value),
                  }
                  if let Some(message) = name_error {
                    FieldError { id: "checkout-name-error", "{message}" }
                  }
                }
                Field { class: "sm:col-span-2", invalid: address_error.is_some(),
                  FieldLabel { r#for: "checkout-address", "Street address" }
                  Input {
                    id: "checkout-address",
                    autocomplete: "shipping street-address",
                    value: address(),
                    invalid: address_error.is_some(),
                    "aria-describedby": address_error.map(|_| "checkout-address-error"),
                    on_value_change: move |value| address.set(value),
                  }
                  if let Some(message) = address_error {
                    FieldError { id: "checkout-address-error", "{message}" }
                  }
                }
                Field { invalid: city_error.is_some(),
                  FieldLabel { r#for: "checkout-city", "City" }
                  Input {
                    id: "checkout-city",
                    autocomplete: "shipping address-level2",
                    value: city(),
                    invalid: city_error.is_some(),
                    "aria-describedby": city_error.map(|_| "checkout-city-error"),
                    on_value_change: move |value| city.set(value),
                  }
                  if let Some(message) = city_error {
                    FieldError { id: "checkout-city-error", "{message}" }
                  }
                }
                Field { invalid: postal_error.is_some(),
                  FieldLabel { r#for: "checkout-postal", "Postal code" }
                  Input {
                    id: "checkout-postal",
                    autocomplete: "shipping postal-code",
                    value: postal_code(),
                    invalid: postal_error.is_some(),
                    "aria-describedby": postal_error.map(|_| "checkout-postal-error"),
                    on_value_change: move |value| postal_code.set(value),
                  }
                  if let Some(message) = postal_error {
                    FieldError { id: "checkout-postal-error", "{message}" }
                  }
                }
                Field { class: "sm:col-span-2", invalid: country_error.is_some(),
                  FieldLabel { r#for: "checkout-country", "Country" }
                  NativeSelect {
                    id: "checkout-country",
                    autocomplete: "shipping country",
                    invalid: country_error.is_some(),
                    "aria-describedby": country_error.map(|_| "checkout-country-error"),
                    on_value_change: move |value| country.set(value),
                    for (code, label) in COUNTRIES {
                      NativeSelectOption { key: "{code}", value: code, selected: country() == code, "{label}" }
                    }
                  }
                  if let Some(message) = country_error {
                    FieldError { id: "checkout-country-error", "{message}" }
                  }
                }
              }
            }
            Card {
              CardHeader {
                CardTitle {
                  h2 { id: "checkout-shipping-label", "Shipping method" }
                }
              }
              CardContent {
                RadioGroup {
                  class: "grid gap-3",
                  "aria-labelledby": "checkout-shipping-label",
                  value: shipping(),
                  on_value_change: move |value| shipping.set(value),
                  for (id, label, cents) in SHIPPING {
                    div { key: "{id}", class: "flex items-center gap-2",
                      RadioGroupItem { id: "checkout-shipping-{id}", value: id }
                      Label { class: "flex-1", r#for: "checkout-shipping-{id}", "{label}" }
                      span { class: "text-sm text-muted-foreground",
                        if cents == 0 { "Free" } else { "{money(cents)}" }
                      }
                    }
                  }
                }
              }
            }
            Card {
              CardHeader {
                CardTitle {
                  h2 { "Payment" }
                }
              }
              CardContent { class: "grid gap-4 sm:grid-cols-[1fr_6rem_6rem]",
                Field { invalid: card_error.is_some(),
                  FieldLabel { r#for: "checkout-card", "Card number" }
                  Input {
                    id: "checkout-card",
                    autocomplete: "cc-number",
                    inputmode: "numeric",
                    value: card(),
                    invalid: card_error.is_some(),
                    "aria-describedby": card_error.map(|_| "checkout-card-error"),
                    on_value_change: move |value| card.set(value),
                  }
                  if let Some(message) = card_error {
                    FieldError { id: "checkout-card-error", "{message}" }
                  }
                }
                Field { invalid: expiry_error.is_some(),
                  FieldLabel { r#for: "checkout-expiry", "Expiry" }
                  Input {
                    id: "checkout-expiry",
                    autocomplete: "cc-exp",
                    placeholder: "MM/YY",
                    value: expiry(),
                    invalid: expiry_error.is_some(),
                    "aria-describedby": expiry_error.map(|_| "checkout-expiry-error"),
                    on_value_change: move |value| expiry.set(value),
                  }
                  if let Some(message) = expiry_error {
                    FieldError { id: "checkout-expiry-error", "{message}" }
                  }
                }
                Field { invalid: cvc_error.is_some(),
                  FieldLabel { r#for: "checkout-cvc", "CVC" }
                  Input {
                    id: "checkout-cvc",
                    autocomplete: "cc-csc",
                    inputmode: "numeric",
                    value: cvc(),
                    invalid: cvc_error.is_some(),
                    "aria-describedby": cvc_error.map(|_| "checkout-cvc-error"),
                    on_value_change: move |value| cvc.set(value),
                  }
                  if let Some(message) = cvc_error {
                    FieldError { id: "checkout-cvc-error", "{message}" }
                  }
                }
              }
            }
            Button { class: "w-full", r#type: "submit", "Pay {money(total)}" }
          }
          aside { "aria-label": "Order summary", class: "lg:sticky lg:top-6 lg:self-start",
            Card {
              CardHeader {
                CardTitle {
                  h2 { "Order summary" }
                }
              }
              CardContent { class: "grid gap-3 text-sm",
                ul { class: "grid gap-2",
                  for (item, quantity, cents) in ITEMS {
                    li { key: "{item}", class: "flex justify-between gap-2",
                      span { "{item} \u{d7} {quantity}" }
                      span { "{money(quantity * cents)}" }
                    }
                  }
                }
                Separator { decorative: true }
                dl { class: "grid grid-cols-[1fr_auto] gap-y-1",
                  dt { class: "text-muted-foreground", "Subtotal" }
                  dd { "{money(subtotal)}" }
                  dt { class: "text-muted-foreground", "Shipping" }
                  dd { if shipping_cents == 0 { "Free" } else { "{money(shipping_cents)}" } }
                  dt { class: "font-medium", "Total" }
                  dd { class: "font-medium", "data-checkout-total": "", "{money(total)}" }
                }
              }
            }
          }
        }
      }
    }
  }
}
