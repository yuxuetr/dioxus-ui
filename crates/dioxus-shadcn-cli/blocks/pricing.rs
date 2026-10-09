use dioxus::prelude::*;

use crate::components::ui::badge::{Badge, BadgeVariant};
use crate::components::ui::button::{Button, ButtonVariant};
use crate::components::ui::card::{
  Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
};
use crate::components::ui::separator::Separator;
use crate::components::ui::toggle_group::{ToggleGroup, ToggleGroupItem};

/// How often a plan is billed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Billing {
  Monthly,
  Yearly,
}

/// What choosing a plan reports.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanChoice {
  pub plan: String,
  pub billing: Billing,
}

struct Plan {
  id: &'static str,
  name: &'static str,
  description: &'static str,
  monthly: u32,
  yearly: u32,
  features: &'static [&'static str],
  featured: bool,
}

const PLANS: &[Plan] = &[
  Plan {
    id: "starter",
    name: "Starter",
    description: "For trying things out on your own.",
    monthly: 0,
    yearly: 0,
    features: &["1 project", "Community support", "1 GB storage"],
    featured: false,
  },
  Plan {
    id: "pro",
    name: "Pro",
    description: "For people shipping real work.",
    monthly: 12,
    yearly: 120,
    features: &["Unlimited projects", "Email support", "50 GB storage", "Custom domains"],
    featured: true,
  },
  Plan {
    id: "team",
    name: "Team",
    description: "For groups that share projects.",
    monthly: 29,
    yearly: 290,
    features: &[
      "Everything in Pro",
      "Shared workspaces",
      "Roles and permissions",
      "Priority support",
    ],
    featured: false,
  },
];

/// A pricing page: three plans, a Monthly or Yearly switch that changes every
/// price, a highlighted plan, and a feature list per plan. Choosing a plan
/// calls `on_choose`; replace `PLANS` with your own.
#[component]
pub fn PricingBlock(#[props(default)] on_choose: Option<EventHandler<PlanChoice>>) -> Element {
  let mut billing = use_signal(|| Billing::Monthly);
  let period = match billing() {
    Billing::Monthly => "monthly",
    Billing::Yearly => "yearly",
  };

  rsx! {
    div { class: "min-h-screen bg-background px-4 py-16",
      div { class: "mx-auto grid max-w-5xl gap-10",
        div { class: "grid justify-items-center gap-4 text-center",
          h1 { class: "text-3xl font-semibold tracking-tight sm:text-4xl", "Simple pricing" }
          p { class: "max-w-md text-muted-foreground",
            "Start free and upgrade when you need more. Yearly plans cost two months less."
          }
          ToggleGroup {
            "aria-label": "Billing period",
            value: period,
            // Releasing the pressed item reports ""; keep the current period.
            on_value_change: move |value: String| match value.as_str() {
              "monthly" => billing.set(Billing::Monthly),
              "yearly" => billing.set(Billing::Yearly),
              _ => {}
            },
            ToggleGroupItem { value: "monthly", "Monthly" }
            ToggleGroupItem { value: "yearly", "Yearly" }
          }
        }
        div { class: "grid gap-6 md:grid-cols-3",
          for plan in PLANS {
            section { key: "{plan.id}", class: "grid", "aria-labelledby": "pricing-{plan.id}",
            Card {
              class: if plan.featured { "relative border-primary shadow-md" } else { "relative" },
              CardHeader {
                if plan.featured {
                  Badge { class: "absolute -top-3 right-4", variant: BadgeVariant::Default, "Most popular" }
                }
                CardTitle {
                  h2 { id: "pricing-{plan.id}", "{plan.name}" }
                }
                CardDescription { "{plan.description}" }
              }
              CardContent { class: "grid gap-4",
                p { class: "flex items-baseline gap-1",
                  span { class: "text-4xl font-semibold tracking-tight",
                    match billing() {
                      Billing::Monthly => format!("${}", plan.monthly),
                      Billing::Yearly => format!("${}", plan.yearly),
                    }
                  }
                  span { class: "text-sm text-muted-foreground",
                    match billing() {
                      Billing::Monthly => "/month",
                      Billing::Yearly => "/year",
                    }
                  }
                }
                Separator { decorative: true }
                ul { class: "grid gap-2 text-sm",
                  for feature in plan.features.iter() {
                    li { key: "{feature}", class: "flex items-center gap-2",
                      svg {
                        class: "size-4 shrink-0 text-primary",
                        "aria-hidden": "true",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M20 6 9 17l-5-5" }
                      }
                      "{feature}"
                    }
                  }
                }
              }
              CardFooter {
                Button {
                  class: "w-full",
                  variant: if plan.featured { ButtonVariant::Primary } else { ButtonVariant::Outline },
                  onclick: move |_| {
                    if let Some(handler) = on_choose {
                      handler.call(PlanChoice { plan: plan.id.to_string(), billing: billing() });
                    }
                  },
                  "Choose {plan.name}"
                }
              }
            }
            }
          }
        }
      }
    }
  }
}
