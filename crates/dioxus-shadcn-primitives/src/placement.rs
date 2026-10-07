//! Overlay positioning: where to put floating content next to its anchor so it stays inside
//! the viewport, flipping to the opposite side or sliding along the edge on collision.
//! Coordinates are CSS pixels in the same space as the anchor and viewport rects.

use crate::{OverlayAlign, OverlaySide};

/// A rectangle in CSS pixels, positioned by its top-left corner.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OverlayRect {
  /// Left edge.
  pub x: i32,
  /// Top edge.
  pub y: i32,
  /// Width.
  pub width: i32,
  /// Height.
  pub height: i32,
}

impl OverlayRect {
  /// The right edge: `x + width`.
  pub const fn right(self) -> i32 {
    self.x + self.width
  }

  /// The bottom edge: `y + height`.
  pub const fn bottom(self) -> i32 {
    self.y + self.height
  }
}

/// The measured size of the overlay content, in CSS pixels.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OverlaySize {
  /// Width.
  pub width: i32,
  /// Height.
  pub height: i32,
}

/// Extra distance, in CSS pixels, between the overlay and its anchor.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OverlayOffset {
  /// Gap away from the anchor along the side's axis; negative values overlap the anchor.
  pub main_axis: i16,
  /// Slide along the anchor's edge, added after alignment (right or down when positive).
  pub cross_axis: i16,
}

/// Margin, in CSS pixels, kept between the overlay and each viewport edge when checking for
/// collisions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CollisionPadding {
  /// Margin from the top edge.
  pub top: u16,
  /// Margin from the right edge.
  pub right: u16,
  /// Margin from the bottom edge.
  pub bottom: u16,
  /// Margin from the left edge.
  pub left: u16,
}

/// What to do when the overlay would not fit inside the padded viewport.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CollisionStrategy {
  /// Keep the requested position even if it overflows.
  #[default]
  None,
  /// Move to the opposite side when the requested side lacks room.
  Flip,
  /// Slide along the anchor's edge to stay inside the viewport.
  Shift,
  /// Flip first, then shift.
  FlipShift,
}

/// Everything `compute_overlay_placement` needs. All rects and sizes are CSS pixels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OverlayPlacementInput {
  /// The trigger element's rect.
  pub anchor: OverlayRect,
  /// The overlay content's size.
  pub overlay: OverlaySize,
  /// The area the overlay must stay inside, before collision padding.
  pub viewport: OverlayRect,
  /// Requested side of the anchor.
  pub side: OverlaySide,
  /// Requested alignment along the anchor's edge.
  pub align: OverlayAlign,
  /// Gap and slide away from the aligned position.
  pub offset: OverlayOffset,
  /// Margin kept from the viewport edges during collision handling.
  pub collision_padding: CollisionPadding,
  /// How to react when the overlay would overflow.
  pub collision_strategy: CollisionStrategy,
}

/// Where the overlay ends up, and whether collision handling moved it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OverlayPlacement {
  /// Left edge of the overlay, in CSS pixels.
  pub x: i32,
  /// Top edge of the overlay, in CSS pixels.
  pub y: i32,
  /// The side actually used, which differs from the requested one after a flip.
  pub side: OverlaySide,
  /// The requested alignment, unchanged.
  pub align: OverlayAlign,
  /// The overlay moved to the opposite side.
  pub flipped: bool,
  /// The overlay slid along the anchor's edge to stay inside the viewport.
  pub shifted: bool,
}

/// Positions the overlay on its requested side and alignment, then applies the collision
/// strategy. A flip happens only when the requested side lacks room for the overlay plus a
/// positive main-axis offset and the opposite side has at least as much room. A shift clamps
/// the cross axis into the padded viewport, pinning to its start edge when the overlay is too
/// big to fit. `OverlaySide::Inline` is never flipped or shifted.
pub fn compute_overlay_placement(input: OverlayPlacementInput) -> OverlayPlacement {
  let side = resolved_side(input);
  let (mut x, mut y) = base_position(input, side);
  let mut shifted = false;

  if matches!(input.collision_strategy, CollisionStrategy::Shift | CollisionStrategy::FlipShift) {
    let shifted_position = shift_cross_axis(input, side, x, y);
    shifted = shifted_position != (x, y);
    x = shifted_position.0;
    y = shifted_position.1;
  }

  OverlayPlacement { x, y, side, align: input.align, flipped: side != input.side, shifted }
}

fn resolved_side(input: OverlayPlacementInput) -> OverlaySide {
  if !matches!(input.collision_strategy, CollisionStrategy::Flip | CollisionStrategy::FlipShift) {
    return input.side;
  }

  let Some(opposite_side) = opposite_side(input.side) else {
    return input.side;
  };

  let required_space =
    main_axis_size(input.overlay, input.side) + i32::from(input.offset.main_axis.max(0));
  let preferred_space = available_space(input, input.side);
  let opposite_space = available_space(input, opposite_side);

  if preferred_space < required_space && opposite_space >= preferred_space {
    opposite_side
  } else {
    input.side
  }
}

fn opposite_side(side: OverlaySide) -> Option<OverlaySide> {
  match side {
    OverlaySide::Top => Some(OverlaySide::Bottom),
    OverlaySide::Right => Some(OverlaySide::Left),
    OverlaySide::Bottom => Some(OverlaySide::Top),
    OverlaySide::Left => Some(OverlaySide::Right),
    OverlaySide::Inline => None,
  }
}

fn available_space(input: OverlayPlacementInput, side: OverlaySide) -> i32 {
  let bounds = padded_viewport(input.viewport, input.collision_padding);

  match side {
    OverlaySide::Top => input.anchor.y - bounds.y,
    OverlaySide::Right => bounds.right() - input.anchor.right(),
    OverlaySide::Bottom => bounds.bottom() - input.anchor.bottom(),
    OverlaySide::Left => input.anchor.x - bounds.x,
    OverlaySide::Inline => i32::MAX,
  }
}

fn main_axis_size(size: OverlaySize, side: OverlaySide) -> i32 {
  match side {
    OverlaySide::Top | OverlaySide::Bottom | OverlaySide::Inline => size.height,
    OverlaySide::Right | OverlaySide::Left => size.width,
  }
}

fn base_position(input: OverlayPlacementInput, side: OverlaySide) -> (i32, i32) {
  let main_offset = i32::from(input.offset.main_axis);
  let cross_offset = i32::from(input.offset.cross_axis);

  match side {
    OverlaySide::Top => (
      aligned_cross_axis(input.anchor.x, input.anchor.width, input.overlay.width, input.align)
        + cross_offset,
      input.anchor.y - input.overlay.height - main_offset,
    ),
    OverlaySide::Right => (
      input.anchor.right() + main_offset,
      aligned_cross_axis(input.anchor.y, input.anchor.height, input.overlay.height, input.align)
        + cross_offset,
    ),
    OverlaySide::Bottom => (
      aligned_cross_axis(input.anchor.x, input.anchor.width, input.overlay.width, input.align)
        + cross_offset,
      input.anchor.bottom() + main_offset,
    ),
    OverlaySide::Left => (
      input.anchor.x - input.overlay.width - main_offset,
      aligned_cross_axis(input.anchor.y, input.anchor.height, input.overlay.height, input.align)
        + cross_offset,
    ),
    OverlaySide::Inline => (input.anchor.x + cross_offset, input.anchor.y + main_offset),
  }
}

fn aligned_cross_axis(start: i32, anchor_size: i32, overlay_size: i32, align: OverlayAlign) -> i32 {
  match align {
    OverlayAlign::Start => start,
    OverlayAlign::Center => start + (anchor_size - overlay_size) / 2,
    OverlayAlign::End => start + anchor_size - overlay_size,
  }
}

fn shift_cross_axis(input: OverlayPlacementInput, side: OverlaySide, x: i32, y: i32) -> (i32, i32) {
  let bounds = padded_viewport(input.viewport, input.collision_padding);

  match side {
    OverlaySide::Top | OverlaySide::Bottom => {
      let shifted_x = clamp_axis(x, input.overlay.width, bounds.x, bounds.right());
      (shifted_x, y)
    }
    OverlaySide::Right | OverlaySide::Left => {
      let shifted_y = clamp_axis(y, input.overlay.height, bounds.y, bounds.bottom());
      (x, shifted_y)
    }
    OverlaySide::Inline => (x, y),
  }
}

fn padded_viewport(viewport: OverlayRect, padding: CollisionPadding) -> OverlayRect {
  let left = i32::from(padding.left);
  let top = i32::from(padding.top);
  let right = i32::from(padding.right);
  let bottom = i32::from(padding.bottom);

  OverlayRect {
    x: viewport.x + left,
    y: viewport.y + top,
    width: (viewport.width - left - right).max(0),
    height: (viewport.height - top - bottom).max(0),
  }
}

fn clamp_axis(position: i32, size: i32, min: i32, max: i32) -> i32 {
  if size >= max - min { min } else { position.clamp(min, max - size) }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn base_input() -> OverlayPlacementInput {
    OverlayPlacementInput {
      anchor: OverlayRect { x: 80, y: 80, width: 40, height: 20 },
      overlay: OverlaySize { width: 60, height: 30 },
      viewport: OverlayRect { x: 0, y: 0, width: 240, height: 180 },
      side: OverlaySide::Bottom,
      align: OverlayAlign::Center,
      offset: OverlayOffset { main_axis: 8, cross_axis: 0 },
      collision_padding: CollisionPadding::default(),
      collision_strategy: CollisionStrategy::None,
    }
  }

  #[test]
  fn no_collision_uses_requested_placement() {
    let placement = compute_overlay_placement(base_input());

    assert_eq!(
      placement,
      OverlayPlacement {
        x: 70,
        y: 108,
        side: OverlaySide::Bottom,
        align: OverlayAlign::Center,
        flipped: false,
        shifted: false,
      }
    );
  }

  #[test]
  fn flip_uses_opposite_side_when_preferred_side_is_clipped() {
    let mut input = base_input();
    input.anchor.y = 145;
    input.collision_strategy = CollisionStrategy::Flip;

    let placement = compute_overlay_placement(input);

    assert_eq!(placement.side, OverlaySide::Top);
    assert_eq!(placement.y, 107);
    assert!(placement.flipped);
    assert!(!placement.shifted);
  }

  #[test]
  fn shift_keeps_side_and_moves_cross_axis_inside_viewport() {
    let mut input = base_input();
    input.anchor.x = 220;
    input.collision_strategy = CollisionStrategy::Shift;

    let placement = compute_overlay_placement(input);

    assert_eq!(placement.side, OverlaySide::Bottom);
    assert_eq!(placement.x, 180);
    assert!(!placement.flipped);
    assert!(placement.shifted);
  }

  #[test]
  fn flip_shift_flips_then_shifts_cross_axis() {
    let mut input = base_input();
    input.anchor.x = 220;
    input.anchor.y = 145;
    input.collision_strategy = CollisionStrategy::FlipShift;

    let placement = compute_overlay_placement(input);

    assert_eq!(placement.side, OverlaySide::Top);
    assert_eq!(placement.x, 180);
    assert_eq!(placement.y, 107);
    assert!(placement.flipped);
    assert!(placement.shifted);
  }

  #[test]
  fn collision_padding_limits_shift_bounds() {
    let mut input = base_input();
    input.anchor.x = 0;
    input.collision_padding = CollisionPadding { left: 12, right: 16, top: 0, bottom: 0 };
    input.collision_strategy = CollisionStrategy::Shift;

    let placement = compute_overlay_placement(input);

    assert_eq!(placement.x, 12);
    assert!(placement.shifted);
  }
}
