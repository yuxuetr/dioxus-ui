#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SidebarState {
  pub collapsed: bool,
}

impl SidebarState {
  pub const fn new(collapsed: bool) -> Self {
    Self { collapsed }
  }

  pub const fn toggled(self) -> Self {
    Self { collapsed: !self.collapsed }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResizablePanelState {
  pub size: f64,
  pub min_size: f64,
  pub max_size: f64,
  pub collapsed: bool,
}

impl ResizablePanelState {
  pub fn new(size: f64, min_size: f64, max_size: f64) -> Self {
    let (min_size, max_size) = ordered_bounds(min_size, max_size);
    let size = resizable_clamp(size, min_size, max_size);

    Self { size, min_size, max_size, collapsed: false }
  }

  pub const fn with_collapsed(mut self, collapsed: bool) -> Self {
    self.collapsed = collapsed;
    self
  }

  pub fn with_size(self, size: f64) -> Self {
    Self { size: resizable_clamp(size, self.min_size, self.max_size), ..self }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScrollAreaOrientation {
  Vertical,
  Horizontal,
  #[default]
  Both,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LayoutOrientation {
  #[default]
  Horizontal,
  Vertical,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CarouselState {
  pub index: usize,
  pub item_count: usize,
  pub looping: bool,
}

impl CarouselState {
  pub const fn new(index: usize, item_count: usize) -> Self {
    Self { index, item_count, looping: false }
  }

  pub const fn with_looping(mut self, looping: bool) -> Self {
    self.looping = looping;
    self
  }

  pub fn clamped(self) -> Self {
    Self { index: carousel_clamp_index(self.index, self.item_count), ..self }
  }

  pub fn next(self) -> Self {
    Self { index: carousel_next(self.index, self.item_count, self.looping), ..self }
  }

  pub fn previous(self) -> Self {
    Self { index: carousel_previous(self.index, self.item_count, self.looping), ..self }
  }
}

pub const fn sidebar_toggle(collapsed: bool) -> bool {
  !collapsed
}

pub fn resizable_clamp(size: f64, min_size: f64, max_size: f64) -> f64 {
  let (min_size, max_size) = ordered_bounds(min_size, max_size);
  let size = finite_or_default(size, min_size);

  size.clamp(min_size, max_size)
}

pub fn resizable_resize_pair(
  first: ResizablePanelState,
  second: ResizablePanelState,
  delta: f64,
) -> (ResizablePanelState, ResizablePanelState) {
  let delta = finite_or_default(delta, 0.0);
  let first_size = resizable_clamp(first.size + delta, first.min_size, first.max_size);
  let consumed_delta = first_size - first.size;
  let second_size = resizable_clamp(second.size - consumed_delta, second.min_size, second.max_size);
  let second_consumed_delta = second.size - second_size;
  let first_size =
    resizable_clamp(first.size + second_consumed_delta, first.min_size, first.max_size);

  (first.with_size(first_size), second.with_size(second_size))
}

pub fn scroll_area_orientation_attribute(orientation: ScrollAreaOrientation) -> &'static str {
  match orientation {
    ScrollAreaOrientation::Vertical => "vertical",
    ScrollAreaOrientation::Horizontal => "horizontal",
    ScrollAreaOrientation::Both => "both",
  }
}

pub fn layout_orientation_attribute(orientation: LayoutOrientation) -> &'static str {
  match orientation {
    LayoutOrientation::Horizontal => "horizontal",
    LayoutOrientation::Vertical => "vertical",
  }
}

pub fn carousel_clamp_index(index: usize, item_count: usize) -> usize {
  if item_count == 0 { 0 } else { index.min(item_count - 1) }
}

pub fn carousel_can_go_next(index: usize, item_count: usize, looping: bool) -> bool {
  item_count > 1 && (looping || index + 1 < item_count)
}

pub fn carousel_can_go_previous(index: usize, item_count: usize, looping: bool) -> bool {
  item_count > 1 && (looping || index > 0)
}

pub fn carousel_next(index: usize, item_count: usize, looping: bool) -> usize {
  if item_count == 0 {
    return 0;
  }

  let index = carousel_clamp_index(index, item_count);

  if index + 1 < item_count {
    index + 1
  } else if looping {
    0
  } else {
    index
  }
}

pub fn carousel_previous(index: usize, item_count: usize, looping: bool) -> usize {
  if item_count == 0 {
    return 0;
  }

  let index = carousel_clamp_index(index, item_count);

  if index > 0 {
    index - 1
  } else if looping {
    item_count - 1
  } else {
    index
  }
}

fn ordered_bounds(min_size: f64, max_size: f64) -> (f64, f64) {
  let min_size = finite_or_default(min_size, 0.0);
  let max_size = finite_or_default(max_size, min_size);

  if min_size <= max_size { (min_size, max_size) } else { (max_size, min_size) }
}

fn finite_or_default(value: f64, default: f64) -> f64 {
  if value.is_finite() { value } else { default }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn toggles_sidebar_state() {
    let state = SidebarState::new(false).toggled();

    assert!(state.collapsed);
    assert!(!sidebar_toggle(state.collapsed));
  }

  #[test]
  fn clamps_resizable_panel_size() {
    assert_eq!(resizable_clamp(120.0, 20.0, 80.0), 80.0);
    assert_eq!(resizable_clamp(10.0, 20.0, 80.0), 20.0);
    assert_eq!(resizable_clamp(50.0, 80.0, 20.0), 50.0);
  }

  #[test]
  fn resizes_panel_pair_with_bounds() {
    let first = ResizablePanelState::new(50.0, 20.0, 80.0);
    let second = ResizablePanelState::new(50.0, 20.0, 80.0);
    let (first, second) = resizable_resize_pair(first, second, 40.0);

    assert_eq!(first.size, 80.0);
    assert_eq!(second.size, 20.0);
  }

  #[test]
  fn maps_scroll_orientation() {
    assert_eq!(scroll_area_orientation_attribute(ScrollAreaOrientation::Horizontal), "horizontal");
  }

  #[test]
  fn carousel_navigation_respects_boundaries() {
    assert_eq!(carousel_next(0, 3, false), 1);
    assert_eq!(carousel_next(2, 3, false), 2);
    assert_eq!(carousel_previous(0, 3, false), 0);
    assert_eq!(carousel_previous(0, 3, true), 2);
  }

  #[test]
  fn carousel_state_moves_and_clamps() {
    let state = CarouselState::new(9, 3).with_looping(true).clamped();

    assert_eq!(state.index, 2);
    assert_eq!(state.next().index, 0);
    assert_eq!(state.previous().index, 1);
  }
}
