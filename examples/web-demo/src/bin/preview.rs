use dioxus::prelude::*;
use dioxus_ui::{
  attachment_class, bubble_class, button_class, chart_area_path, chart_area_series_class,
  chart_bar_rects, chart_bar_series_class, chart_class, chart_fallback_rows, chart_line_path,
  chart_line_series_class, chart_view_box, input_group_class, input_otp_class, marker_class,
  message_avatar_class, message_class, message_content_class, message_footer_class,
  message_group_class, message_header_class, message_scroller_class, AttachmentOrientation,
  AttachmentSize, AttachmentState, BubbleAlign, ButtonSize, ButtonVariant, ChartColorToken,
  ChartDomain, ChartPoint, ChartScale, ChartSeries, MarkerVariant, MessageAlign,
  MessageScrollerIntent, UiDensity,
};
use dioxus_ui_preview_states::{preview_lines, PreviewTarget};

const PREVIEW_CSS: Asset = asset!("/assets/preview.css");

fn main() {
  dioxus::launch(PreviewApp);
}

#[component]
fn PreviewApp() -> Element {
  let states = preview_lines(PreviewTarget::Web);
  let primary_button_class = button_class(
    ButtonVariant::Primary,
    ButtonSize::Md,
    UiDensity::Comfortable,
    "",
  );
  let secondary_button_class = button_class(
    ButtonVariant::Secondary,
    ButtonSize::Sm,
    UiDensity::Compact,
    "",
  );
  let form_group_class = input_group_class(false, false, "max-w-sm");
  let otp_class = input_otp_class(false, "max-w-xs");
  let message_group = message_group_class("max-w-2xl");
  let user_message = message_class(MessageAlign::End, "");
  let assistant_message = message_class(MessageAlign::Start, "");
  let user_content = message_content_class(MessageAlign::End, "");
  let assistant_content = message_content_class(MessageAlign::Start, "");
  let attachment = attachment_class(
    AttachmentState::Uploading,
    AttachmentSize::Default,
    AttachmentOrientation::Horizontal,
    "max-w-md",
  );
  let bubble = bubble_class(BubbleAlign::Start, "rounded-xl bg-zinc-100 p-3");
  let marker = marker_class(MarkerVariant::Border, "text-blue-700");
  let scroller = message_scroller_class(MessageScrollerIntent::Hold, "h-64 overflow-auto");
  let chart_series = ChartSeries::new(
    "revenue",
    "Revenue",
    vec![
      ChartPoint::new(0.0, 12.0),
      ChartPoint::new(1.0, 18.0),
      ChartPoint::missing(2.0),
      ChartPoint::new(3.0, 24.0),
    ],
  );
  let chart_x = ChartScale::new(ChartDomain::new(0.0, 3.0), ChartDomain::new(40.0, 600.0));
  let chart_y = ChartScale::new(ChartDomain::new(0.0, 24.0), ChartDomain::new(260.0, 32.0));
  let view_box = chart_view_box(640.0, 300.0);
  let line_path = chart_line_path(&chart_series, chart_x, chart_y);
  let area_path = chart_area_path(&chart_series, chart_x, chart_y, 0.0);
  let bar_rects = chart_bar_rects(&chart_series, chart_x, chart_y, 0.0, 24.0);
  let fallback_rows = chart_fallback_rows(&[chart_series]);

  rsx! {
    document::Title { "dioxus-ui preview" }
    document::Stylesheet { href: PREVIEW_CSS }
    main {
      class: "min-h-screen bg-white text-zinc-950",
      "data-preview-root": "web",
      section {
        class: "mx-auto flex w-full max-w-6xl flex-col gap-6 px-6 py-8",
        "data-preview-panel": "overview",
        header {
          class: "flex flex-col gap-2 border-b border-zinc-200 pb-4",
          h1 { class: "text-2xl font-semibold", "dioxus-ui Web Preview" }
          p {
            class: "max-w-3xl text-sm text-zinc-600",
            "Rendered preview shell for representative component states. This is a component preview surface, not a landing page."
          }
        }
        section {
          class: "grid gap-3 md:grid-cols-2",
          "data-preview-panel": "actions",
          button { class: "{primary_button_class}", "Primary action" }
          button { class: "{secondary_button_class}", "Secondary action" }
        }
        section {
          class: "grid gap-4 lg:grid-cols-2",
          "data-preview-panel": "form",
          article {
            class: "rounded-md border border-zinc-200 p-4",
            h2 { class: "text-sm font-medium", "Form states" }
            div {
              class: "{form_group_class} mt-3",
              span { class: "text-sm text-zinc-500", "https://" }
              input {
                class: "min-w-0 flex-1 bg-transparent px-3 py-2 text-sm outline-none",
                value: "dioxus-ui.dev",
                "aria-label": "domain",
              }
            }
            div {
              class: "{otp_class} mt-4 flex gap-2",
              "aria-label": "one-time code",
              for digit in ["1", "2", "3", "", "", ""] {
                span {
                  class: "flex h-10 w-10 items-center justify-center rounded-md border border-zinc-200 text-sm",
                  "{digit}"
                }
              }
            }
          }
          article {
            class: "rounded-md border border-zinc-200 p-4",
            "data-preview-panel": "overlay-open",
            h2 { class: "text-sm font-medium", "Open overlay state" }
            div {
              class: "relative mt-3 min-h-36 rounded-md bg-zinc-50 p-4",
              button { class: "{secondary_button_class}", "Open popover" }
              div {
                class: "absolute left-4 top-16 z-10 w-64 rounded-md border border-zinc-200 bg-white p-3 text-sm shadow-md",
                role: "dialog",
                "aria-label": "Preview popover",
                p { class: "font-medium", "Preview popover" }
                p { class: "mt-1 text-zinc-600", "Open-state target for screenshot verification." }
              }
            }
          }
        }
        section {
          class: "rounded-md border border-zinc-200 p-4",
          "data-preview-panel": "message",
          h2 { class: "text-sm font-medium", "Message states" }
          div {
            class: "{scroller} mt-3 rounded-md bg-zinc-50 p-3",
            div {
              class: "{message_group}",
              div {
                class: "{user_message}",
                div { class: "{message_avatar_class(\"bg-blue-100\")}", "U" }
                div {
                  class: "{user_content}",
                  div { class: "{message_header_class(\"\")}", "User - just now" }
                  div { class: "rounded-xl bg-blue-600 px-3 py-2 text-sm text-white", "Can we preview the expanded parity states?" }
                  div { class: "{message_footer_class(\"\")}", "sent" }
                }
              }
              div {
                class: "{assistant_message}",
                div { class: "{message_avatar_class(\"bg-zinc-200\")}", "A" }
                div {
                  class: "{assistant_content}",
                  div { class: "{message_header_class(\"\")}", "Assistant - preview" }
                  div { class: "{bubble}", "Yes. Message, attachment, marker, and scroller states are visible here." }
                  div { class: "{attachment}", "design-notes.md - uploading" }
                  div { class: "{marker}", "Cited preview marker" }
                }
              }
            }
          }
        }
        section {
          class: "{chart_class(\"rounded-md border border-zinc-200 p-4\")}",
          "data-preview-panel": "chart",
          h2 { class: "text-sm font-medium", "Chart states" }
          p { class: "mt-1 text-sm text-zinc-600", "Line, area, bar, legend, and fallback table targets." }
          div {
            class: "mt-3 flex flex-wrap gap-3 text-sm text-zinc-600",
            span { "Revenue" }
            span { "Area" }
            span { "Bars" }
          }
          svg {
            class: "mt-4 h-auto w-full overflow-visible",
            role: "img",
            view_box: "{view_box}",
            "aria-label": "Revenue preview chart",
            path {
              class: "{chart_area_series_class(ChartColorToken::Primary, \"opacity-15\")}",
              d: "{area_path}",
            }
            path {
              class: "{chart_line_series_class(ChartColorToken::Primary, \"stroke-2\")}",
              d: "{line_path}",
            }
            g {
              class: "{chart_bar_series_class(ChartColorToken::Secondary, \"opacity-60\")}",
              for rect in bar_rects {
                rect {
                  x: "{rect.x}",
                  y: "{rect.y}",
                  width: "{rect.width}",
                  height: "{rect.height}",
                  rx: "2",
                }
              }
            }
          }
          table {
            class: "mt-4 w-full text-left text-sm",
            caption { class: "sr-only", "Chart fallback data" }
            tbody {
              for row in fallback_rows {
                tr {
                  td { class: "border-t border-zinc-200 py-1", "{row.series_label}" }
                  td { class: "border-t border-zinc-200 py-1", "{row.x_label}" }
                  td { class: "border-t border-zinc-200 py-1", "{row.y_label}" }
                }
              }
            }
          }
        }
        section {
          class: "grid gap-3 md:grid-cols-2 xl:grid-cols-3",
          "data-preview-panel": "inventory",
          for state in states {
            article {
              key: "{state.label}",
              class: "rounded-md border border-zinc-200 bg-white p-4 shadow-sm",
              "data-preview-state": "{state.label}",
              h2 { class: "text-sm font-medium text-zinc-950", "{state.label}" }
              code {
                class: "mt-3 block overflow-hidden text-ellipsis whitespace-nowrap rounded bg-zinc-50 px-2 py-1 text-xs text-zinc-700",
                title: "{state.value}",
                "{state.value}"
              }
            }
          }
        }
      }
    }
  }
}
