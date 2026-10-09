# Blocks

Blocks are whole screens built from the components (see
[RFC 0073](../rfcs/0073-blocks.md)). `dxui add <block>` copies the block to
`src/blocks/`, declares it in `src/blocks/mod.rs`, and adds the components it
uses to `src/components/ui/`. Declare `mod blocks;` once, next to
`mod components;`, and render the block's component:

```rust
mod blocks;
mod components;

fn App() -> Element {
  rsx! { blocks::login::LoginBlock { on_sign_in: move |sign_in| { /* authenticate */ } } }
}
```

A block holds its own sample data and state, so it renders as soon as it is
added; it is your code from then on. `dxui list blocks` prints the blocks.

| Block | Command | Screen |
| --- | --- | --- |
| [Dashboard](dashboard.md) | `dxui add dashboard` | App shell with sidebar, metrics, chart, and orders table |
| [Login](login.md) | `dxui add login` | Sign-in form with checked fields |
| [Pricing](pricing.md) | `dxui add pricing` | Three plans with a monthly or yearly switch |
| [Settings](settings.md) | `dxui add settings` | Profile and notification settings with save and reset |
| [Signup](signup.md) | `dxui add signup` | Account creation with a password strength meter and checked fields |
