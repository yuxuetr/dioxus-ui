//! Styled Dioxus UI components.

#[cfg(feature = "accordion")]
pub mod accordion;

#[cfg(feature = "alert")]
pub mod alert;

#[cfg(feature = "alert-dialog")]
pub mod alert_dialog;

#[cfg(feature = "avatar")]
pub mod avatar;

#[cfg(feature = "badge")]
pub mod badge;

#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "card")]
pub mod card;

#[cfg(feature = "checkbox")]
pub mod checkbox;

#[cfg(feature = "context-menu")]
pub mod context_menu;

#[cfg(feature = "dialog")]
pub mod dialog;

#[cfg(feature = "drawer")]
pub mod drawer;

#[cfg(feature = "dropdown")]
pub mod dropdown;

#[cfg(feature = "hover-card")]
pub mod hover_card;

#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "label")]
pub mod label;

#[cfg(feature = "pagination")]
pub mod pagination;

#[cfg(feature = "popover")]
pub mod popover;

#[cfg(feature = "progress")]
pub mod progress;

#[cfg(feature = "radio-group")]
pub mod radio_group;

#[cfg(feature = "select")]
pub mod select;

#[cfg(feature = "separator")]
pub mod separator;

#[cfg(feature = "sheet")]
pub mod sheet;

#[cfg(feature = "skeleton")]
pub mod skeleton;

#[cfg(feature = "slider")]
pub mod slider;

#[cfg(feature = "spinner")]
pub mod spinner;

#[cfg(feature = "textarea")]
pub mod textarea;

#[cfg(feature = "table")]
pub mod table;

#[cfg(feature = "accordion")]
pub use accordion::{
  accordion_content_class, accordion_item_class, accordion_trigger_class, AccordionContent,
  AccordionItem, AccordionTrigger, ACCORDION_CONTENT_BASE_CLASS, ACCORDION_ITEM_BASE_CLASS,
  ACCORDION_TRIGGER_BASE_CLASS,
};
#[cfg(feature = "alert")]
pub use alert::{
  alert_class, alert_description_class, alert_title_class, Alert, AlertDescription, AlertTitle,
  AlertVariant, ALERT_BASE_CLASS, ALERT_DESCRIPTION_BASE_CLASS, ALERT_TITLE_BASE_CLASS,
};
#[cfg(feature = "alert-dialog")]
pub use alert_dialog::{
  alert_dialog_action_class, alert_dialog_cancel_class, alert_dialog_content_class,
  alert_dialog_description_class, alert_dialog_footer_class, alert_dialog_header_class,
  alert_dialog_overlay_class, alert_dialog_title_class, AlertDialogAction,
  AlertDialogActionVariant, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
  AlertDialogFooter, AlertDialogHeader, AlertDialogOverlay, AlertDialogTitle,
  DialogPrimitiveConfig as AlertDialogPrimitiveConfig,
  DismissBehavior as AlertDialogDismissBehavior, FocusReturn as AlertDialogFocusReturn,
  FocusStrategy as AlertDialogFocusStrategy, PortalTarget as AlertDialogPortalTarget,
  ALERT_DIALOG_ACTION_BASE_CLASS, ALERT_DIALOG_CANCEL_BASE_CLASS,
  ALERT_DIALOG_CONTENT_BASE_CLASS, ALERT_DIALOG_DESCRIPTION_BASE_CLASS,
  ALERT_DIALOG_FOOTER_BASE_CLASS, ALERT_DIALOG_HEADER_BASE_CLASS,
  ALERT_DIALOG_OVERLAY_BASE_CLASS, ALERT_DIALOG_TITLE_BASE_CLASS,
};
#[cfg(feature = "avatar")]
pub use avatar::{
  avatar_class, avatar_fallback_class, avatar_image_class, Avatar, AvatarFallback, AvatarImage,
  AVATAR_BASE_CLASS, AVATAR_FALLBACK_BASE_CLASS, AVATAR_IMAGE_BASE_CLASS,
};
#[cfg(feature = "badge")]
pub use badge::{badge_class, Badge, BadgeVariant, BADGE_BASE_CLASS};
#[cfg(feature = "button")]
pub use button::{button_class, Button, ButtonSize, ButtonVariant, BUTTON_BASE_CLASS};
#[cfg(feature = "card")]
pub use card::{
  card_class, card_content_class, card_description_class, card_footer_class, card_header_class,
  card_title_class, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle,
  CARD_BASE_CLASS, CARD_CONTENT_BASE_CLASS, CARD_DESCRIPTION_BASE_CLASS, CARD_FOOTER_BASE_CLASS,
  CARD_HEADER_BASE_CLASS, CARD_TITLE_BASE_CLASS,
};
#[cfg(feature = "checkbox")]
pub use checkbox::{checkbox_class, Checkbox, CHECKBOX_BASE_CLASS};
#[cfg(feature = "context-menu")]
pub use context_menu::{
  context_menu_content_class, context_menu_group_class, context_menu_item_class,
  context_menu_label_class, context_menu_separator_class, context_menu_shortcut_class,
  ContextMenuCheckboxItem, ContextMenuContent, ContextMenuGroup, ContextMenuItem,
  ContextMenuLabel, ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSeparator,
  ContextMenuShortcut, DropdownPrimitiveConfig as ContextMenuPrimitiveConfig,
  CONTEXT_MENU_CONTENT_BASE_CLASS, CONTEXT_MENU_GROUP_BASE_CLASS,
  CONTEXT_MENU_ITEM_BASE_CLASS, CONTEXT_MENU_ITEM_INSET_CLASS, CONTEXT_MENU_LABEL_BASE_CLASS,
  CONTEXT_MENU_SEPARATOR_BASE_CLASS, CONTEXT_MENU_SHORTCUT_BASE_CLASS,
};
#[cfg(feature = "dialog")]
pub use dialog::{
  dialog_close_class, dialog_content_class, dialog_description_class, dialog_overlay_class,
  dialog_title_class, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogTitle,
  DialogPrimitiveConfig, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
  DIALOG_CLOSE_BASE_CLASS, DIALOG_CONTENT_BASE_CLASS, DIALOG_DESCRIPTION_BASE_CLASS,
  DIALOG_OVERLAY_BASE_CLASS, DIALOG_TITLE_BASE_CLASS,
};
#[cfg(feature = "drawer")]
pub use drawer::{
  drawer_close_class, drawer_content_class, drawer_description_class, drawer_footer_class,
  drawer_header_class, drawer_overlay_class, drawer_title_class,
  DialogPrimitiveConfig as DrawerPrimitiveConfig, DismissBehavior as DrawerDismissBehavior,
  DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerOverlay,
  DrawerTitle, FocusReturn as DrawerFocusReturn, FocusStrategy as DrawerFocusStrategy,
  PortalTarget as DrawerPortalTarget, DRAWER_CLOSE_BASE_CLASS, DRAWER_CONTENT_BASE_CLASS,
  DRAWER_DESCRIPTION_BASE_CLASS, DRAWER_FOOTER_BASE_CLASS, DRAWER_HEADER_BASE_CLASS,
  DRAWER_OVERLAY_BASE_CLASS, DRAWER_TITLE_BASE_CLASS,
};
#[cfg(feature = "dropdown")]
pub use dropdown::{
  dropdown_content_class, dropdown_group_class, dropdown_item_class, dropdown_label_class,
  dropdown_separator_class, DropdownContent, DropdownGroup, DropdownItem, DropdownLabel,
  DropdownPrimitiveConfig, DropdownSeparator, DROPDOWN_CONTENT_BASE_CLASS,
  DROPDOWN_GROUP_BASE_CLASS, DROPDOWN_ITEM_BASE_CLASS, DROPDOWN_LABEL_BASE_CLASS,
  DROPDOWN_SEPARATOR_BASE_CLASS,
};
#[cfg(feature = "hover-card")]
pub use hover_card::{
  hover_card_align_attribute, hover_card_content_class, hover_card_description_class,
  hover_card_header_class, hover_card_side_attribute, hover_card_title_class,
  HoverCardContent, HoverCardDescription, HoverCardHeader, HoverCardTitle,
  OverlayAlign as HoverCardAlign, OverlaySide as HoverCardSide,
  PopoverPrimitiveConfig as HoverCardPrimitiveConfig, HOVER_CARD_CONTENT_BASE_CLASS,
  HOVER_CARD_DESCRIPTION_BASE_CLASS, HOVER_CARD_HEADER_BASE_CLASS,
  HOVER_CARD_TITLE_BASE_CLASS,
};
#[cfg(feature = "input")]
pub use input::{input_class, Input, INPUT_BASE_CLASS};
#[cfg(feature = "label")]
pub use label::{label_class, Label, LABEL_BASE_CLASS};
#[cfg(feature = "pagination")]
pub use pagination::{
  pagination_class, pagination_content_class, pagination_ellipsis_class, pagination_item_class,
  pagination_link_class, Pagination, PaginationContent, PaginationEllipsis, PaginationItem,
  PaginationLink, PaginationNext, PaginationPrevious, PAGINATION_BASE_CLASS,
  PAGINATION_CONTENT_BASE_CLASS, PAGINATION_ELLIPSIS_BASE_CLASS, PAGINATION_ITEM_BASE_CLASS,
  PAGINATION_LINK_ACTIVE_CLASS, PAGINATION_LINK_BASE_CLASS, PAGINATION_LINK_DISABLED_CLASS,
};
#[cfg(feature = "popover")]
pub use popover::{
  popover_content_class, popover_description_class, popover_header_class, popover_title_class,
  OverlayAlign, OverlaySide, PopoverContent, PopoverDescription, PopoverHeader,
  PopoverPrimitiveConfig, PopoverTitle, POPOVER_CONTENT_BASE_CLASS,
  POPOVER_DESCRIPTION_BASE_CLASS, POPOVER_HEADER_BASE_CLASS, POPOVER_TITLE_BASE_CLASS,
};
#[cfg(feature = "progress")]
pub use progress::{
  progress_class, progress_indicator_class, progress_percent, Progress,
  PROGRESS_BASE_CLASS, PROGRESS_INDICATOR_BASE_CLASS,
};
#[cfg(any(feature = "radio-group", feature = "toggle-group"))]
pub use dioxus_ui_primitives::{FocusMove, NavigationOrientation, RovingFocusItem};
#[cfg(feature = "radio-group")]
pub use radio_group::{
  radio_group_class, radio_group_focus_state, radio_group_indicator_class,
  radio_group_item_class, radio_group_item_tabindex, radio_group_move_value,
  radio_group_orientation_attribute, RadioGroup, RadioGroupItem, RADIO_GROUP_BASE_CLASS,
  RADIO_GROUP_INDICATOR_BASE_CLASS, RADIO_GROUP_ITEM_BASE_CLASS,
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
#[cfg(feature = "sheet")]
pub use sheet::{
  sheet_close_class, sheet_content_class, sheet_description_class, sheet_footer_class,
  sheet_header_class, sheet_overlay_class, sheet_title_class,
  DialogPrimitiveConfig as SheetPrimitiveConfig, DismissBehavior as SheetDismissBehavior,
  FocusReturn as SheetFocusReturn, FocusStrategy as SheetFocusStrategy,
  PortalTarget as SheetPortalTarget, SheetClose, SheetContent, SheetDescription, SheetFooter,
  SheetHeader, SheetOverlay, SheetSide, SheetTitle, SHEET_CLOSE_BASE_CLASS,
  SHEET_CONTENT_BASE_CLASS, SHEET_DESCRIPTION_BASE_CLASS, SHEET_FOOTER_BASE_CLASS,
  SHEET_HEADER_BASE_CLASS, SHEET_OVERLAY_BASE_CLASS, SHEET_TITLE_BASE_CLASS,
};
#[cfg(feature = "skeleton")]
pub use skeleton::{skeleton_class, Skeleton, SKELETON_BASE_CLASS};
#[cfg(feature = "slider")]
pub use slider::{
  slider_aria_attributes, slider_percent, slider_range_class, slider_range_style,
  slider_root_class, slider_state, slider_thumb_class, slider_thumb_style, slider_track_class,
  Slider, SLIDER_RANGE_BASE_CLASS, SLIDER_ROOT_BASE_CLASS, SLIDER_THUMB_BASE_CLASS,
  SLIDER_TRACK_BASE_CLASS,
};
#[cfg(feature = "slider")]
pub use dioxus_ui_primitives::{SliderAriaAttributes, SliderKeyMove, SliderState};
#[cfg(feature = "spinner")]
pub use spinner::{spinner_class, Spinner, SpinnerSize, SPINNER_BASE_CLASS};
#[cfg(feature = "textarea")]
pub use textarea::{textarea_class, Textarea, TEXTAREA_BASE_CLASS};
#[cfg(feature = "switch")]
pub mod switch;

#[cfg(feature = "tabs")]
pub mod tabs;

#[cfg(feature = "toggle")]
pub mod toggle;

#[cfg(feature = "toggle-group")]
pub mod toggle_group;

#[cfg(feature = "tooltip")]
pub mod tooltip;

#[cfg(feature = "switch")]
pub use switch::{switch_class, switch_thumb_class, Switch, SWITCH_BASE_CLASS, SWITCH_THUMB_BASE_CLASS};
#[cfg(feature = "table")]
pub use table::{
  table_body_class, table_caption_class, table_cell_class, table_class, table_container_class,
  table_footer_class, table_head_class, table_header_class, table_row_class, Table, TableBody,
  TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow, TABLE_BASE_CLASS,
  TABLE_BODY_BASE_CLASS, TABLE_CAPTION_BASE_CLASS, TABLE_CELL_BASE_CLASS,
  TABLE_CONTAINER_BASE_CLASS, TABLE_FOOTER_BASE_CLASS, TABLE_HEADER_BASE_CLASS,
  TABLE_HEAD_BASE_CLASS, TABLE_ROW_BASE_CLASS,
};
#[cfg(feature = "tabs")]
pub use tabs::{
  tabs_content_class, tabs_list_class, tabs_trigger_class, TabsContent, TabsList, TabsTrigger,
  TABS_CONTENT_BASE_CLASS, TABS_LIST_BASE_CLASS, TABS_TRIGGER_BASE_CLASS,
};
#[cfg(feature = "toggle")]
pub use toggle::{toggle_class, Toggle, ToggleSize, ToggleVariant, TOGGLE_BASE_CLASS};
#[cfg(feature = "toggle-group")]
pub use toggle_group::{
  toggle_group_class, toggle_group_focus_state, toggle_group_item_class,
  toggle_group_item_tabindex, toggle_group_move_value, toggle_group_multiple_selection,
  toggle_group_orientation_attribute, toggle_group_single_selection, ToggleGroup,
  ToggleGroupItem, ToggleGroupType, TOGGLE_GROUP_BASE_CLASS, TOGGLE_GROUP_ITEM_BASE_CLASS,
};
#[cfg(feature = "tooltip")]
pub use tooltip::{
  tooltip_content_class, TooltipContent, TooltipPrimitiveConfig, TOOLTIP_CONTENT_BASE_CLASS,
};
pub use dioxus_ui_core::UiDensity;
