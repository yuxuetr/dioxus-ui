use dioxus::prelude::*;
use dioxus_ui::{
  AlertDialogAction, AlertDialogActionVariant, AlertDialogCancel, AlertDialogContent,
  AlertDialogDescription, AlertDialogOverlay, AlertDialogTitle, AttachmentOrientation,
  AttachmentSize, AttachmentState, BubbleAlign, ButtonGroupOrientation, ButtonSize, ButtonVariant,
  ChartColorToken, ChartDomain, ChartPoint, ChartScale, ChartSeries, DialogClose, DialogContent,
  DialogDescription, DialogOverlay, DialogTitle, DismissBehavior, MarkerVariant, MessageAlign,
  MessageScrollerIntent, MessageScrollerMetrics, TextDirection, UiDensity, attachment_class,
  bubble_class, button_class, button_group_class, chart_area_path, chart_area_series_class,
  chart_bar_rects, chart_bar_series_class, chart_class, chart_fallback_rows, chart_line_path,
  chart_line_series_class, chart_view_box, collapsible_class, direction_class, input_group_class,
  input_otp_class, marker_class, message_avatar_class, message_class, message_content_class,
  message_footer_class, message_group_class, message_header_class, message_scroller_class,
  message_scroller_intent_attribute, message_scroller_is_at_bottom,
  message_scroller_jump_button_class, message_scroller_show_unread_marker,
  otp_apply_paste_filtered, otp_slots,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewTarget {
  Web,
  Desktop,
}

impl PreviewTarget {
  fn label(self) -> &'static str {
    match self {
      Self::Web => "web",
      Self::Desktop => "desktop",
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewLine {
  pub label: &'static str,
  pub value: String,
}

impl PreviewLine {
  pub fn render(&self, target: PreviewTarget) -> String {
    format!("dioxus-ui {} demo {}: {}", target.label(), self.label, self.value)
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentPreviewTarget {
  pub component: &'static str,
  pub label: &'static str,
  pub panel: &'static str,
  pub test_id: &'static str,
  pub coverage_level: &'static str,
  pub notes: &'static str,
}

pub const COMPONENT_PREVIEW_TARGETS: &[ComponentPreviewTarget] = &[
  ComponentPreviewTarget {
    component: "accordion",
    label: "Accordion",
    panel: "layout",
    test_id: "component-preview-accordion",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "alert",
    label: "Alert",
    panel: "feedback",
    test_id: "component-preview-alert",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "alert-dialog",
    label: "Alert Dialog",
    panel: "overlays",
    test_id: "component-preview-alert-dialog",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "aspect-ratio",
    label: "Aspect Ratio",
    panel: "layout",
    test_id: "component-preview-aspect-ratio",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "attachment",
    label: "Attachment",
    panel: "messaging",
    test_id: "component-preview-attachment",
    coverage_level: "app-owned",
    notes: "Composition target exists; domain behavior remains app-owned.",
  },
  ComponentPreviewTarget {
    component: "avatar",
    label: "Avatar",
    panel: "data-display",
    test_id: "component-preview-avatar",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "badge",
    label: "Badge",
    panel: "data-display",
    test_id: "component-preview-badge",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "breadcrumb",
    label: "Breadcrumb",
    panel: "navigation",
    test_id: "component-preview-breadcrumb",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "bubble",
    label: "Bubble",
    panel: "messaging",
    test_id: "component-preview-bubble",
    coverage_level: "app-owned",
    notes: "Composition target exists; domain behavior remains app-owned.",
  },
  ComponentPreviewTarget {
    component: "button",
    label: "Button",
    panel: "actions",
    test_id: "component-preview-button",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "button-group",
    label: "Button Group",
    panel: "actions",
    test_id: "component-preview-button-group",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "calendar",
    label: "Calendar",
    panel: "forms",
    test_id: "component-preview-calendar",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "card",
    label: "Card",
    panel: "layout",
    test_id: "component-preview-card",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "carousel",
    label: "Carousel",
    panel: "layout",
    test_id: "component-preview-carousel",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "chart",
    label: "Chart",
    panel: "data-display",
    test_id: "component-preview-chart",
    coverage_level: "app-owned",
    notes: "Composition target exists; domain behavior remains app-owned.",
  },
  ComponentPreviewTarget {
    component: "checkbox",
    label: "Checkbox",
    panel: "forms",
    test_id: "component-preview-checkbox",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "collapsible",
    label: "Collapsible",
    panel: "layout",
    test_id: "component-preview-collapsible",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "combobox",
    label: "Combobox",
    panel: "overlays",
    test_id: "component-preview-combobox",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "command",
    label: "Command",
    panel: "actions",
    test_id: "component-preview-command",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "context-menu",
    label: "Context Menu",
    panel: "overlays",
    test_id: "component-preview-context-menu",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "data-table",
    label: "Data Table",
    panel: "data-display",
    test_id: "component-preview-data-table",
    coverage_level: "app-owned",
    notes: "Composition target exists; domain behavior remains app-owned.",
  },
  ComponentPreviewTarget {
    component: "date-picker",
    label: "Date Picker",
    panel: "forms",
    test_id: "component-preview-date-picker",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "dialog",
    label: "Dialog",
    panel: "overlays",
    test_id: "component-preview-dialog",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "direction",
    label: "Direction",
    panel: "layout",
    test_id: "component-preview-direction",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "drawer",
    label: "Drawer",
    panel: "overlays",
    test_id: "component-preview-drawer",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "dropdown",
    label: "Dropdown",
    panel: "overlays",
    test_id: "component-preview-dropdown",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "empty",
    label: "Empty",
    panel: "data-display",
    test_id: "component-preview-empty",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "field",
    label: "Field",
    panel: "forms",
    test_id: "component-preview-field",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "hover-card",
    label: "Hover Card",
    panel: "overlays",
    test_id: "component-preview-hover-card",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "input",
    label: "Input",
    panel: "forms",
    test_id: "component-preview-input",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "input-group",
    label: "Input Group",
    panel: "forms",
    test_id: "component-preview-input-group",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "input-otp",
    label: "Input Otp",
    panel: "forms",
    test_id: "component-preview-input-otp",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "item",
    label: "Item",
    panel: "layout",
    test_id: "component-preview-item",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "kbd",
    label: "Kbd",
    panel: "actions",
    test_id: "component-preview-kbd",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "label",
    label: "Label",
    panel: "forms",
    test_id: "component-preview-label",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "marker",
    label: "Marker",
    panel: "messaging",
    test_id: "component-preview-marker",
    coverage_level: "app-owned",
    notes: "Composition target exists; domain behavior remains app-owned.",
  },
  ComponentPreviewTarget {
    component: "menubar",
    label: "Menubar",
    panel: "overlays",
    test_id: "component-preview-menubar",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "message",
    label: "Message",
    panel: "messaging",
    test_id: "component-preview-message",
    coverage_level: "app-owned",
    notes: "Composition target exists; domain behavior remains app-owned.",
  },
  ComponentPreviewTarget {
    component: "message-scroller",
    label: "Message Scroller",
    panel: "messaging",
    test_id: "component-preview-message-scroller",
    coverage_level: "app-owned",
    notes: "Composition target exists; domain behavior remains app-owned.",
  },
  ComponentPreviewTarget {
    component: "native-select",
    label: "Native Select",
    panel: "forms",
    test_id: "component-preview-native-select",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "navigation-menu",
    label: "Navigation Menu",
    panel: "navigation",
    test_id: "component-preview-navigation-menu",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "pagination",
    label: "Pagination",
    panel: "navigation",
    test_id: "component-preview-pagination",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "popover",
    label: "Popover",
    panel: "overlays",
    test_id: "component-preview-popover",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "progress",
    label: "Progress",
    panel: "data-display",
    test_id: "component-preview-progress",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "radio-group",
    label: "Radio Group",
    panel: "forms",
    test_id: "component-preview-radio-group",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "resizable",
    label: "Resizable",
    panel: "layout",
    test_id: "component-preview-resizable",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "scroll-area",
    label: "Scroll Area",
    panel: "layout",
    test_id: "component-preview-scroll-area",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "select",
    label: "Select",
    panel: "forms",
    test_id: "component-preview-select",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "separator",
    label: "Separator",
    panel: "layout",
    test_id: "component-preview-separator",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "sheet",
    label: "Sheet",
    panel: "overlays",
    test_id: "component-preview-sheet",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "sidebar",
    label: "Sidebar",
    panel: "navigation",
    test_id: "component-preview-sidebar",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "skeleton",
    label: "Skeleton",
    panel: "feedback",
    test_id: "component-preview-skeleton",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "slider",
    label: "Slider",
    panel: "forms",
    test_id: "component-preview-slider",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "sonner",
    label: "Sonner",
    panel: "feedback",
    test_id: "component-preview-sonner",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "spinner",
    label: "Spinner",
    panel: "feedback",
    test_id: "component-preview-spinner",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "switch",
    label: "Switch",
    panel: "forms",
    test_id: "component-preview-switch",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "table",
    label: "Table",
    panel: "data-display",
    test_id: "component-preview-table",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "tabs",
    label: "Tabs",
    panel: "navigation",
    test_id: "component-preview-tabs",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "textarea",
    label: "Textarea",
    panel: "forms",
    test_id: "component-preview-textarea",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "toast",
    label: "Toast",
    panel: "feedback",
    test_id: "component-preview-toast",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "toggle",
    label: "Toggle",
    panel: "actions",
    test_id: "component-preview-toggle",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "toggle-group",
    label: "Toggle Group",
    panel: "actions",
    test_id: "component-preview-toggle-group",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
  },
  ComponentPreviewTarget {
    component: "tooltip",
    label: "Tooltip",
    panel: "overlays",
    test_id: "component-preview-tooltip",
    coverage_level: "runtime-planned",
    notes: "Rendered target exists; renderer behavior needs separate runtime verification.",
  },
  ComponentPreviewTarget {
    component: "typography",
    label: "Typography",
    panel: "data-display",
    test_id: "component-preview-typography",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
];

pub fn preview_lines(target: PreviewTarget) -> Vec<PreviewLine> {
  let config = PreviewConfig::for_target(target);
  let otp = otp_apply_paste_filtered(
    config.otp_seed,
    config.otp_index,
    config.otp_paste,
    config.otp_len,
    |ch| ch.is_ascii_digit(),
  );
  let chart_series = ChartSeries::new(config.chart_id, config.chart_label, config.chart_points);
  let chart_x = ChartScale::new(config.chart_x_domain, config.chart_x_range);
  let chart_y = ChartScale::new(config.chart_y_domain, config.chart_y_range);
  let metrics =
    MessageScrollerMetrics::new(config.scroll_top, config.viewport_height, config.content_height);
  let unread_visible = message_scroller_show_unread_marker(config.scroll_intent, config.unread);

  vec![
    PreviewLine {
      label: "button group class",
      value: button_group_class(
        config.button_orientation,
        config.button_attached,
        config.button_class,
      ),
    },
    PreviewLine {
      label: "input group class",
      value: input_group_class(
        config.input_disabled,
        config.input_invalid,
        config.input_group_class,
      ),
    },
    PreviewLine {
      label: "input otp helper",
      value: format!("{}/{}", otp, otp_slots(&otp, config.otp_len, config.otp_active).len()),
    },
    PreviewLine {
      label: "attachment class",
      value: attachment_class(
        config.attachment_state,
        config.attachment_size,
        config.attachment_orientation,
        config.attachment_class,
      ),
    },
    PreviewLine {
      label: "bubble class",
      value: bubble_class(config.bubble_align, config.bubble_class),
    },
    PreviewLine {
      label: "message class",
      value: message_class(config.message_align, config.message_class),
    },
    PreviewLine {
      label: "message scroller helper",
      value: format!(
        "{}/{}/{}",
        message_scroller_is_at_bottom(metrics, config.bottom_threshold),
        message_scroller_jump_button_class(unread_visible, config.jump_button_class),
        message_scroller_intent_attribute(config.scroll_intent)
      ),
    },
    PreviewLine {
      label: "marker class",
      value: marker_class(config.marker_variant, config.marker_class),
    },
    PreviewLine {
      label: "chart helper",
      value: format!(
        "{}/{}/{}",
        chart_view_box(config.chart_width, config.chart_height),
        chart_line_path(&chart_series, chart_x, chart_y),
        chart_fallback_rows(&[chart_series]).len()
      ),
    },
    PreviewLine {
      label: "direction class/attr",
      value: format!(
        "{}/{}",
        direction_class(config.direction_class),
        config.direction.attribute()
      ),
    },
    PreviewLine {
      label: "collapsible class",
      value: collapsible_class(config.collapsible_open, config.collapsible_class),
    },
  ]
}

pub fn preview_smoke_lines(target: PreviewTarget) -> Vec<String> {
  preview_lines(target).into_iter().map(|line| line.render(target)).collect()
}

#[component]
pub fn PreviewSurface(target: PreviewTarget, title: String) -> Element {
  let states = preview_lines(target);
  let mut disclosure_open = use_signal(|| false);
  let mut overlay_open = use_signal(|| false);
  let mut selected_option = use_signal(|| "alpha");
  let mut command_active = use_signal(|| "open-file");
  let mut scroll_status = use_signal(|| "held");
  let mut dialog_open = use_signal(|| false);
  let mut alert_dialog_open = use_signal(|| false);
  let mut alert_dialog_result = use_signal(|| "pending");
  let root = match target {
    PreviewTarget::Web => "web",
    PreviewTarget::Desktop => "desktop",
  };
  let disclosure_state = if disclosure_open() { "open" } else { "closed" };
  let disclosure_expanded = if disclosure_open() { "true" } else { "false" };
  let overlay_state = if overlay_open() { "open" } else { "closed" };
  let overlay_expanded = if overlay_open() { "true" } else { "false" };
  let selected_option_value = selected_option();
  let command_active_value = command_active();
  let scroll_status_value = scroll_status();
  let primary_button_class =
    button_class(ButtonVariant::Primary, ButtonSize::Md, UiDensity::Comfortable, "");
  let secondary_button_class =
    button_class(ButtonVariant::Secondary, ButtonSize::Sm, UiDensity::Compact, "");
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
    main {
      class: "min-h-screen bg-white text-zinc-950",
      "data-preview-root": "{root}",
      section {
        class: "mx-auto flex w-full max-w-6xl flex-col gap-6 px-6 py-8",
        "data-preview-panel": "overview",
        header {
          class: "flex flex-col gap-2 border-b border-zinc-200 pb-4",
          h1 { class: "text-2xl font-semibold", "{title}" }
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
          class: "grid gap-3 rounded-md border border-zinc-200 p-4 sm:grid-cols-2 lg:grid-cols-3",
          "data-preview-panel": "mobile-profile",
          h2 { class: "text-sm font-medium sm:col-span-2 lg:col-span-3", "Mobile Web profile" }
          p {
            class: "text-sm text-zinc-600 sm:col-span-2 lg:col-span-3",
            "Source-level markers for mobile-width Web verification. Native device behavior remains app-owned."
          }
          div {
            class: "rounded-md bg-zinc-50 p-3 text-sm",
            "data-mobile-profile": "touch-targets",
            button { class: "{primary_button_class} min-h-11 w-full", "Touch target" }
          }
          div {
            class: "rounded-md bg-zinc-50 p-3 text-sm",
            "data-mobile-profile": "hover-alternative",
            button { class: "{secondary_button_class} min-h-11 w-full", "Tap or focus" }
          }
          div {
            class: "rounded-md bg-zinc-50 p-3 text-sm",
            "data-mobile-profile": "safe-area-owned",
            "Safe-area padding is owned by the app shell."
          }
          div {
            class: "rounded-md bg-zinc-50 p-3 text-sm",
            "data-mobile-profile": "reduced-motion",
            "Animation and timer policy remains app-owned."
          }
          div {
            class: "rounded-md bg-zinc-50 p-3 text-sm",
            "data-mobile-profile": "visible-status",
            "Visible status text mirrors runtime-sensitive behavior."
          }
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
          class: "grid gap-4 lg:grid-cols-2",
          "data-preview-panel": "interactions",
          "data-interaction-root": "runtime",
          article {
            class: "rounded-md border border-zinc-200 p-4",
            "data-interaction-target": "disclosure",
            "data-state": "{disclosure_state}",
            h2 { class: "text-sm font-medium", "Disclosure interaction" }
            button {
              class: "{secondary_button_class} mt-3",
              "aria-expanded": "{disclosure_expanded}",
              "aria-controls": "interaction-disclosure-content",
              "data-interaction-control": "disclosure-trigger",
              onclick: move |_| disclosure_open.toggle(),
              "Toggle disclosure"
            }
            div {
              id: "interaction-disclosure-content",
              class: "mt-3 rounded-md bg-zinc-50 p-3 text-sm text-zinc-700",
              "data-interaction-state": "disclosure-content",
              hidden: !disclosure_open(),
              "Disclosure content is visible when open."
            }
          }
          article {
            class: "rounded-md border border-zinc-200 p-4",
            "data-interaction-target": "overlay",
            "data-state": "{overlay_state}",
            h2 { class: "text-sm font-medium", "Overlay interaction" }
            button {
              class: "{secondary_button_class} mt-3",
              "aria-expanded": "{overlay_expanded}",
              "aria-controls": "interaction-overlay-content",
              "data-interaction-control": "overlay-trigger",
              onclick: move |_| overlay_open.set(true),
              "Open overlay"
            }
            div {
              id: "interaction-overlay-content",
              class: "mt-3 rounded-md border border-zinc-200 bg-white p-3 text-sm shadow-sm",
              role: "dialog",
              "aria-label": "Interaction overlay",
              "data-interaction-state": "overlay-content",
              hidden: !overlay_open(),
              p { class: "font-medium", "Interaction overlay" }
              button {
                class: "{secondary_button_class} mt-3",
                "data-interaction-control": "overlay-close",
                onclick: move |_| overlay_open.set(false),
                "Close"
              }
            }
          }
          article {
            class: "rounded-md border border-zinc-200 p-4",
            "data-interaction-target": "selection",
            "data-state": "{selected_option_value}",
            h2 { class: "text-sm font-medium", "Selection interaction" }
            div {
              class: "mt-3 flex gap-2",
              role: "radiogroup",
              "aria-label": "Interaction selection",
              for option in ["alpha", "beta"] {
                button {
                  class: "{secondary_button_class}",
                  role: "radio",
                  "aria-checked": "{selected_option_value == option}",
                  "data-interaction-option": "{option}",
                  onclick: move |_| selected_option.set(option),
                  "{option}"
                }
              }
            }
          }
          article {
            class: "rounded-md border border-zinc-200 p-4",
            "data-interaction-target": "keyboard",
            h2 { class: "text-sm font-medium", "Keyboard-visible state" }
            div {
              class: "mt-3 rounded-md border border-zinc-200 p-2",
              role: "listbox",
              tabindex: "0",
              "aria-activedescendant": "interaction-command-{command_active_value}",
              "data-interaction-control": "keyboard-listbox",
              for option in ["open-file", "save-file"] {
                button {
                  id: "interaction-command-{option}",
                  class: "{secondary_button_class} mr-2",
                  role: "option",
                  "aria-selected": "{command_active_value == option}",
                  "data-interaction-option": "{option}",
                  onclick: move |_| command_active.set(option),
                  "{option}"
                }
              }
            }
          }
          article {
            class: "rounded-md border border-zinc-200 p-4",
            "data-interaction-target": "scroll-status",
            "data-state": "{scroll_status_value}",
            h2 { class: "text-sm font-medium", "Scroll status interaction" }
            p {
              class: "mt-3 text-sm text-zinc-600",
              "data-interaction-state": "scroll-status",
              "Scroll status: {scroll_status_value}"
            }
            button {
              class: "{secondary_button_class} mt-3",
              "data-interaction-control": "scroll-jump",
              onclick: move |_| scroll_status.set("jumped"),
              "Jump to latest"
            }
          }
          article {
            class: "rounded-md border border-zinc-200 p-4",
            "data-interaction-target": "alert-dialog",
            "data-state": if alert_dialog_open() { "open" } else { "closed" },
            "data-result": "{alert_dialog_result}",
            h2 { class: "text-sm font-medium", "Alert dialog interaction" }
            button {
              class: "{secondary_button_class} mt-3",
              "data-interaction-control": "alert-dialog-trigger",
              onclick: move |_| alert_dialog_open.set(true),
              "Delete project"
            }
            AlertDialogOverlay { open: alert_dialog_open() }
            AlertDialogContent {
              open: alert_dialog_open(),
              on_open_change: move |open| alert_dialog_open.set(open),
              AlertDialogTitle { "Delete project?" }
              AlertDialogDescription { "This cannot be undone." }
              AlertDialogCancel { on_open_change: move |open| alert_dialog_open.set(open), "Cancel" }
              AlertDialogAction {
                variant: AlertDialogActionVariant::Destructive,
                onclick: move |_| alert_dialog_result.set("confirmed"),
                on_open_change: move |open| alert_dialog_open.set(open),
                "Delete"
              }
            }
          }
          article {
            class: "rounded-md border border-zinc-200 p-4",
            "data-interaction-target": "dialog",
            "data-state": if dialog_open() { "open" } else { "closed" },
            h2 { class: "text-sm font-medium", "Dialog interaction" }
            button {
              class: "{secondary_button_class} mt-3",
              "data-interaction-control": "dialog-trigger",
              onclick: move |_| dialog_open.set(true),
              "Open dialog"
            }
            DialogOverlay {
              open: dialog_open(),
              on_open_change: move |open| dialog_open.set(open),
              dismiss: DismissBehavior { outside_pointer: true, ..DismissBehavior::dialog_default() },
            }
            DialogContent {
              open: dialog_open(),
              on_open_change: move |open| dialog_open.set(open),
              DialogTitle { "Rename project" }
              DialogDescription { "Focus stays inside until the dialog closes." }
              input {
                class: "rounded-md border border-zinc-200 px-2 py-1 text-sm",
                "aria-label": "Project name",
                "data-interaction-control": "dialog-input",
              }
              DialogClose {
                class: "{secondary_button_class}",
                on_open_change: move |open| dialog_open.set(open),
                "Cancel"
              }
            }
          }
        }
        section {
          class: "grid gap-3 md:grid-cols-2 xl:grid-cols-3",
          "data-preview-panel": "inventory",
          section {
            class: "grid gap-2 md:col-span-2 xl:col-span-3",
            "aria-label": "Rendered component coverage targets",
            for target in COMPONENT_PREVIEW_TARGETS {
              article {
                key: "{target.test_id}",
                class: "rounded-md border border-zinc-200 bg-zinc-50 p-3",
                "data-component-preview": "{target.test_id}",
                "data-component": "{target.component}",
                "data-component-panel": "{target.panel}",
                "data-component-coverage": "{target.coverage_level}",
                h2 { class: "text-sm font-medium text-zinc-950", "{target.label}" }
                p { class: "mt-1 text-xs text-zinc-600", "{target.notes}" }
              }
            }
          }
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

struct PreviewConfig {
  button_orientation: ButtonGroupOrientation,
  button_attached: bool,
  button_class: &'static str,
  input_disabled: bool,
  input_invalid: bool,
  input_group_class: &'static str,
  otp_seed: &'static str,
  otp_index: usize,
  otp_paste: &'static str,
  otp_len: usize,
  otp_active: usize,
  attachment_state: AttachmentState,
  attachment_size: AttachmentSize,
  attachment_orientation: AttachmentOrientation,
  attachment_class: &'static str,
  bubble_align: BubbleAlign,
  bubble_class: &'static str,
  message_align: MessageAlign,
  message_class: &'static str,
  scroll_top: f64,
  viewport_height: f64,
  content_height: f64,
  unread: usize,
  scroll_intent: MessageScrollerIntent,
  bottom_threshold: f64,
  jump_button_class: &'static str,
  marker_variant: MarkerVariant,
  marker_class: &'static str,
  chart_id: &'static str,
  chart_label: &'static str,
  chart_points: Vec<ChartPoint>,
  chart_x_domain: ChartDomain,
  chart_x_range: ChartDomain,
  chart_y_domain: ChartDomain,
  chart_y_range: ChartDomain,
  chart_width: f64,
  chart_height: f64,
  direction: TextDirection,
  direction_class: &'static str,
  collapsible_open: bool,
  collapsible_class: &'static str,
}

impl PreviewConfig {
  fn for_target(target: PreviewTarget) -> Self {
    match target {
      PreviewTarget::Web => Self {
        button_orientation: ButtonGroupOrientation::Horizontal,
        button_attached: true,
        button_class: "w-full",
        input_disabled: false,
        input_invalid: false,
        input_group_class: "max-w-sm",
        otp_seed: "",
        otp_index: 0,
        otp_paste: "12a3",
        otp_len: 6,
        otp_active: 3,
        attachment_state: AttachmentState::Uploading,
        attachment_size: AttachmentSize::Default,
        attachment_orientation: AttachmentOrientation::Horizontal,
        attachment_class: "max-w-md",
        bubble_align: BubbleAlign::End,
        bubble_class: "max-w-lg",
        message_align: MessageAlign::End,
        message_class: "max-w-2xl",
        scroll_top: 880.0,
        viewport_height: 300.0,
        content_height: 1200.0,
        unread: 2,
        scroll_intent: MessageScrollerIntent::Hold,
        bottom_threshold: 24.0,
        jump_button_class: "rounded-full",
        marker_variant: MarkerVariant::Border,
        marker_class: "text-blue-700",
        chart_id: "revenue",
        chart_label: "Revenue",
        chart_points: vec![
          ChartPoint::new(0.0, 12.0),
          ChartPoint::new(1.0, 18.0),
          ChartPoint::missing(2.0),
          ChartPoint::new(3.0, 24.0),
        ],
        chart_x_domain: ChartDomain::new(0.0, 3.0),
        chart_x_range: ChartDomain::new(32.0, 608.0),
        chart_y_domain: ChartDomain::new(0.0, 24.0),
        chart_y_range: ChartDomain::new(288.0, 32.0),
        chart_width: 640.0,
        chart_height: 320.0,
        direction: TextDirection::Rtl,
        direction_class: "block",
        collapsible_open: false,
        collapsible_class: "max-w-sm",
      },
      PreviewTarget::Desktop => Self {
        button_orientation: ButtonGroupOrientation::Vertical,
        button_attached: false,
        button_class: "items-start",
        input_disabled: true,
        input_invalid: true,
        input_group_class: "max-w-xs",
        otp_seed: "1",
        otp_index: 1,
        otp_paste: "2b34",
        otp_len: 4,
        otp_active: 3,
        attachment_state: AttachmentState::Error,
        attachment_size: AttachmentSize::Sm,
        attachment_orientation: AttachmentOrientation::Vertical,
        attachment_class: "max-w-xs",
        bubble_align: BubbleAlign::Start,
        bubble_class: "max-w-sm",
        message_align: MessageAlign::Start,
        message_class: "max-w-xl",
        scroll_top: 500.0,
        viewport_height: 240.0,
        content_height: 1200.0,
        unread: 3,
        scroll_intent: MessageScrollerIntent::JumpToLatest,
        bottom_threshold: 16.0,
        jump_button_class: "text-red-700",
        marker_variant: MarkerVariant::Separator,
        marker_class: "text-red-700",
        chart_id: "cost",
        chart_label: "Cost",
        chart_points: vec![
          ChartPoint::new(0.0, 8.0),
          ChartPoint::new(1.0, 11.0),
          ChartPoint::new(2.0, 10.0),
          ChartPoint::new(3.0, 13.0),
        ],
        chart_x_domain: ChartDomain::new(0.0, 3.0),
        chart_x_range: ChartDomain::new(24.0, 456.0),
        chart_y_domain: ChartDomain::new(0.0, 16.0),
        chart_y_range: ChartDomain::new(220.0, 24.0),
        chart_width: 480.0,
        chart_height: 240.0,
        direction: TextDirection::Ltr,
        direction_class: "inline",
        collapsible_open: true,
        collapsible_class: "max-w-xs",
      },
    }
  }
}
