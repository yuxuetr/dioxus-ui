//! Styled Dioxus UI components.

#[cfg(feature = "accordion")]
pub mod accordion;

#[cfg(feature = "badge")]
pub mod badge;

#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "checkbox")]
pub mod checkbox;

#[cfg(feature = "dialog")]
pub mod dialog;

#[cfg(feature = "dropdown")]
pub mod dropdown;

#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "label")]
pub mod label;

#[cfg(feature = "popover")]
pub mod popover;

#[cfg(feature = "select")]
pub mod select;

#[cfg(feature = "separator")]
pub mod separator;

#[cfg(feature = "skeleton")]
pub mod skeleton;

#[cfg(feature = "textarea")]
pub mod textarea;

#[cfg(feature = "accordion")]
pub use accordion::{
  accordion_content_class, accordion_item_class, accordion_trigger_class, AccordionContent,
  AccordionItem, AccordionTrigger, ACCORDION_CONTENT_BASE_CLASS, ACCORDION_ITEM_BASE_CLASS,
  ACCORDION_TRIGGER_BASE_CLASS,
};
#[cfg(feature = "badge")]
pub use badge::{badge_class, Badge, BadgeVariant, BADGE_BASE_CLASS};
#[cfg(feature = "button")]
pub use button::{button_class, Button, ButtonSize, ButtonVariant, BUTTON_BASE_CLASS};
#[cfg(feature = "checkbox")]
pub use checkbox::{checkbox_class, Checkbox, CHECKBOX_BASE_CLASS};
#[cfg(feature = "dialog")]
pub use dialog::{
  dialog_close_class, dialog_content_class, dialog_description_class, dialog_overlay_class,
  dialog_title_class, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogTitle,
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
  DIALOG_CLOSE_BASE_CLASS, DIALOG_CONTENT_BASE_CLASS, DIALOG_DESCRIPTION_BASE_CLASS,
  DIALOG_OVERLAY_BASE_CLASS, DIALOG_TITLE_BASE_CLASS,
};
#[cfg(feature = "dropdown")]
pub use dropdown::{
  dropdown_content_class, dropdown_group_class, dropdown_item_class, dropdown_label_class,
  dropdown_separator_class, DropdownContent, DropdownGroup, DropdownItem, DropdownLabel,
  DropdownPrimitiveConfig, DropdownSeparator, DROPDOWN_CONTENT_BASE_CLASS,
  DROPDOWN_GROUP_BASE_CLASS, DROPDOWN_ITEM_BASE_CLASS, DROPDOWN_LABEL_BASE_CLASS,
  DROPDOWN_SEPARATOR_BASE_CLASS,
};
#[cfg(feature = "input")]
pub use input::{input_class, Input, INPUT_BASE_CLASS};
#[cfg(feature = "label")]
pub use label::{label_class, Label, LABEL_BASE_CLASS};
#[cfg(feature = "popover")]
pub use popover::{
  popover_content_class, popover_description_class, popover_header_class, popover_title_class,
  OverlayAlign, OverlaySide, PopoverContent, PopoverDescription, PopoverHeader,
  PopoverPrimitiveConfig, PopoverTitle, POPOVER_CONTENT_BASE_CLASS,
  POPOVER_DESCRIPTION_BASE_CLASS, POPOVER_HEADER_BASE_CLASS, POPOVER_TITLE_BASE_CLASS,
};
#[cfg(feature = "select")]
pub use select::{
  select_content_class, select_group_class, select_item_class, select_label_class,
  select_separator_class, select_trigger_class, select_value_class, SelectContent, SelectGroup,
  SelectItem, SelectLabel, SelectPrimitiveConfig, SelectSeparator, SelectTrigger, SelectValue,
  SELECT_CONTENT_BASE_CLASS, SELECT_GROUP_BASE_CLASS, SELECT_ITEM_BASE_CLASS,
  SELECT_LABEL_BASE_CLASS, SELECT_SEPARATOR_BASE_CLASS, SELECT_TRIGGER_BASE_CLASS,
  SELECT_VALUE_BASE_CLASS,
};
#[cfg(feature = "separator")]
pub use separator::{
  separator_class, Separator, SeparatorOrientation, SEPARATOR_BASE_CLASS,
};
#[cfg(feature = "skeleton")]
pub use skeleton::{skeleton_class, Skeleton, SKELETON_BASE_CLASS};
#[cfg(feature = "textarea")]
pub use textarea::{textarea_class, Textarea, TEXTAREA_BASE_CLASS};
#[cfg(feature = "switch")]
pub mod switch;

#[cfg(feature = "tabs")]
pub mod tabs;

#[cfg(feature = "tooltip")]
pub mod tooltip;

#[cfg(feature = "switch")]
pub use switch::{switch_class, switch_thumb_class, Switch, SWITCH_BASE_CLASS, SWITCH_THUMB_BASE_CLASS};
#[cfg(feature = "tabs")]
pub use tabs::{
  tabs_content_class, tabs_list_class, tabs_trigger_class, TabsContent, TabsList, TabsTrigger,
  TABS_CONTENT_BASE_CLASS, TABS_LIST_BASE_CLASS, TABS_TRIGGER_BASE_CLASS,
};
#[cfg(feature = "tooltip")]
pub use tooltip::{
  tooltip_content_class, TooltipContent, TooltipPrimitiveConfig, TOOLTIP_CONTENT_BASE_CLASS,
};
pub use dioxus_ui_core::UiDensity;
