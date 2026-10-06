use dioxus::prelude::*;
use dioxus_shadcn_core::{classes, merge_classes};

pub const TYPOGRAPHY_PROSE_BASE_CLASS: &str = "max-w-none text-foreground";
pub const TYPOGRAPHY_H1_BASE_CLASS: &str =
  "scroll-m-20 text-4xl font-extrabold tracking-normal text-foreground";
pub const TYPOGRAPHY_H2_BASE_CLASS: &str =
  "scroll-m-20 border-b border-border pb-2 text-3xl font-semibold tracking-normal text-foreground";
pub const TYPOGRAPHY_H3_BASE_CLASS: &str =
  "scroll-m-20 text-2xl font-semibold tracking-normal text-foreground";
pub const TYPOGRAPHY_P_BASE_CLASS: &str = "leading-7 text-foreground";
pub const TYPOGRAPHY_LEAD_BASE_CLASS: &str = "text-xl text-muted-foreground";
pub const TYPOGRAPHY_MUTED_BASE_CLASS: &str = "text-sm text-muted-foreground";
pub const TYPOGRAPHY_BLOCKQUOTE_BASE_CLASS: &str =
  "mt-6 border-l-2 border-border pl-6 italic text-foreground";
pub const TYPOGRAPHY_INLINE_CODE_BASE_CLASS: &str =
  "relative rounded bg-muted px-1.5 py-0.5 font-mono text-sm text-foreground";

pub fn typography_prose_class(class: &str) -> String {
  merge_classes(classes([Some(TYPOGRAPHY_PROSE_BASE_CLASS)]), class)
}

pub fn typography_h1_class(class: &str) -> String {
  merge_classes(classes([Some(TYPOGRAPHY_H1_BASE_CLASS)]), class)
}

pub fn typography_h2_class(class: &str) -> String {
  merge_classes(classes([Some(TYPOGRAPHY_H2_BASE_CLASS)]), class)
}

pub fn typography_h3_class(class: &str) -> String {
  merge_classes(classes([Some(TYPOGRAPHY_H3_BASE_CLASS)]), class)
}

pub fn typography_p_class(class: &str) -> String {
  merge_classes(classes([Some(TYPOGRAPHY_P_BASE_CLASS)]), class)
}

pub fn typography_lead_class(class: &str) -> String {
  merge_classes(classes([Some(TYPOGRAPHY_LEAD_BASE_CLASS)]), class)
}

pub fn typography_muted_class(class: &str) -> String {
  merge_classes(classes([Some(TYPOGRAPHY_MUTED_BASE_CLASS)]), class)
}

pub fn typography_blockquote_class(class: &str) -> String {
  merge_classes(classes([Some(TYPOGRAPHY_BLOCKQUOTE_BASE_CLASS)]), class)
}

pub fn typography_inline_code_class(class: &str) -> String {
  merge_classes(classes([Some(TYPOGRAPHY_INLINE_CODE_BASE_CLASS)]), class)
}

#[component]
pub fn TypographyProse(#[props(default)] class: String, children: Element) -> Element {
  let class = typography_prose_class(&class);
  rsx! { div { class, {children} } }
}

#[component]
pub fn TypographyH1(#[props(default)] class: String, children: Element) -> Element {
  let class = typography_h1_class(&class);
  rsx! { h1 { class, {children} } }
}

#[component]
pub fn TypographyH2(#[props(default)] class: String, children: Element) -> Element {
  let class = typography_h2_class(&class);
  rsx! { h2 { class, {children} } }
}

#[component]
pub fn TypographyH3(#[props(default)] class: String, children: Element) -> Element {
  let class = typography_h3_class(&class);
  rsx! { h3 { class, {children} } }
}

#[component]
pub fn TypographyP(#[props(default)] class: String, children: Element) -> Element {
  let class = typography_p_class(&class);
  rsx! { p { class, {children} } }
}

#[component]
pub fn TypographyLead(#[props(default)] class: String, children: Element) -> Element {
  let class = typography_lead_class(&class);
  rsx! { p { class, {children} } }
}

#[component]
pub fn TypographyMuted(#[props(default)] class: String, children: Element) -> Element {
  let class = typography_muted_class(&class);
  rsx! { p { class, {children} } }
}

#[component]
pub fn TypographyBlockquote(#[props(default)] class: String, children: Element) -> Element {
  let class = typography_blockquote_class(&class);
  rsx! { blockquote { class, {children} } }
}

#[component]
pub fn TypographyInlineCode(#[props(default)] class: String, children: Element) -> Element {
  let class = typography_inline_code_class(&class);
  rsx! { code { class, {children} } }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn typography_classes_append_user_class() {
    let actual = typography_h2_class("mt-8");

    assert!(actual.contains(TYPOGRAPHY_H2_BASE_CLASS));
    assert!(actual.ends_with("mt-8"));
  }
}
