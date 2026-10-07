//! Pure state for layout components: the collapsible sidebar, resizable panels, scroll
//! areas, and the carousel. The styled `Sidebar`, `Resizable*`, `ScrollArea`, and `Carousel`
//! components in `dioxus-shadcn` keep their state in these types.

/// Whether a sidebar is collapsed.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SidebarState {
  /// The sidebar is collapsed to its narrow or hidden form.
  pub collapsed: bool,
}

impl SidebarState {
  /// A sidebar that starts collapsed or expanded.
  pub const fn new(collapsed: bool) -> Self {
    Self { collapsed }
  }

  /// The same sidebar with `collapsed` flipped.
  pub const fn toggled(self) -> Self {
    Self { collapsed: !self.collapsed }
  }
}

/// One resizable panel's size and limits, in percent of its panel group.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResizablePanelState {
  /// Current size, kept within `min_size..=max_size`.
  pub size: f64,
  /// Smallest size the panel can be dragged to.
  pub min_size: f64,
  /// Largest size the panel can be dragged to.
  pub max_size: f64,
  /// The panel is collapsed.
  pub collapsed: bool,
}

impl ResizablePanelState {
  /// A panel with `size` clamped to its bounds. Swapped bounds are put in order; a non-finite
  /// `min_size` becomes 0, a non-finite `max_size` becomes `min_size`, and a non-finite `size`
  /// becomes `min_size`. Starts expanded.
  pub fn new(size: f64, min_size: f64, max_size: f64) -> Self {
    let (min_size, max_size) = ordered_bounds(min_size, max_size);
    let size = resizable_clamp(size, min_size, max_size);

    Self { size, min_size, max_size, collapsed: false }
  }

  /// The same panel with `collapsed` set.
  pub const fn with_collapsed(mut self, collapsed: bool) -> Self {
    self.collapsed = collapsed;
    self
  }

  /// The same panel resized to `size`, clamped to its bounds.
  pub fn with_size(self, size: f64) -> Self {
    Self { size: resizable_clamp(size, self.min_size, self.max_size), ..self }
  }
}

/// Which directions a scroll area scrolls in, written to its `data-orientation`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ScrollAreaOrientation {
  /// Scrolls up and down.
  Vertical,
  /// Scrolls left and right.
  Horizontal,
  /// Scrolls in both directions.
  #[default]
  Both,
}

/// The direction a resizable panel group or carousel lays out its children.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LayoutOrientation {
  /// Children sit side by side.
  #[default]
  Horizontal,
  /// Children stack top to bottom.
  Vertical,
}

/// Which carousel slide is showing, out of how many.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CarouselState {
  /// Zero-based index of the current slide.
  pub index: usize,
  /// Number of slides.
  pub item_count: usize,
  /// Moving past the last slide wraps to the first, and back from the first wraps to the last.
  pub looping: bool,
}

impl CarouselState {
  /// A carousel at `index`, not looping. The index is not clamped; call `clamped` for that.
  pub const fn new(index: usize, item_count: usize) -> Self {
    Self { index, item_count, looping: false }
  }

  /// The same carousel with `looping` set.
  pub const fn with_looping(mut self, looping: bool) -> Self {
    self.looping = looping;
    self
  }

  /// The same carousel with `index` clamped to the last slide (0 when there are none).
  pub fn clamped(self) -> Self {
    Self { index: carousel_clamp_index(self.index, self.item_count), ..self }
  }

  /// The carousel moved one slide forward; see `carousel_next`.
  pub fn next(self) -> Self {
    Self { index: carousel_next(self.index, self.item_count, self.looping), ..self }
  }

  /// The carousel moved one slide back; see `carousel_previous`.
  pub fn previous(self) -> Self {
    Self { index: carousel_previous(self.index, self.item_count, self.looping), ..self }
  }
}

/// The collapsed state after a toggle: the opposite of `collapsed`.
pub const fn sidebar_toggle(collapsed: bool) -> bool {
  !collapsed
}

/// `size` clamped to `min_size..=max_size`, with the same ordering and non-finite handling as
/// `ResizablePanelState::new`.
pub fn resizable_clamp(size: f64, min_size: f64, max_size: f64) -> f64 {
  let (min_size, max_size) = ordered_bounds(min_size, max_size);
  let size = finite_or_default(size, min_size);

  size.clamp(min_size, max_size)
}

/// Resizes two adjacent panels by dragging the handle between them `delta` percent (positive
/// grows `first`). Each panel stays within its own bounds, and whatever one panel cannot
/// absorb the other gives back, so the pair's total size never changes. A non-finite `delta`
/// is treated as 0.
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

/// The `data-orientation` value for a scroll area: `vertical`, `horizontal`, or `both`.
pub fn scroll_area_orientation_attribute(orientation: ScrollAreaOrientation) -> &'static str {
  match orientation {
    ScrollAreaOrientation::Vertical => "vertical",
    ScrollAreaOrientation::Horizontal => "horizontal",
    ScrollAreaOrientation::Both => "both",
  }
}

/// The `data-orientation` value for a layout: `horizontal` or `vertical`.
pub fn layout_orientation_attribute(orientation: LayoutOrientation) -> &'static str {
  match orientation {
    LayoutOrientation::Horizontal => "horizontal",
    LayoutOrientation::Vertical => "vertical",
  }
}

/// `index` clamped to the last slide, or 0 when there are no slides.
pub fn carousel_clamp_index(index: usize, item_count: usize) -> usize {
  if item_count == 0 { 0 } else { index.min(item_count - 1) }
}

/// Whether a next button should be enabled: needs at least two slides, and either looping or
/// a slide after `index`.
pub fn carousel_can_go_next(index: usize, item_count: usize, looping: bool) -> bool {
  item_count > 1 && (looping || index + 1 < item_count)
}

/// Whether a previous button should be enabled: needs at least two slides, and either
/// looping or a slide before `index`.
pub fn carousel_can_go_previous(index: usize, item_count: usize, looping: bool) -> bool {
  item_count > 1 && (looping || index > 0)
}

/// The slide after `index` (clamped first). At the last slide it wraps to 0 when looping and
/// stays put otherwise; 0 when there are no slides.
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

/// The slide before `index` (clamped first). At the first slide it wraps to the last when
/// looping and stays put otherwise; 0 when there are no slides.
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
