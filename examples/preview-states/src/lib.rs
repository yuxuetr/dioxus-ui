use dioxus_ui::{
  attachment_class, bubble_class, button_group_class, chart_fallback_rows, chart_line_path,
  chart_view_box, collapsible_class, direction_class, input_group_class, marker_class,
  message_class, message_scroller_intent_attribute, message_scroller_is_at_bottom,
  message_scroller_jump_button_class, message_scroller_show_unread_marker,
  otp_apply_paste_filtered, otp_slots, AttachmentOrientation, AttachmentSize, AttachmentState,
  BubbleAlign, ButtonGroupOrientation, ChartDomain, ChartPoint, ChartScale, ChartSeries,
  MarkerVariant, MessageAlign, MessageScrollerIntent, MessageScrollerMetrics, TextDirection,
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
    format!(
      "dioxus-ui {} demo {}: {}",
      target.label(),
      self.label,
      self.value
    )
  }
}

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
  let metrics = MessageScrollerMetrics::new(
    config.scroll_top,
    config.viewport_height,
    config.content_height,
  );
  let unread_visible = message_scroller_show_unread_marker(config.scroll_intent, config.unread);

  vec![
    PreviewLine {
      label: "button group class",
      value: button_group_class(config.button_orientation, config.button_attached, config.button_class),
    },
    PreviewLine {
      label: "input group class",
      value: input_group_class(config.input_disabled, config.input_invalid, config.input_group_class),
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
      value: format!("{}/{}", direction_class(config.direction_class), config.direction.attribute()),
    },
    PreviewLine {
      label: "collapsible class",
      value: collapsible_class(config.collapsible_open, config.collapsible_class),
    },
  ]
}

pub fn preview_smoke_lines(target: PreviewTarget) -> Vec<String> {
  preview_lines(target)
    .into_iter()
    .map(|line| line.render(target))
    .collect()
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
