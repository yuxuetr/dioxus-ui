//! Blocks (RFC 0073), compiled from the sources `dxui add` copies.

use dioxus::prelude::*;

#[path = "../../crates/dioxus-shadcn-cli/blocks/dashboard.rs"]
mod dashboard;
#[path = "../../crates/dioxus-shadcn-cli/blocks/login.rs"]
mod login;
#[path = "../../crates/dioxus-shadcn-cli/blocks/settings.rs"]
mod settings;
#[path = "../../crates/dioxus-shadcn-cli/blocks/signup.rs"]
mod signup;

pub struct Block {
  pub slug: &'static str,
  pub title: &'static str,
  pub description: &'static str,
  pub source: &'static str,
  pub render: fn() -> Element,
}

pub const BLOCKS: &[Block] = &[
  Block {
    slug: "dashboard",
    title: "Dashboard",
    description: "App shell with an off-canvas sidebar, metrics, a chart, and an orders table.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/dashboard.rs"),
    render: || rsx! { dashboard::DashboardBlock {} },
  },
  Block {
    slug: "login",
    title: "Login",
    description: "Sign-in page with checked email and password fields.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/login.rs"),
    render: || rsx! { login::LoginBlock {} },
  },
  Block {
    slug: "settings",
    title: "Settings",
    description: "Settings page with profile and notification tabs, checked fields, and save and reset.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/settings.rs"),
    render: || rsx! { settings::SettingsBlock {} },
  },
  Block {
    slug: "signup",
    title: "Signup",
    description: "Account creation page with a password strength meter and checked fields.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/signup.rs"),
    render: || rsx! { signup::SignupBlock {} },
  },
];
