use dioxus::prelude::*;
use dioxus_ui_core::classes;

pub const TABLE_CONTAINER_BASE_CLASS: &str = "relative w-full overflow-auto";
pub const TABLE_BASE_CLASS: &str = "w-full caption-bottom text-sm";
pub const TABLE_HEADER_BASE_CLASS: &str = "[&_tr]:border-b";
pub const TABLE_BODY_BASE_CLASS: &str = "[&_tr:last-child]:border-0";
pub const TABLE_FOOTER_BASE_CLASS: &str = "border-t bg-muted font-medium [&>tr]:last:border-b-0";
pub const TABLE_ROW_BASE_CLASS: &str = "border-b transition-colors hover:bg-muted";
pub const TABLE_HEAD_BASE_CLASS: &str =
  "h-12 px-4 text-left align-middle font-medium text-muted-foreground";
pub const TABLE_CELL_BASE_CLASS: &str = "p-4 align-middle";
pub const TABLE_CAPTION_BASE_CLASS: &str = "mt-4 text-sm text-muted-foreground";

pub fn table_container_class(class: &str) -> String {
  classes([Some(TABLE_CONTAINER_BASE_CLASS), Some(class)])
}

pub fn table_class(class: &str) -> String {
  classes([Some(TABLE_BASE_CLASS), Some(class)])
}

pub fn table_header_class(class: &str) -> String {
  classes([Some(TABLE_HEADER_BASE_CLASS), Some(class)])
}

pub fn table_body_class(class: &str) -> String {
  classes([Some(TABLE_BODY_BASE_CLASS), Some(class)])
}

pub fn table_footer_class(class: &str) -> String {
  classes([Some(TABLE_FOOTER_BASE_CLASS), Some(class)])
}

pub fn table_row_class(class: &str) -> String {
  classes([Some(TABLE_ROW_BASE_CLASS), Some(class)])
}

pub fn table_head_class(class: &str) -> String {
  classes([Some(TABLE_HEAD_BASE_CLASS), Some(class)])
}

pub fn table_cell_class(class: &str) -> String {
  classes([Some(TABLE_CELL_BASE_CLASS), Some(class)])
}

pub fn table_caption_class(class: &str) -> String {
  classes([Some(TABLE_CAPTION_BASE_CLASS), Some(class)])
}

#[component]
pub fn Table(#[props(default)] class: String, children: Element) -> Element {
  let container_class = table_container_class("");
  let class = table_class(&class);

  rsx! {
    div {
      class: container_class,
      table {
        class,
        {children}
      }
    }
  }
}

#[component]
pub fn TableHeader(#[props(default)] class: String, children: Element) -> Element {
  let class = table_header_class(&class);

  rsx! {
    thead {
      class,
      {children}
    }
  }
}

#[component]
pub fn TableBody(#[props(default)] class: String, children: Element) -> Element {
  let class = table_body_class(&class);

  rsx! {
    tbody {
      class,
      {children}
    }
  }
}

#[component]
pub fn TableFooter(#[props(default)] class: String, children: Element) -> Element {
  let class = table_footer_class(&class);

  rsx! {
    tfoot {
      class,
      {children}
    }
  }
}

#[component]
pub fn TableRow(#[props(default)] class: String, children: Element) -> Element {
  let class = table_row_class(&class);

  rsx! {
    tr {
      class,
      {children}
    }
  }
}

#[component]
pub fn TableHead(#[props(default)] class: String, children: Element) -> Element {
  let class = table_head_class(&class);

  rsx! {
    th {
      class,
      {children}
    }
  }
}

#[component]
pub fn TableCell(#[props(default)] class: String, children: Element) -> Element {
  let class = table_cell_class(&class);

  rsx! {
    td {
      class,
      {children}
    }
  }
}

#[component]
pub fn TableCaption(#[props(default)] class: String, children: Element) -> Element {
  let class = table_caption_class(&class);

  rsx! {
    caption {
      class,
      {children}
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn table_class_appends_user_class() {
    let actual = table_class("min-w-lg");

    assert!(actual.contains(TABLE_BASE_CLASS));
    assert!(actual.ends_with("min-w-lg"));
  }

  #[test]
  fn table_row_class_appends_user_class() {
    let actual = table_row_class("data-[selected=true]:bg-muted");

    assert!(actual.contains(TABLE_ROW_BASE_CLASS));
    assert!(actual.ends_with("data-[selected=true]:bg-muted"));
  }
}
