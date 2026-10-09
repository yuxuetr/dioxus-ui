//! Blocks (RFC 0073), compiled from the sources `dxui add` copies.

use dioxus::prelude::*;

#[path = "../../crates/dioxus-shadcn-cli/blocks/chat.rs"]
mod chat;
#[path = "../../crates/dioxus-shadcn-cli/blocks/dashboard.rs"]
mod dashboard;
#[path = "../../crates/dioxus-shadcn-cli/blocks/files.rs"]
mod files;
#[path = "../../crates/dioxus-shadcn-cli/blocks/inbox.rs"]
mod inbox;
#[path = "../../crates/dioxus-shadcn-cli/blocks/landing.rs"]
mod landing;
#[path = "../../crates/dioxus-shadcn-cli/blocks/login.rs"]
mod login;
#[path = "../../crates/dioxus-shadcn-cli/blocks/pricing.rs"]
mod pricing;
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
    slug: "chat",
    title: "Chat",
    description: "Chat screen with conversations, messages, attachments, and a composer that takes dropped files.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/chat.rs"),
    render: || rsx! { chat::ChatBlock {} },
  },
  Block {
    slug: "dashboard",
    title: "Dashboard",
    description: "App shell with an off-canvas sidebar, metrics, a chart, and an orders table.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/dashboard.rs"),
    render: || rsx! { dashboard::DashboardBlock {} },
  },
  Block {
    slug: "files",
    title: "Files",
    description: "File manager with a folder tree, breadcrumbs, an upload area that takes dropped files, and a file table with a context menu.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/files.rs"),
    render: || rsx! { files::FilesBlock {} },
  },
  Block {
    slug: "inbox",
    title: "Inbox",
    description: "Mail screen with folders, a searchable message list, and a reading pane.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/inbox.rs"),
    render: || rsx! { inbox::InboxBlock {} },
  },
  Block {
    slug: "landing",
    title: "Landing",
    description: "Product page with a hero, features, a testimonial, an early-access form, and a footer.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/landing.rs"),
    render: || rsx! { landing::LandingBlock {} },
  },
  Block {
    slug: "login",
    title: "Login",
    description: "Sign-in page with checked email and password fields.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/login.rs"),
    render: || rsx! { login::LoginBlock {} },
  },
  Block {
    slug: "pricing",
    title: "Pricing",
    description: "Pricing page with three plans and a monthly or yearly switch.",
    source: include_str!("../../crates/dioxus-shadcn-cli/blocks/pricing.rs"),
    render: || rsx! { pricing::PricingBlock {} },
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
