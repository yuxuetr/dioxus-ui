use dioxus::prelude::*;
use dioxus_shadcn::{
  Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
  BreadcrumbSeparator,
};

#[component]
pub fn Demo() -> Element {
  rsx! {
    Breadcrumb {
      BreadcrumbList {
        BreadcrumbItem { BreadcrumbLink { href: "#home", "Home" } }
        BreadcrumbSeparator { "/" }
        BreadcrumbItem { BreadcrumbEllipsis {} }
        BreadcrumbSeparator { "/" }
        BreadcrumbItem { BreadcrumbLink { href: "#components", "Components" } }
        BreadcrumbSeparator { "/" }
        BreadcrumbItem { BreadcrumbPage { "Breadcrumb" } }
      }
    }
  }
}
