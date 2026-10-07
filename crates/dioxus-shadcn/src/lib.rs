//! Styled Dioxus UI components.

// Each module is copied as a source-copy template that must compile in
// edition 2021 apps, which have no `if let` chains, and the template parity
// test (RFC 0066) keeps the module and its template identical.
#![allow(clippy::collapsible_if)]
// Every public item says what it is for (RFC 0079).
#![deny(missing_docs)]

#[cfg(feature = "attachment")]
pub mod attachment;

#[cfg(feature = "accordion")]
pub mod accordion;

#[cfg(feature = "alert")]
pub mod alert;

#[cfg(feature = "alert-dialog")]
pub mod alert_dialog;

#[cfg(feature = "aspect-ratio")]
pub mod aspect_ratio;

#[cfg(feature = "avatar")]
pub mod avatar;

#[cfg(feature = "badge")]
pub mod badge;

#[cfg(feature = "breadcrumb")]
pub mod breadcrumb;

#[cfg(feature = "bubble")]
pub mod bubble;

#[cfg(feature = "button")]
pub mod button;

#[cfg(feature = "button-group")]
pub mod button_group;

#[cfg(feature = "calendar")]
pub mod calendar;

#[cfg(feature = "carousel")]
pub mod carousel;

#[cfg(feature = "card")]
pub mod card;

#[cfg(feature = "checkbox")]
pub mod checkbox;

#[cfg(feature = "chart")]
pub mod chart;

#[cfg(feature = "collapsible")]
pub mod collapsible;

#[cfg(feature = "command")]
pub mod command;

#[cfg(feature = "combobox")]
pub mod combobox;

#[cfg(feature = "context-menu")]
pub mod context_menu;

#[cfg(feature = "data-table")]
pub mod data_table;

#[cfg(feature = "date-picker")]
pub mod date_picker;

#[cfg(feature = "dialog")]
pub mod dialog;

#[cfg(any(
  feature = "accordion",
  feature = "alert-dialog",
  feature = "checkbox",
  feature = "collapsible",
  feature = "combobox",
  feature = "command",
  feature = "context-menu",
  feature = "date-picker",
  feature = "dialog",
  feature = "drawer",
  feature = "dropdown",
  feature = "fab",
  feature = "hover-card",
  feature = "input-otp",
  feature = "menu",
  feature = "menubar",
  feature = "navigation-menu",
  feature = "popover",
  feature = "radio-group",
  feature = "rating",
  feature = "resizable",
  feature = "select",
  feature = "sheet",
  feature = "sidebar",
  feature = "slider",
  feature = "sonner",
  feature = "tabs",
  feature = "theme-controller",
  feature = "toast",
  feature = "toggle-group",
  feature = "tooltip"
))]
mod element_id;

#[cfg(any(
  feature = "accordion",
  feature = "alert-dialog",
  feature = "carousel",
  feature = "collapsible",
  feature = "combobox",
  feature = "command",
  feature = "context-menu",
  feature = "date-picker",
  feature = "dialog",
  feature = "drawer",
  feature = "dropdown",
  feature = "fab",
  feature = "hover-card",
  feature = "input-otp",
  feature = "menu",
  feature = "menubar",
  feature = "navigation-menu",
  feature = "popover",
  feature = "radio-group",
  feature = "select",
  feature = "sheet",
  feature = "sidebar",
  feature = "tabs",
  feature = "toggle-group",
  feature = "tooltip"
))]
mod root_state;

#[cfg(any(
  feature = "alert-dialog",
  feature = "collapsible",
  feature = "context-menu",
  feature = "dialog",
  feature = "drawer",
  feature = "dropdown",
  feature = "fab",
  feature = "hover-card",
  feature = "menubar",
  feature = "popover",
  feature = "sheet",
  feature = "tooltip"
))]
mod overlay_root;

#[cfg(any(
  feature = "alert-dialog",
  feature = "date-picker",
  feature = "dialog",
  feature = "drawer",
  feature = "sheet",
  feature = "sidebar"
))]
mod modal_focus;

#[cfg(any(
  feature = "alert-dialog",
  feature = "dialog",
  feature = "drawer",
  feature = "popover",
  feature = "sheet"
))]
mod dialog_labels;

#[cfg(any(
  feature = "combobox",
  feature = "context-menu",
  feature = "date-picker",
  feature = "dropdown",
  feature = "hover-card",
  feature = "menubar",
  feature = "popover",
  feature = "select",
  feature = "tooltip"
))]
mod anchored_overlay;

#[cfg(any(
  feature = "combobox",
  feature = "command",
  feature = "context-menu",
  feature = "dropdown",
  feature = "menubar",
  feature = "select"
))]
mod listbox;

#[cfg(any(
  feature = "accordion",
  feature = "combobox",
  feature = "select",
  feature = "toggle-group"
))]
mod choice;

#[cfg(any(feature = "context-menu", feature = "dropdown", feature = "menubar"))]
mod menu_marks;

#[cfg(any(feature = "context-menu", feature = "dropdown", feature = "menubar"))]
mod menu_radio;

#[cfg(any(feature = "context-menu", feature = "dropdown", feature = "menubar"))]
mod menu_sub;

#[cfg(feature = "sidebar")]
mod media_query;

#[cfg(any(feature = "sonner", feature = "toast"))]
mod dismiss_timer;

#[cfg(any(
  feature = "alert-dialog",
  feature = "carousel",
  feature = "collapsible",
  feature = "combobox",
  feature = "dialog",
  feature = "drawer",
  feature = "dropdown",
  feature = "message-scroller",
  feature = "pagination",
  feature = "popover",
  feature = "scroll-area",
  feature = "select",
  feature = "sheet",
  feature = "sidebar"
))]
mod default_attribute;

#[cfg(any(feature = "hover-card", feature = "tooltip"))]
mod hover_open;

#[cfg(any(
  feature = "accordion",
  feature = "radio-group",
  feature = "tabs",
  feature = "toggle-group"
))]
mod roving_group;

#[cfg(feature = "direction")]
pub mod direction;

#[cfg(feature = "drawer")]
pub mod drawer;

#[cfg(feature = "dropdown")]
pub mod dropdown;

#[cfg(feature = "empty")]
pub mod empty;

#[cfg(feature = "field")]
pub mod field;

#[cfg(feature = "hover-card")]
pub mod hover_card;

#[cfg(feature = "input")]
pub mod input;

#[cfg(feature = "input-group")]
pub mod input_group;

#[cfg(feature = "input-otp")]
pub mod input_otp;

#[cfg(feature = "item")]
pub mod item;

#[cfg(feature = "kbd")]
pub mod kbd;

#[cfg(feature = "label")]
pub mod label;

#[cfg(feature = "marker")]
pub mod marker;

#[cfg(feature = "menubar")]
pub mod menubar;

#[cfg(feature = "message")]
pub mod message;

#[cfg(feature = "message-scroller")]
pub mod message_scroller;

#[cfg(feature = "native-select")]
pub mod native_select;

#[cfg(feature = "navigation-menu")]
pub mod navigation_menu;

#[cfg(feature = "pagination")]
pub mod pagination;

#[cfg(feature = "popover")]
pub mod popover;

#[cfg(feature = "progress")]
pub mod progress;

#[cfg(feature = "radio-group")]
pub mod radio_group;

#[cfg(feature = "resizable")]
pub mod resizable;

#[cfg(feature = "select")]
pub mod select;

#[cfg(feature = "scroll-area")]
pub mod scroll_area;

#[cfg(feature = "separator")]
pub mod separator;

#[cfg(feature = "sheet")]
pub mod sheet;

#[cfg(feature = "sidebar")]
pub mod sidebar;

#[cfg(feature = "skeleton")]
pub mod skeleton;

#[cfg(feature = "slider")]
pub mod slider;

#[cfg(feature = "sonner")]
pub mod sonner;

#[cfg(feature = "spinner")]
pub mod spinner;

#[cfg(feature = "textarea")]
pub mod textarea;

#[cfg(feature = "table")]
pub mod table;

#[cfg(feature = "accordion")]
pub use accordion::{
  Accordion, AccordionContent, AccordionItem, AccordionTrigger, accordion_content_class,
  accordion_item_class, accordion_trigger_class,
};
#[cfg(feature = "alert")]
pub use alert::{
  Alert, AlertDescription, AlertTitle, AlertVariant, alert_class, alert_description_class,
  alert_title_class,
};
#[cfg(feature = "alert-dialog")]
pub use alert_dialog::{
  AlertDialog, AlertDialogAction, AlertDialogActionVariant, AlertDialogCancel, AlertDialogContent,
  AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogOverlay,
  AlertDialogTitle, AlertDialogTrigger, DialogPrimitiveConfig as AlertDialogPrimitiveConfig,
  DismissBehavior as AlertDialogDismissBehavior, FocusReturn as AlertDialogFocusReturn,
  FocusStrategy as AlertDialogFocusStrategy, PortalTarget as AlertDialogPortalTarget,
  alert_dialog_action_class, alert_dialog_content_class, alert_dialog_overlay_class,
};
#[cfg(feature = "aspect-ratio")]
pub use aspect_ratio::{AspectRatio, aspect_ratio_class, aspect_ratio_style};
#[cfg(feature = "attachment")]
pub use attachment::{
  Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription,
  AttachmentGroup, AttachmentMedia, AttachmentMediaVariant, AttachmentOrientation, AttachmentSize,
  AttachmentState, AttachmentTitle, AttachmentTrigger, attachment_action_class,
  attachment_actions_class, attachment_class, attachment_content_class,
  attachment_description_class, attachment_group_class, attachment_media_class,
  attachment_title_class, attachment_trigger_class,
};
#[cfg(feature = "avatar")]
pub use avatar::{
  Avatar, AvatarFallback, AvatarImage, avatar_class, avatar_fallback_class, avatar_image_class,
};
#[cfg(feature = "badge")]
pub use badge::{Badge, BadgeVariant, badge_class};
#[cfg(feature = "breadcrumb")]
pub use breadcrumb::{
  Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
  BreadcrumbSeparator, breadcrumb_class, breadcrumb_ellipsis_class, breadcrumb_item_class,
  breadcrumb_link_class, breadcrumb_list_class, breadcrumb_page_class, breadcrumb_separator_class,
};
#[cfg(feature = "bubble")]
pub use bubble::{
  Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleReactionAlign, BubbleReactionSide,
  BubbleReactions, BubbleVariant, bubble_class, bubble_content_class, bubble_group_class,
  bubble_reactions_class,
};
#[cfg(feature = "button")]
pub use button::{Button, ButtonSize, ButtonVariant, button_class};
#[cfg(feature = "button-group")]
pub use button_group::{
  ButtonGroup, ButtonGroupItem, ButtonGroupOrientation, button_group_class, button_group_item_class,
};
#[cfg(feature = "calendar")]
pub use calendar::{
  Calendar, CalendarBody, CalendarCaption, CalendarDate, CalendarDay, CalendarGrid, CalendarHead,
  CalendarHeadCell, CalendarHeader, CalendarKeyMove, CalendarMonth, CalendarMonthGrid, CalendarNav,
  CalendarNavButton, CalendarNavDirection, CalendarPrimitiveDay, CalendarRangeState, CalendarRow,
  CalendarWeekday, calendar_key_move, calendar_month_grid, calendar_move_date,
  calendar_range_state, days_in_month, is_leap_year,
};
#[cfg(feature = "card")]
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, card_class};
#[cfg(feature = "carousel")]
pub use carousel::{
  Carousel, CarouselContent, CarouselIndicator, CarouselItem, CarouselNext, CarouselOrientation,
  CarouselPrevious, CarouselState, CarouselStep, CarouselViewport, carousel_can_go_next,
  carousel_can_go_previous, carousel_next, carousel_previous,
};
#[cfg(feature = "chart")]
pub use chart::{
  CHART_COLOR_CLASSES, ChartAreaSeries, ChartBarSeries, ChartColorToken, ChartDescription,
  ChartDomain, ChartFallbackRow, ChartFallbackTable, ChartLegend, ChartLineSeries, ChartPieSeries,
  ChartPoint, ChartRoot, ChartScale, ChartSeries, ChartSlice, ChartSvg, ChartTitle,
  ChartTooltipSlot, chart_color_attribute, chart_color_class, chart_domain, chart_fallback_rows,
  chart_scale_value, chart_series_label, chart_series_x_domain, chart_series_y_domain,
  chart_summary, chart_value_label, chart_view_box,
};
#[cfg(feature = "checkbox")]
pub use checkbox::{Checkbox, checkbox_class};
#[cfg(feature = "collapsible")]
pub use collapsible::{
  Collapsible, CollapsibleContent, CollapsibleTrigger, collapsible_class,
  collapsible_content_class, collapsible_trigger_class,
};
#[cfg(feature = "combobox")]
pub use combobox::{
  ActiveDescendantState as ComboboxActiveDescendantState, Combobox, ComboboxContent, ComboboxEmpty,
  ComboboxGroup, ComboboxInput, ComboboxItem, ComboboxList, ComboboxStatus, ComboboxTrigger,
  ComboboxValue, DismissBehavior as ComboboxDismissBehavior, OverlayAlign as ComboboxAlign,
  OverlaySide as ComboboxSide, PopoverPrimitiveConfig as ComboboxPrimitiveConfig,
  combobox_input_class, combobox_item_class, combobox_trigger_class,
};
#[cfg(feature = "command")]
pub use command::{
  ActiveDescendantState, Command, CommandEmpty, CommandGroup, CommandInput, CommandItem,
  CommandLabel, CommandList, CommandSeparator, CommandShortcut, CommandStatus, command_class,
  command_input_class, command_item_class, command_matches,
};
#[cfg(feature = "context-menu")]
pub use context_menu::{
  ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuGroup, ContextMenuItem,
  ContextMenuLabel, ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSeparator,
  ContextMenuShortcut, ContextMenuSub, ContextMenuSubContent, ContextMenuSubTrigger,
  ContextMenuTrigger, DismissBehavior as ContextMenuDismissBehavior,
  DropdownPrimitiveConfig as ContextMenuPrimitiveConfig, OverlayAlign as ContextMenuAlign,
  OverlaySide as ContextMenuSide, context_menu_checkbox_item_class, context_menu_content_class,
  context_menu_item_class, context_menu_radio_item_class, context_menu_sub_trigger_class,
};
#[cfg(feature = "data-table")]
pub use data_table::{
  DataTable, DataTableCell, DataTableColumnState, DataTableContainer, DataTableEmpty,
  DataTableHeaderCell, DataTableLoading, DataTablePagination, DataTablePaginationState,
  DataTableRow, DataTableSelectedCount, DataTableSelectionState, DataTableSortDirection,
  DataTableSortState, DataTableToolbar, data_table_is_column_visible, data_table_page_count,
  data_table_page_window, data_table_toggle_all_rows, data_table_toggle_column,
  data_table_toggle_row, data_table_toggle_sort,
};
#[cfg(feature = "date-picker")]
pub use date_picker::{
  DateOrder, DatePicker, DatePickerContent, DatePickerInput, DatePickerTrigger, DatePickerValue,
  DismissBehavior as DatePickerDismissBehavior, OverlayAlign as DatePickerAlign,
  OverlaySide as DatePickerSide, PopoverPrimitiveConfig as DatePickerPrimitiveConfig,
  date_picker_content_class, date_picker_input_class, date_picker_trigger_class,
  date_picker_value_class, format_date, parse_date,
};
#[cfg(feature = "dialog")]
pub use dialog::{
  Dialog, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogPrimitiveConfig,
  DialogTitle, DialogTrigger, DismissBehavior, FocusReturn, FocusStrategy, PortalTarget,
  dialog_content_class, dialog_overlay_class,
};
#[cfg(any(feature = "radio-group", feature = "toggle-group"))]
pub use dioxus_shadcn_primitives::{FocusMove, NavigationOrientation, RovingFocusItem};
#[cfg(feature = "slider")]
pub use dioxus_shadcn_primitives::{SliderKeyMove, SliderState};
#[cfg(feature = "direction")]
pub use direction::{Direction, TextDirection, direction_class};
#[cfg(feature = "drawer")]
pub use drawer::{
  DialogPrimitiveConfig as DrawerPrimitiveConfig, DismissBehavior as DrawerDismissBehavior, Drawer,
  DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerOverlay,
  DrawerTitle, DrawerTrigger, FocusReturn as DrawerFocusReturn,
  FocusStrategy as DrawerFocusStrategy, PortalTarget as DrawerPortalTarget, drawer_content_class,
  drawer_overlay_class,
};
#[cfg(feature = "dropdown")]
pub use dropdown::{
  DismissBehavior as DropdownDismissBehavior, Dropdown, DropdownCheckboxItem, DropdownContent,
  DropdownGroup, DropdownItem, DropdownLabel, DropdownPrimitiveConfig, DropdownRadioGroup,
  DropdownRadioItem, DropdownSeparator, DropdownShortcut, DropdownSub, DropdownSubContent,
  DropdownSubTrigger, DropdownTrigger, OverlayAlign as DropdownAlign, OverlaySide as DropdownSide,
  dropdown_checkbox_item_class, dropdown_content_class, dropdown_inset_item_class,
  dropdown_item_class, dropdown_radio_item_class, dropdown_shortcut_class,
  dropdown_sub_trigger_class,
};
#[cfg(feature = "empty")]
pub use empty::{
  Empty, EmptyActions, EmptyContent, EmptyDescription, EmptyHeader, EmptyTitle,
  empty_actions_class, empty_class, empty_content_class, empty_description_class,
  empty_header_class, empty_title_class,
};
#[cfg(feature = "field")]
pub use field::{
  Field, FieldDescription, FieldError, FieldGroup, FieldLabel, field_class,
  field_description_class, field_error_class, field_group_class, field_label_class,
};
#[cfg(feature = "hover-card")]
pub use hover_card::{
  DismissBehavior as HoverCardDismissBehavior, HoverCard, HoverCardContent, HoverCardDescription,
  HoverCardHeader, HoverCardTitle, HoverCardTrigger, OverlayAlign as HoverCardAlign,
  OverlaySide as HoverCardSide, PopoverPrimitiveConfig as HoverCardPrimitiveConfig,
  hover_card_content_class,
};
#[cfg(feature = "input")]
pub use input::{Input, input_class};
#[cfg(feature = "input-group")]
pub use input_group::{
  InputGroup, InputGroupAction, InputGroupAddon, InputGroupAddonPosition, InputGroupControl,
  input_group_action_class, input_group_addon_class, input_group_class, input_group_control_class,
};
#[cfg(feature = "input-otp")]
pub use input_otp::{
  InputOtp, InputOtpGroup, InputOtpHiddenInput, InputOtpInputMode, InputOtpSeparator, InputOtpSlot,
  OtpSlotState, input_otp_class, input_otp_group_class, input_otp_hidden_input_class,
  input_otp_separator_class, input_otp_slot_class, otp_apply_paste, otp_apply_paste_filtered,
  otp_clamp_value, otp_delete_char, otp_insert_char, otp_insert_char_filtered, otp_is_complete,
  otp_next_index, otp_previous_index, otp_slots, otp_slots_with_disabled,
};
#[cfg(feature = "item")]
pub use item::{
  Item, ItemActions, ItemContent, ItemDescription, ItemMedia, ItemTitle, item_actions_class,
  item_class, item_content_class, item_description_class, item_media_class, item_title_class,
};
#[cfg(feature = "kbd")]
pub use kbd::{Kbd, KbdSize, kbd_class};
#[cfg(feature = "label")]
pub use label::{Label, label_class};
#[cfg(feature = "marker")]
pub use marker::{
  Marker, MarkerContent, MarkerIcon, MarkerVariant, marker_class, marker_content_class,
  marker_icon_class,
};
#[cfg(feature = "menubar")]
pub use menubar::{
  DismissBehavior as MenubarDismissBehavior, DropdownPrimitiveConfig as MenubarPrimitiveConfig,
  Menubar, MenubarCheckboxItem, MenubarContent, MenubarItem, MenubarLabel, MenubarMenu,
  MenubarRadioGroup, MenubarRadioItem, MenubarSeparator, MenubarShortcut, MenubarSub,
  MenubarSubContent, MenubarSubTrigger, MenubarTrigger, OverlayAlign as MenubarAlign,
  OverlaySide as MenubarSide, menubar_checkbox_item_class, menubar_class, menubar_item_class,
  menubar_radio_item_class, menubar_sub_trigger_class, menubar_trigger_class,
};
#[cfg(feature = "message")]
pub use message::{
  Message, MessageAlign, MessageAvatar, MessageContent, MessageFooter, MessageGroup, MessageHeader,
  message_avatar_class, message_class, message_content_class, message_footer_class,
  message_group_class, message_header_class,
};
#[cfg(feature = "message-scroller")]
pub use message_scroller::{
  MessageScroller, MessageScrollerBottomAnchor, MessageScrollerContent, MessageScrollerEvent,
  MessageScrollerIntent, MessageScrollerJumpButton, MessageScrollerMetrics,
  MessageScrollerUnreadMarker, MessageScrollerViewport, message_scroller_bottom_anchor_class,
  message_scroller_class, message_scroller_content_class, message_scroller_distance_to_bottom,
  message_scroller_is_at_bottom, message_scroller_jump_button_class, message_scroller_next_intent,
  message_scroller_should_follow, message_scroller_show_unread_marker,
  message_scroller_unread_marker_class, message_scroller_viewport_class,
};
#[cfg(feature = "native-select")]
pub use native_select::{
  NativeSelect, NativeSelectGroup, NativeSelectOption, native_select_class,
  native_select_group_class, native_select_option_class,
};
#[cfg(feature = "navigation-menu")]
pub use navigation_menu::{
  NavigationMenu, NavigationMenuContent, NavigationMenuIndicator, NavigationMenuItem,
  NavigationMenuLink, NavigationMenuList, NavigationMenuOrientation, NavigationMenuTrigger,
  NavigationMenuViewport, PopoverPrimitiveConfig as NavigationMenuPrimitiveConfig,
  navigation_menu_class, navigation_menu_link_class, navigation_menu_trigger_class,
};
#[cfg(feature = "pagination")]
pub use pagination::{
  Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
  PaginationNext, PaginationPrevious, PaginationRangeItem, pagination_link_class, pagination_range,
};
#[cfg(feature = "popover")]
pub use popover::{
  DismissBehavior as PopoverDismissBehavior, OverlayAlign, OverlaySide, Popover, PopoverContent,
  PopoverDescription, PopoverHeader, PopoverPrimitiveConfig, PopoverTitle, PopoverTrigger,
  popover_content_class,
};
#[cfg(feature = "progress")]
pub use progress::{Progress, progress_class, progress_indicator_class};
#[cfg(feature = "radio-group")]
pub use radio_group::{
  RadioGroup, RadioGroupItem, radio_group_class, radio_group_item_class, radio_group_item_tabindex,
  radio_group_move_value,
};
#[cfg(feature = "resizable")]
pub use resizable::{
  LayoutOrientation, ResizableHandle, ResizablePanel, ResizablePanelGroup, ResizablePanelState,
  resizable_clamp, resizable_resize_pair,
};
#[cfg(feature = "scroll-area")]
pub use scroll_area::{
  ScrollArea, ScrollAreaContent, ScrollAreaCorner, ScrollAreaOrientation, ScrollAreaScrollbar,
  ScrollAreaThumb, ScrollAreaViewport, scroll_area_orientation_attribute,
};
#[cfg(feature = "select")]
pub use select::{
  DismissBehavior as SelectDismissBehavior, OverlayAlign as SelectAlign, OverlaySide as SelectSide,
  Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectPrimitiveConfig,
  SelectSeparator, SelectTrigger, SelectValue, select_item_class, select_trigger_class,
};
#[cfg(feature = "separator")]
pub use separator::{Separator, SeparatorOrientation, separator_class};
#[cfg(feature = "sheet")]
pub use sheet::{
  DialogPrimitiveConfig as SheetPrimitiveConfig, DismissBehavior as SheetDismissBehavior,
  FocusReturn as SheetFocusReturn, FocusStrategy as SheetFocusStrategy,
  PortalTarget as SheetPortalTarget, Sheet, SheetClose, SheetContent, SheetDescription,
  SheetFooter, SheetHeader, SheetOverlay, SheetSide, SheetTitle, SheetTrigger, sheet_content_class,
  sheet_overlay_class,
};
#[cfg(feature = "sidebar")]
pub use sidebar::{
  SIDEBAR_MOBILE_QUERY, Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
  SidebarHeader, SidebarItem, SidebarProvider, SidebarRail, SidebarSide, SidebarState,
  SidebarTrigger, sidebar_mobile_class, sidebar_mobile_panel_class, sidebar_overlay_class,
  sidebar_toggle,
};
#[cfg(feature = "skeleton")]
pub use skeleton::{Skeleton, skeleton_class};
#[cfg(feature = "slider")]
pub use slider::{RangeSlider, Slider, SliderOrientation, slider_key_move};
#[cfg(feature = "sonner")]
pub use sonner::{
  SonnerAction, SonnerClose, SonnerContent, SonnerDescription, SonnerDismissReason, SonnerIcon,
  SonnerItem, SonnerPlacement, SonnerQueue, SonnerTitle, SonnerToast, SonnerVariant,
  SonnerViewport, sonner_dismiss_reason_attribute, sonner_is_expired, sonner_queue_dismiss,
  sonner_queue_push,
};
#[cfg(feature = "spinner")]
pub use spinner::{Spinner, SpinnerSize, spinner_class};
#[cfg(feature = "textarea")]
pub use textarea::{Textarea, textarea_class};
#[cfg(feature = "switch")]
pub mod switch;

#[cfg(feature = "tabs")]
pub mod tabs;

#[cfg(feature = "toggle")]
pub mod toggle;

#[cfg(feature = "toggle-group")]
pub mod toggle_group;

#[cfg(feature = "toast")]
pub mod toast;

#[cfg(feature = "tooltip")]
pub mod tooltip;

#[cfg(feature = "typography")]
pub mod typography;

pub mod density;
pub use density::{DensityProvider, density_control_class, density_hit_area_class, use_density};
pub use dioxus_shadcn_core::UiDensity;
#[cfg(feature = "fab")]
pub mod fab;

#[cfg(feature = "theme-controller")]
pub mod theme_controller;

#[cfg(feature = "menu")]
pub mod menu;

#[cfg(feature = "mockup")]
pub mod mockup;

#[cfg(feature = "dock")]
pub mod dock;

#[cfg(feature = "swap")]
pub mod swap;

#[cfg(feature = "file-input")]
pub mod file_input;

#[cfg(feature = "tags-input")]
pub mod tags_input;

#[cfg(feature = "number-input")]
pub mod number_input;

#[cfg(feature = "rating")]
pub mod rating;

#[cfg(feature = "diff")]
pub mod diff;

#[cfg(feature = "countdown")]
pub mod countdown;

#[cfg(feature = "radial-progress")]
pub mod radial_progress;

#[cfg(feature = "status")]
pub mod status;

#[cfg(feature = "indicator")]
pub mod indicator;

#[cfg(feature = "steps")]
pub mod steps;

#[cfg(feature = "timeline")]
pub mod timeline;

#[cfg(feature = "stat")]
pub mod stat;

#[cfg(feature = "switch")]
pub use switch::{Switch, switch_class, switch_thumb_class};
#[cfg(feature = "table")]
pub use table::{
  Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
  table_class,
};
#[cfg(feature = "tabs")]
pub use tabs::{
  Tabs, TabsActivation, TabsContent, TabsList, TabsOrientation, TabsTrigger, tabs_class,
  tabs_content_class, tabs_list_class, tabs_trigger_class,
};
#[cfg(feature = "toast")]
pub use toast::{
  ToastAction, ToastClose, ToastDescription, ToastDismissReason, ToastItem, ToastPlacement,
  ToastQueue, ToastRoot, ToastTitle, ToastVariant, ToastViewport, toast_dismiss_reason_attribute,
  toast_is_expired, toast_placement_attribute, toast_queue_dismiss, toast_queue_limit,
  toast_queue_push, toast_variant_attribute,
};
#[cfg(feature = "toggle")]
pub use toggle::{Toggle, ToggleSize, ToggleVariant, toggle_class};
#[cfg(feature = "toggle-group")]
pub use toggle_group::{
  ToggleGroup, ToggleGroupItem, ToggleGroupType, toggle_group_class, toggle_group_item_class,
};
#[cfg(feature = "tooltip")]
pub use tooltip::{
  DismissBehavior as TooltipDismissBehavior, OverlayAlign as TooltipAlign,
  OverlaySide as TooltipSide, Tooltip, TooltipContent, TooltipPrimitiveConfig, TooltipTrigger,
  tooltip_content_class,
};
#[cfg(feature = "typography")]
pub use typography::{
  TypographyBlockquote, TypographyH1, TypographyH2, TypographyH3, TypographyInlineCode,
  TypographyLead, TypographyMuted, TypographyP, TypographyProse,
};

#[cfg(feature = "stat")]
pub use stat::{
  Stat, StatDescription, StatFigure, StatGroup, StatGroupOrientation, StatTitle, StatValue,
  stat_class, stat_description_class, stat_figure_class, stat_group_class, stat_title_class,
  stat_value_class,
};

#[cfg(feature = "timeline")]
pub use timeline::{
  Timeline, TimelineContent, TimelineItem, TimelineMarker, TimelineOrientation, TimelineTime,
  timeline_class, timeline_content_class, timeline_item_class, timeline_marker_class,
  timeline_time_class,
};

#[cfg(feature = "steps")]
pub use steps::{
  Step, StepStatus, Steps, StepsOrientation, step_class, step_indicator_class, step_label_class,
  step_track_class, steps_class,
};

#[cfg(feature = "indicator")]
pub use indicator::{
  Indicator, IndicatorItem, IndicatorPlacement, indicator_class, indicator_item_class,
};

#[cfg(feature = "status")]
pub use status::{Status, StatusSize, StatusVariant, status_class};

#[cfg(feature = "radial-progress")]
pub use radial_progress::{RadialProgress, RadialProgressSize, radial_progress_class};

#[cfg(feature = "countdown")]
pub use countdown::{Countdown, CountdownParts, countdown_class, countdown_parts};

#[cfg(feature = "diff")]
pub use diff::{Diff, DiffAfter, DiffBefore, diff_class, diff_layer_class};

#[cfg(feature = "rating")]
pub use rating::{Rating, rating_class};

#[cfg(feature = "number-input")]
pub use number_input::{NumberInput, number_input_class};

#[cfg(feature = "tags-input")]
pub use tags_input::{TagsInput, tags_input_class};

#[cfg(feature = "file-input")]
pub use file_input::{FileInput, file_input_class};

#[cfg(feature = "swap")]
pub use swap::{Swap, SwapEffect, swap_class, swap_layer_class};

#[cfg(feature = "dock")]
pub use dock::{Dock, DockItem, DockLabel, dock_class, dock_item_class, dock_label_class};

#[cfg(feature = "fab")]
pub use fab::{Fab, FabAction, fab_action_class, fab_class};
#[cfg(feature = "menu")]
pub use menu::{Menu, MenuGroup, MenuItem, MenuTitle, menu_class, menu_item_class};
#[cfg(feature = "mockup")]
pub use mockup::{
  MockupBrowser, MockupCode, MockupCodeLine, MockupPhone, MockupWindow, mockup_code_line_class,
  mockup_frame_class,
};
#[cfg(feature = "theme-controller")]
pub use theme_controller::{THEME_STORAGE_KEY, Theme, ThemeController, theme_init_script};
