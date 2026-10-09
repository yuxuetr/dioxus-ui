use dioxus::prelude::*;

mod self_test;
use dioxus_shadcn::FileInput;
use dioxus_shadcn::NavigationMenuOrientation;
use dioxus_shadcn::NumberInput;
use dioxus_shadcn::Progress;
use dioxus_shadcn::RangeSlider;
use dioxus_shadcn::Rating;
use dioxus_shadcn::SliderOrientation;
use dioxus_shadcn::TagsInput;
use dioxus_shadcn::{
  Accordion, AccordionContent, AccordionItem, AccordionTrigger, Calendar, CalendarBody,
  CalendarCaption, CalendarDate, CalendarDay, CalendarGrid, CalendarHeader, CalendarMonth,
  CalendarNav, CalendarNavButton, CalendarNavDirection, CalendarRow, CalendarWeekday,
  ChartAreaSeries, ChartBarSeries, ChartDescription, ChartFallbackTable, ChartLegend,
  ChartLineSeries, ChartRoot, ChartSvg, ChartTitle, Checkbox, Combobox, ComboboxContent,
  ComboboxInput, ComboboxItem, ComboboxList, ComboboxStatus, Command, CommandEmpty, CommandGroup,
  CommandInput, CommandItem, CommandLabel, CommandList, CommandStatus, ContextMenu,
  ContextMenuCheckboxItem, ContextMenuContent, ContextMenuItem, ContextMenuSub,
  ContextMenuSubContent, ContextMenuSubTrigger, ContextMenuTrigger, DatePicker, DatePickerContent,
  DatePickerTrigger, DatePickerValue, Dropdown, DropdownContent, DropdownItem, DropdownSeparator,
  HoverCard, HoverCardContent, HoverCardDescription, HoverCardHeader, HoverCardTitle,
  HoverCardTrigger, Label, Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarSub,
  MenubarSubContent, MenubarSubTrigger, MenubarTrigger, NavigationMenu, NavigationMenuContent,
  NavigationMenuItem, NavigationMenuLink, NavigationMenuList, NavigationMenuTrigger,
  NavigationOrientation, RadioGroup, RadioGroupItem, Select, SelectContent, SelectItem,
  SelectTrigger, SonnerClose, SonnerContent, SonnerTitle, SonnerToast, SonnerVariant,
  SonnerViewport, Switch, Tabs, TabsActivation, TabsContent, TabsList, TabsOrientation,
  TabsTrigger, ToastAction, ToastClose, ToastRoot, ToastTitle, ToastViewport, ToggleGroup,
  ToggleGroupItem, calendar_month_grid, calendar_move_date, command_matches,
  sonner_dismiss_reason_attribute, toast_dismiss_reason_attribute,
};
use dioxus_shadcn::{
  AlertDialog, AlertDialogAction, AlertDialogActionVariant, AlertDialogCancel, AlertDialogContent,
  AlertDialogDescription, AlertDialogOverlay, AlertDialogTitle, AlertDialogTrigger,
  AttachmentOrientation, AttachmentSize, AttachmentState, BubbleAlign, ButtonGroupOrientation,
  ButtonSize, ButtonVariant, ChartColorToken, ChartDomain, ChartPoint, ChartScale, ChartSeries,
  Dialog, DialogClose, DialogContent, DialogDescription, DialogOverlay, DialogTitle, DialogTrigger,
  DismissBehavior, MarkerVariant, MessageAlign, MessageScrollerIntent, MessageScrollerMetrics,
  Popover, PopoverContent, PopoverDescription, PopoverTitle, PopoverTrigger, Table, TableBody,
  TableCell, TableFooter, TableHead, TableHeader, TableRow, TextDirection, Tooltip, TooltipContent,
  TooltipTrigger, UiDensity, attachment_class, bubble_class, button_class, button_group_class,
  chart_fallback_rows, chart_view_box, collapsible_class, density_control_class, direction_class,
  input_group_class, input_otp_class, marker_class, message_avatar_class, message_class,
  message_content_class, message_footer_class, message_group_class, message_header_class,
  message_scroller_class, message_scroller_is_at_bottom, message_scroller_jump_button_class,
  message_scroller_show_unread_marker, otp_apply_paste_filtered, otp_slots, use_density,
};
use dioxus_shadcn::{
  AttachmentAction, AttachmentTrigger, ButtonGroup, ButtonGroupItem, ComboboxTrigger, Field,
  FieldLabel, InputGroup, InputGroupAction, InputGroupControl, MessageScrollerJumpButton,
};
use dioxus_shadcn::{
  Button, Collapsible, CollapsibleContent, CollapsibleTrigger, Input, NativeSelect,
  NativeSelectOption, Slider, Textarea, Toggle,
};
use dioxus_shadcn::{
  Carousel, CarouselContent, CarouselIndicator, CarouselItem, CarouselNext, CarouselPrevious,
  CarouselViewport,
};
use dioxus_shadcn::{DateOrder, DatePickerInput};
use dioxus_shadcn::{Diff, DiffAfter, DiffBefore};
use dioxus_shadcn::{
  DropdownCheckboxItem, DropdownRadioGroup, DropdownRadioItem, DropdownShortcut, DropdownSub,
  DropdownSubContent, DropdownSubTrigger, DropdownTrigger,
};
use dioxus_shadcn::{Fab, FabAction};
use dioxus_shadcn::{
  InputOtp, InputOtpGroup, InputOtpHiddenInput, InputOtpSlot, Pagination, PaginationContent,
  PaginationItem, PaginationLink, PaginationNext, PaginationPrevious,
};
use dioxus_shadcn::{
  LayoutOrientation, ResizableHandle, ResizablePanel, ResizablePanelGroup, ResizablePanelState,
  resizable_resize_pair,
};
use dioxus_shadcn::{Menu, MenuGroup, MenuItem, MenuTitle};
use dioxus_shadcn::{
  Sidebar, SidebarContent, SidebarGroup, SidebarGroupLabel, SidebarItem, SidebarProvider,
  SidebarTrigger,
};
use dioxus_shadcn::{Swap, SwapEffect};
use dioxus_shadcn::{Theme, ThemeController};
pub use self_test::{INTERACTION_SELF_TEST_SCRIPT, InteractionSelfTest};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewTarget {
  Web,
  Desktop,
  Mobile,
}

impl PreviewTarget {
  fn label(self) -> &'static str {
    match self {
      Self::Web => "web",
      Self::Desktop => "desktop",
      Self::Mobile => "mobile",
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
    format!("dioxus-shadcn {} demo {}: {}", target.label(), self.label, self.value)
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
    notes: "Controlled state target; keyboard and click requests are browser-verified in the interaction panel.",
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
    notes: "Controlled state target; keyboard highlight, filtering resets, and choices are browser-verified in the interaction panel.",
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
    component: "countdown",
    label: "Countdown",
    panel: "data-display",
    test_id: "component-preview-countdown",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
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
    component: "diff",
    label: "Diff",
    panel: "data-display",
    test_id: "component-preview-diff",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
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
    component: "dock",
    label: "Dock",
    panel: "navigation",
    test_id: "component-preview-dock",
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
    component: "fab",
    label: "Fab",
    panel: "actions",
    test_id: "component-preview-fab",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
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
    component: "file-input",
    label: "File Input",
    panel: "forms",
    test_id: "component-preview-file-input",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
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
    component: "indicator",
    label: "Indicator",
    panel: "layout",
    test_id: "component-preview-indicator",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
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
    label: "Input OTP",
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
    component: "menu",
    label: "Menu",
    panel: "navigation",
    test_id: "component-preview-menu",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
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
    component: "mockup",
    label: "Mockup",
    panel: "layout",
    test_id: "component-preview-mockup",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
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
    component: "number-input",
    label: "Number Input",
    panel: "forms",
    test_id: "component-preview-number-input",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
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
    component: "radial-progress",
    label: "Radial Progress",
    panel: "data-display",
    test_id: "component-preview-radial-progress",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "radio-group",
    label: "Radio Group",
    panel: "forms",
    test_id: "component-preview-radio-group",
    coverage_level: "controlled",
    notes: "Controlled state target; keyboard and click requests are browser-verified in the interaction panel.",
  },
  ComponentPreviewTarget {
    component: "rating",
    label: "Rating",
    panel: "forms",
    test_id: "component-preview-rating",
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
    component: "stat",
    label: "Stat",
    panel: "data-display",
    test_id: "component-preview-stat",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "status",
    label: "Status",
    panel: "feedback",
    test_id: "component-preview-status",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "steps",
    label: "Steps",
    panel: "navigation",
    test_id: "component-preview-steps",
    coverage_level: "static",
    notes: "Rendered markup and class-state coverage target.",
  },
  ComponentPreviewTarget {
    component: "swap",
    label: "Swap",
    panel: "actions",
    test_id: "component-preview-swap",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
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
    notes: "Controlled state target; keyboard and click requests are browser-verified in the interaction panel.",
  },
  ComponentPreviewTarget {
    component: "tags-input",
    label: "Tags Input",
    panel: "forms",
    test_id: "component-preview-tags-input",
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
    component: "timeline",
    label: "Timeline",
    panel: "data-display",
    test_id: "component-preview-timeline",
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
    component: "theme-controller",
    label: "Theme Controller",
    panel: "actions",
    test_id: "component-preview-theme-controller",
    coverage_level: "controlled",
    notes: "Controlled rendered state target; mutations remain app-owned.",
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
    notes: "Controlled state target; keyboard and click requests are browser-verified in the interaction panel.",
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
        "{}/{}",
        message_scroller_is_at_bottom(metrics, config.bottom_threshold),
        message_scroller_jump_button_class(unread_visible, config.jump_button_class)
      ),
    },
    PreviewLine {
      label: "marker class",
      value: marker_class(config.marker_variant, config.marker_class),
    },
    PreviewLine {
      label: "chart helper",
      value: format!(
        "{}/{}",
        chart_view_box(config.chart_width, config.chart_height),
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

// Compiled Tailwind for every preview target; regenerate with
// `npm run css:preview` after class changes (RFC 0049).
const PREVIEW_CSS: Asset = asset!("/assets/preview.generated.css");

#[component]
pub fn PreviewSurface(target: PreviewTarget, title: String) -> Element {
  let states = preview_lines(target);
  // Assistive technology needs `lang` to pick a voice; the preview text is
  // English. The web demo's `index.html` sets it, since the page's CSP
  // refuses eval (RFC 0080); the Desktop and Mobile pages have no template.
  #[cfg(not(target_arch = "wasm32"))]
  use_effect(|| {
    document::eval("document.documentElement.lang = 'en';");
  });
  let mut disclosure_open = use_signal(|| false);
  let mut overlay_open = use_signal(|| false);
  let mut selected_option = use_signal(|| "alpha");
  let mut command_active = use_signal(|| "open-file");
  let mut scroll_status = use_signal(|| "held");
  let mut dialog_open = use_signal(|| false);
  let mut alert_dialog_open = use_signal(|| false);
  let mut popover_open = use_signal(|| false);
  let mut select_value = use_signal(|| "banana".to_string());
  let mut combobox_open = use_signal(|| false);
  let mut combobox_query = use_signal(String::new);
  let mut combobox_value = use_signal(|| "none".to_string());
  let mut command_query = use_signal(String::new);
  let mut command_result = use_signal(|| "none".to_string());
  let command_status = if command_query().trim().is_empty() {
    String::new()
  } else {
    result_count_text(
      INTERACTION_COMMANDS
        .iter()
        .filter(|(_, _, label, _)| command_matches(label, &command_query()))
        .count(),
    )
  };
  let combobox_status = if combobox_open() {
    result_count_text(
      INTERACTION_FRUITS
        .iter()
        .filter(|(_, label, _)| label.to_lowercase().contains(&combobox_query().to_lowercase()))
        .count(),
    )
  } else {
    String::new()
  };
  let mut dropdown_action = use_signal(|| "none");
  let mut options_open = use_signal(|| false);
  let mut file_action = use_signal(|| "none");
  let mut status_bar = use_signal(|| true);
  let mut minimap = use_signal(|| false);
  let mut panel = use_signal(|| "bottom".to_string());
  let mut menubar_value = use_signal(String::new);
  let mut menubar_action = use_signal(|| "none");
  let mut navigation_active = use_signal(String::new);
  let mut mega_active = use_signal(String::new);
  let mut typed_date = use_signal(|| None::<CalendarDate>);
  let mut mega_sub = use_signal(|| "web".to_string());
  let mut tabs_value = use_signal(|| "account".to_string());
  let mut settings_tab = use_signal(|| "general".to_string());
  let mut radio_value = use_signal(|| None::<String>);
  let mut toggle_value = use_signal(String::new);
  let mut wifi_enabled = use_signal(|| false);
  let mut terms_accepted = use_signal(|| false);
  let mut disabled_changes = use_signal(|| 0_u32);
  let mut mixed_items = use_signal(|| [true, false]);
  let mut stays_mixed_changes = use_signal(|| 0_u32);
  let mut stays_mixed_request = use_signal(|| None::<bool>);
  let mut button_clicks = use_signal(|| 0_u32);
  let mut bold_pressed = use_signal(|| false);
  // The previews start light; the header toggle shows the opt-in `.dark`
  // theme (RFC 0050).
  let mut dark_theme = use_signal(|| false);
  let theme_class = if dark_theme() { "dark " } else { "" };
  let mut email_value = use_signal(String::new);
  let mut notes_value = use_signal(String::new);
  let mut volume = use_signal(|| 40.0);
  let mut balance = use_signal(|| 50.0);
  let mut diff_position = use_signal(|| 50.0);
  let mut rating = use_signal(|| 3_u8);
  let mut quantity = use_signal(|| 5.0);
  let mut topics = use_signal(Vec::<String>::new);
  let mut chosen_files = use_signal(String::new);
  let mut menu_swapped = use_signal(|| false);
  let mut fab_open = use_signal(|| false);
  let mut fruits = use_signal(Vec::<String>::new);
  let mut tag_query = use_signal(String::new);
  let mut tag_values = use_signal(Vec::<String>::new);
  let mut fab_action = use_signal(String::new);
  let mut locked_slider_changes = use_signal(|| 0_u32);
  let mut details_open = use_signal(|| false);
  let mut size_value = use_signal(|| "md".to_string());
  let mut otp_code = use_signal(String::new);
  let mut results_page = use_signal(|| 1);
  let mut disabled_link_clicks = use_signal(|| 0);
  let mut last_part_action = use_signal(String::new);
  let mut part_form_submits = use_signal(|| 0);
  let mut carousel_index = use_signal(|| 0_usize);
  let mut shell_collapsed = use_signal(|| false);
  let mut price = use_signal(|| (20.0, 80.0));
  let mut chosen_theme = use_signal(|| Theme::System);
  let mut menu_page = use_signal(|| "overview");
  let mut menu_reports_open = use_signal(|| false);
  let mut controller_mounted = use_signal(|| true);
  let mut shell_mobile_open = use_signal(|| false);
  let mut sidebar_collapsed = use_signal(|| false);
  let mut sidebar_section = use_signal(|| "inbox");
  let mut sidebar_disabled_clicks = use_signal(|| 0);
  let mut resizable_panels = use_signal(|| {
    (ResizablePanelState::new(30.0, 20.0, 80.0), ResizablePanelState::new(70.0, 20.0, 80.0))
  });
  let mut accordion_value = use_signal(String::new);
  let mut context_bookmarked = use_signal(|| false);
  let mut context_action = use_signal(|| "none");
  let mut date_open = use_signal(|| false);
  let mut date_month = use_signal(|| CalendarMonth::unchecked(2026, 10));
  let mut date_focused = use_signal(|| CalendarDate::unchecked(2026, 10, 15));
  let mut date_selected = use_signal(|| CalendarDate::unchecked(2026, 10, 15));
  let mut move_date_focus = move |date: CalendarDate| {
    date_focused.set(date);
    date_month.set(CalendarMonth::unchecked(date.year, date.month));
  };
  let mut tooltip_open = use_signal(|| false);
  let mut hover_card_open = use_signal(|| false);
  let mut toast_open = use_signal(|| false);
  let mut toast_reason = use_signal(|| "none");
  let mut sonner_open = use_signal(|| false);
  let mut sonner_reason = use_signal(|| "none");
  let mut alert_dialog_result = use_signal(|| "pending");
  let root = match target {
    PreviewTarget::Web => "web",
    PreviewTarget::Desktop => "desktop",
    PreviewTarget::Mobile => "mobile",
  };
  let disclosure_state = if disclosure_open() { "open" } else { "closed" };
  let disclosure_expanded = if disclosure_open() { "true" } else { "false" };
  let overlay_state = if overlay_open() { "open" } else { "closed" };
  let overlay_expanded = if overlay_open() { "true" } else { "false" };
  let selected_option_value = selected_option();
  let command_active_value = command_active();
  let scroll_status_value = scroll_status();
  let primary_button_class =
    button_class(ButtonVariant::Primary, ButtonSize::Md, use_density(), "");
  let secondary_button_class =
    button_class(ButtonVariant::Secondary, ButtonSize::Sm, use_density(), "");
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
  let bubble = bubble_class(BubbleAlign::Start, "rounded-xl bg-secondary p-3");
  let marker = marker_class(MarkerVariant::Border, "text-primary");
  let scroller = message_scroller_class("h-64 overflow-auto");
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
  let fallback_rows = chart_fallback_rows(std::slice::from_ref(&chart_series));

  rsx! {
    document::Stylesheet { href: PREVIEW_CSS }
    main {
      class: "{theme_class}min-h-screen bg-background text-foreground",
      "data-preview-root": "{root}",
      section {
        class: "mx-auto flex w-full max-w-6xl flex-col gap-6 px-6 py-8",
        "data-preview-panel": "overview",
        header {
          class: "flex flex-col gap-2 border-b border-border pb-4",
          div { class: "flex flex-wrap items-center justify-between gap-3",
            h1 { class: "text-2xl font-semibold", "{title}" }
            Toggle {
              id: "preview-theme-toggle",
              pressed: dark_theme(),
              on_pressed_change: move |pressed| dark_theme.set(pressed),
              "Dark theme"
            }
          }
          p {
            class: "max-w-3xl text-sm text-muted-foreground",
            "Rendered preview shell for representative component states. This is a component preview surface, not a landing page."
          }
        }
        section {
          class: "grid grid-cols-1 gap-3 md:grid-cols-2",
          "data-preview-panel": "actions",
          button { class: "{primary_button_class}", "Primary action" }
          button { class: "{secondary_button_class}", "Secondary action" }
        }
        section {
          class: "grid grid-cols-1 gap-3 rounded-md border border-border p-4 sm:grid-cols-2 lg:grid-cols-3",
          "data-preview-panel": "mobile-profile",
          h2 { class: "text-sm font-medium sm:col-span-2 lg:col-span-3", "Mobile Web profile" }
          p {
            class: "text-sm text-muted-foreground sm:col-span-2 lg:col-span-3",
            "Source-level markers for mobile-width Web verification. Native device behavior remains app-owned."
          }
          div {
            class: "rounded-md bg-muted p-3 text-sm",
            "data-mobile-profile": "touch-targets",
            button {
              class: button_class(
                ButtonVariant::Primary,
                ButtonSize::Md,
                UiDensity::Comfortable,
                "min-h-11 w-full",
              ),
              "Touch target"
            }
          }
          div {
            class: "rounded-md bg-muted p-3 text-sm",
            "data-mobile-profile": "hover-alternative",
            button {
              class: button_class(
                ButtonVariant::Secondary,
                ButtonSize::Sm,
                UiDensity::Compact,
                "min-h-11 w-full",
              ),
              "Tap or focus"
            }
          }
          div {
            class: "rounded-md bg-muted p-3 text-sm",
            "data-mobile-profile": "safe-area-owned",
            "Safe-area padding is owned by the app shell."
          }
          div {
            class: "rounded-md bg-muted p-3 text-sm",
            "data-mobile-profile": "reduced-motion",
            "Animation and timer policy remains app-owned."
          }
          div {
            class: "rounded-md bg-muted p-3 text-sm",
            "data-mobile-profile": "visible-status",
            "Visible status text mirrors runtime-sensitive behavior."
          }
        }
        section {
          class: "grid grid-cols-1 gap-4 lg:grid-cols-2",
          "data-preview-panel": "form",
          article {
            class: "rounded-md border border-border p-4",
            h2 { class: "text-sm font-medium", "Form states" }
            div {
              class: "{form_group_class} mt-3",
              span { class: "text-sm text-muted-foreground", "https://" }
              input {
                class: "min-w-0 flex-1 bg-transparent px-3 py-2 text-sm outline-none",
                value: "dioxus-shadcn.dev",
                "aria-label": "domain",
              }
            }
            div {
              class: "{otp_class} mt-4 flex gap-2",
              "aria-label": "one-time code",
              for digit in ["1", "2", "3", "", "", ""] {
                span {
                  class: "flex h-10 w-10 items-center justify-center rounded-md border border-border text-sm",
                  "{digit}"
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-preview-panel": "overlay-open",
            h2 { class: "text-sm font-medium", "Open overlay state" }
            div {
              class: "relative mt-3 min-h-36 rounded-md bg-muted p-4",
              button { class: "{secondary_button_class}", "Open popover" }
              div {
                class: "absolute left-4 top-16 z-10 w-64 rounded-md border border-border bg-background p-3 text-sm shadow-md",
                role: "dialog",
                "aria-label": "Preview popover",
                p { class: "font-medium", "Preview popover" }
                p { class: "mt-1 text-muted-foreground", "Open-state target for screenshot verification." }
              }
            }
          }
        }
        section {
          class: "rounded-md border border-border p-4",
          "data-preview-panel": "message",
          h2 { class: "text-sm font-medium", "Message states" }
          div {
            class: "{scroller} mt-3 rounded-md bg-muted p-3",
            // It scrolls, so keyboard users need a Tab stop to reach it.
            tabindex: "0",
            role: "region",
            "aria-label": "Conversation",
            div {
              class: "{message_group}",
              div {
                class: "{user_message}",
                div { class: "{message_avatar_class(\"bg-info/20!\")}", "U" }
                div {
                  class: "{user_content}",
                  div { class: "{message_header_class(\"\")}", "User - just now" }
                  div { class: "rounded-xl bg-primary px-3 py-2 text-sm text-primary-foreground", "Can we preview the expanded parity states?" }
                  div { class: "{message_footer_class(\"\")}", "sent" }
                }
              }
              div {
                class: "{assistant_message}",
                div { class: "{message_avatar_class(\"bg-success/20!\")}", "A" }
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
          class: "rounded-md border border-border p-4",
          "data-preview-panel": "chart",
          h2 { class: "text-sm font-medium", "Chart states" }
          p { class: "mt-1 text-sm text-muted-foreground", "Line, area, bar, legend, and fallback table targets." }
          ChartRoot { class: "mt-3",
            ChartLegend {
              span { "Revenue" }
              span { "Area" }
              span { "Bars" }
            }
            ChartSvg { view_box: view_box, title_id: "preview-chart-title", description_id: "preview-chart-description", class: "mt-4",
              ChartTitle { id: "preview-chart-title", "Revenue preview chart" }
              ChartDescription { id: "preview-chart-description", "Revenue over four periods, one missing." }
              ChartAreaSeries { series: chart_series.clone(), x_scale: chart_x, y_scale: chart_y }
              ChartLineSeries { series: chart_series.clone(), x_scale: chart_x, y_scale: chart_y }
              ChartBarSeries {
                series: chart_series,
                x_scale: chart_x,
                y_scale: chart_y,
                color: ChartColorToken::Secondary,
                bar_width: 24.0,
                class: "opacity-60",
              }
            }
            ChartFallbackTable { rows: fallback_rows, caption: "Chart fallback data", class: "mt-4" }
          }
        }
        section {
          class: "grid grid-cols-1 gap-4 lg:grid-cols-2",
          "data-preview-panel": "interactions",
          "data-interaction-root": "runtime",
          article {
            class: "rounded-md border border-border p-4",
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
              class: "mt-3 rounded-md bg-muted p-3 text-sm text-muted-foreground",
              "data-interaction-state": "disclosure-content",
              hidden: !disclosure_open(),
              "Disclosure content is visible when open."
            }
          }
          article {
            class: "rounded-md border border-border p-4",
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
              class: "mt-3 rounded-md border border-border bg-background p-3 text-sm shadow-sm",
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
            class: "rounded-md border border-border p-4",
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
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "keyboard",
            h2 { class: "text-sm font-medium", "Keyboard-visible state" }
            div {
              class: "mt-3 rounded-md border border-border p-2",
              role: "listbox",
              tabindex: "0",
              "aria-label": "Commands",
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
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "scroll-status",
            "data-state": "{scroll_status_value}",
            h2 { class: "text-sm font-medium", "Scroll status interaction" }
            p {
              class: "mt-3 text-sm text-muted-foreground",
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
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "popover",
            "data-state": if popover_open() { "open" } else { "closed" },
            h2 { class: "text-sm font-medium", "Popover interaction" }
            // Controlled: the trigger asks the app, which sets `open`.
            Popover { open: popover_open(), on_open_change: move |open| popover_open.set(open),
              PopoverTrigger {
                class: "{secondary_button_class} mt-3",
                "data-interaction-control": "popover-trigger",
                "Toggle popover"
              }
              PopoverContent {
                PopoverTitle { "Dimensions" }
                PopoverDescription { "Placed below the trigger, or above it near the viewport bottom." }
              }
            }
            Popover {
              PopoverContent { "data-interaction-control": "plain-popover", "Plain popover" }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "select",
            "data-value": "{select_value}",
            h2 { class: "text-sm font-medium", "Select interaction" }
            // Controlled: the app holds the value (RFC 0077).
            Select {
              id: "interaction-select-trigger",
              value: select_value(),
              on_value_change: move |value| select_value.set(value),
              SelectTrigger {
                class: "mt-3",
                "aria-label": "Fruit",
                "aria-describedby": "interaction-select-hint",
                span { class: "truncate", "{fruit_label(&select_value())}" }
              }
              p { id: "interaction-select-hint", class: "text-xs", "Pick one fruit." }
              SelectContent {
                for (value, label, disabled) in INTERACTION_FRUITS {
                  SelectItem { key: "{value}", value: *value, disabled: *disabled, "{label}" }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "multi-select",
            "data-values": "{fruits().join(\"|\")}",
            h2 { class: "text-sm font-medium", "Multi-select interaction" }
            // Uncontrolled: the root holds the values and reports them.
            Select {
              id: "interaction-multi-select-trigger",
              multiple: true,
              on_values_change: move |values| fruits.set(values),
              SelectTrigger {
                class: "mt-3",
                "aria-label": "Fruits",
                span {
                  class: "truncate",
                  if fruits().is_empty() {
                    "Pick fruits"
                  } else {
                    "{fruits().len()} selected"
                  }
                }
              }
              SelectContent {
                for (value, label, disabled) in INTERACTION_FRUITS {
                  SelectItem { key: "{value}", value: *value, disabled: *disabled, "{label}" }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "command",
            "data-result": "{command_result}",
            h2 { class: "text-sm font-medium", "Command interaction" }
            Command {
              class: "mt-3 border border-border",
              on_select: move |value: String| command_result.set(value),
              CommandInput {
                value: command_query(),
                placeholder: "Type a command...",
                oninput: move |event: FormEvent| command_query.set(event.value()),
              }
              CommandStatus { "{command_status}" }
              CommandList {
                if !INTERACTION_COMMANDS.iter().any(|(_, _, label, _)| command_matches(label, &command_query())) {
                  CommandEmpty { "No results found." }
                }
                for group in ["Suggestions", "Settings"] {
                  if INTERACTION_COMMANDS
                    .iter()
                    .any(|(item_group, _, label, _)| *item_group == group && command_matches(label, &command_query()))
                  {
                    CommandGroup { key: "{group}",
                      CommandLabel { "{group}" }
                      for (_, value, label, disabled) in INTERACTION_COMMANDS
                        .iter()
                        .filter(|(item_group, _, label, _)| *item_group == group && command_matches(label, &command_query()))
                      {
                        CommandItem {
                          key: "{value}",
                          id: "interaction-command-item-{value}",
                          value: *value,
                          disabled: *disabled,
                          "{label}"
                        }
                      }
                    }
                  }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "combobox",
            "data-value": "{combobox_value}",
            h2 { class: "text-sm font-medium", "Combobox interaction" }
            // Controlled open, for the status; the root holds the value.
            Combobox {
              id: "interaction-combobox-input",
              open: combobox_open(),
              on_open_change: move |open| combobox_open.set(open),
              on_value_change: move |value: String| {
                combobox_query.set(fruit_label(&value).to_string());
                combobox_value.set(value);
              },
              ComboboxInput {
                class: "mt-3 border border-border",
                "aria-label": "Fruit",
                "aria-describedby": "interaction-select-hint",
                value: combobox_query(),
                placeholder: "Search fruit",
                oninput: move |event: FormEvent| combobox_query.set(event.value()),
              }
              ComboboxStatus { "{combobox_status}" }
              ComboboxContent {
                ComboboxList {
                  for (value, label, disabled) in INTERACTION_FRUITS
                    .iter()
                    .filter(|(_, label, _)| label.to_lowercase().contains(&combobox_query().to_lowercase()))
                  {
                    ComboboxItem { key: "{value}", value: *value, disabled: *disabled, "{label}" }
                  }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "multi-combobox",
            "data-values": "{tag_values().join(\"|\")}",
            h2 { class: "text-sm font-medium", "Multi-combobox interaction" }
            // Uncontrolled: the root holds the values and the open state.
            Combobox {
              id: "interaction-multi-combobox-input",
              multiple: true,
              on_values_change: move |values| tag_values.set(values),
              ComboboxInput {
                class: "mt-3 border border-border",
                "aria-label": "Fruit tags",
                value: tag_query(),
                oninput: move |event: FormEvent| tag_query.set(event.value()),
              }
              ComboboxContent {
                ComboboxList {
                  for (value, label, disabled) in INTERACTION_FRUITS {
                    ComboboxItem { key: "{value}", value: *value, disabled: *disabled, "{label}" }
                  }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "date-input",
            "data-value": "{typed_date().map(iso_date).unwrap_or_default()}",
            h2 { class: "text-sm font-medium", "Date input interaction" }
            DatePickerInput {
              class: "mt-3",
              "aria-label": "Birthday",
              order: DateOrder::DayMonthYear,
              value: typed_date(),
              on_value_change: move |date| typed_date.set(date),
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "date-picker",
            "data-value": "{iso_date(date_selected())}",
            h2 { class: "text-sm font-medium", "Date picker interaction" }
            DatePicker {
              id: "interaction-date-trigger",
              open: date_open(),
              on_open_change: move |open| {
                if open {
                  move_date_focus(date_selected());
                }
                date_open.set(open);
              },
            DatePickerTrigger { class: "mt-3",
              DatePickerValue { "{iso_date(date_selected())}" }
            }
            DatePickerContent {
              Calendar {
                CalendarHeader {
                  CalendarCaption { id: "interaction-date-caption",
                    "{date_month().year}-{date_month().month:02}"
                  }
                  CalendarNav {
                    CalendarNavButton {
                      direction: CalendarNavDirection::Previous,
                      onclick: move |_| date_month.set(date_month().add_months(-1)),
                      "<"
                    }
                    CalendarNavButton {
                      direction: CalendarNavDirection::Next,
                      onclick: move |_| date_month.set(date_month().add_months(1)),
                      ">"
                    }
                  }
                }
                CalendarGrid { "aria-labelledby": "interaction-date-caption",
                  CalendarBody {
                    for week in calendar_month_grid(
                        date_month(),
                        CalendarWeekday::Sunday,
                        None,
                        Some(date_selected()),
                        None,
                        None,
                        &[],
                      )
                      .weeks
                    {
                      CalendarRow {
                        for day in week {
                          CalendarDay {
                            key: "{iso_date(day.date)}",
                            date: day.date,
                            selected: day.selected,
                            outside_month: day.outside_month,
                            focused: day.date == date_focused(),
                            on_key_move: move |key_move| {
                              move_date_focus(
                                calendar_move_date(date_focused(), key_move, CalendarWeekday::Sunday),
                              );
                            },
                            on_select: move |date| {
                              date_selected.set(date);
                              date_open.set(false);
                            },
                            "{day.date.day}"
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "dropdown",
            "data-action": "{dropdown_action}",
            h2 { class: "text-sm font-medium", "Dropdown interaction" }
            Dropdown {
              DropdownTrigger {
                class: "{secondary_button_class} mt-3",
                "data-interaction-control": "dropdown-trigger",
                "Actions"
              }
              DropdownContent {
                DropdownItem { onclick: move |_| dropdown_action.set("edit"), "Edit" }
                DropdownItem { onclick: move |_| dropdown_action.set("duplicate"), "Duplicate" }
                DropdownItem {
                  disabled: true,
                  onclick: move |_| dropdown_action.set("archive"),
                  "Archive"
                }
                DropdownSeparator {}
                DropdownItem {
                  destructive: true,
                  onclick: move |_| dropdown_action.set("delete"),
                  "Delete"
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "dropdown-options",
            "data-status-bar": "{status_bar}",
            "data-minimap": "{minimap}",
            "data-panel": "{panel}",
            h2 { class: "text-sm font-medium", "Dropdown options interaction" }
            // Controlled: the app holds `open` and the radio group's value.
            Dropdown { open: options_open(), on_open_change: move |open| options_open.set(open),
              DropdownTrigger {
                class: "{secondary_button_class} mt-3",
                "data-interaction-control": "dropdown-options-trigger",
                "View"
              }
              DropdownContent {
                DropdownCheckboxItem {
                  checked: status_bar(),
                  onclick: move |_| status_bar.toggle(),
                  "Status bar"
                  DropdownShortcut { "Ctrl+/" }
                }
                DropdownCheckboxItem {
                  checked: minimap(),
                  onclick: move |_| minimap.toggle(),
                  "Minimap"
                }
                DropdownSeparator {}
                DropdownRadioGroup { value: panel(), on_value_change: move |value| panel.set(value),
                  DropdownRadioItem { value: "bottom", "Panel bottom" }
                  DropdownRadioItem { value: "right", "Panel right" }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "dropdown-submenu",
            "data-action": "{file_action}",
            h2 { class: "text-sm font-medium", "Dropdown submenu interaction" }
            Dropdown {
              DropdownTrigger {
                class: "{secondary_button_class} mt-3",
                "data-interaction-control": "dropdown-submenu-trigger",
                "File"
              }
              DropdownContent { align: dioxus_shadcn::DropdownAlign::Start,
                DropdownItem { onclick: move |_| file_action.set("new"), "New file" }
                DropdownSub {
                  DropdownSubTrigger { "Share" }
                  DropdownSubContent {
                    DropdownItem { onclick: move |_| file_action.set("copy"), "Copy link" }
                    DropdownItem { onclick: move |_| file_action.set("email"), "Email" }
                  }
                }
                DropdownItem { onclick: move |_| file_action.set("print"), "Print" }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "menubar",
            "data-action": "{menubar_action}",
            h2 { class: "text-sm font-medium", "Menubar interaction" }
            Menubar {
              class: "mt-3",
              "aria-label": "Editor",
              // Controlled: the app holds the open menu's value.
              value: menubar_value(),
              on_value_change: move |value| menubar_value.set(value),
              MenubarMenu {
                value: "file",
                MenubarTrigger { "File" }
                MenubarContent {
                  MenubarItem { onclick: move |_| menubar_action.set("new"), "New" }
                  MenubarItem { onclick: move |_| menubar_action.set("open"), "Open" }
                }
              }
              MenubarMenu {
                value: "edit",
                MenubarTrigger { "Edit" }
                MenubarContent {
                  MenubarItem { onclick: move |_| menubar_action.set("undo"), "Undo" }
                  MenubarItem { onclick: move |_| menubar_action.set("redo"), "Redo" }
                }
              }
              MenubarMenu {
                value: "help",
                MenubarTrigger { disabled: true, "Help" }
              }
              MenubarMenu {
                value: "view",
                MenubarTrigger { "View" }
                MenubarContent {
                  MenubarItem { onclick: move |_| menubar_action.set("zoom-in"), "Zoom in" }
                  MenubarItem { onclick: move |_| menubar_action.set("zoom-out"), "Zoom out" }
                  MenubarSub {
                    MenubarSubTrigger { "Appearance" }
                    MenubarSubContent {
                      MenubarItem {
                        onclick: move |_| menubar_action.set("fullscreen"),
                        "Full screen"
                      }
                    }
                  }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "navigation-menu",
            "data-value": "{navigation_active}",
            h2 { class: "text-sm font-medium", "Navigation menu interaction" }
            NavigationMenu {
              class: "mt-3",
              "aria-label": "Product",
              // Uncontrolled: the root holds the open item and reports it.
              on_value_change: move |value: String| navigation_active.set(value),
              NavigationMenuList {
                NavigationMenuItem {
                  value: "docs",
                  NavigationMenuTrigger { "Docs" }
                  NavigationMenuContent {
                    NavigationMenuLink { href: "#navigation-install", "Install" }
                    NavigationMenuLink { href: "#navigation-theming", disabled: true, "Theming" }
                    NavigationMenuLink { href: "#navigation-cli", "CLI" }
                  }
                }
                NavigationMenuItem {
                  NavigationMenuLink { href: "#navigation-blog", "Blog" }
                }
                NavigationMenuItem {
                  value: "archive",
                  NavigationMenuTrigger { disabled: true, "Archive" }
                }
                NavigationMenuItem {
                  value: "examples",
                  NavigationMenuTrigger { "Examples" }
                  NavigationMenuContent {
                    NavigationMenuLink { href: "#navigation-dashboard", "Dashboard" }
                    NavigationMenuLink { href: "#navigation-chat", "Chat" }
                  }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "navigation-submenu",
            "data-value": "{mega_active}",
            "data-sub": "{mega_sub}",
            h2 { class: "text-sm font-medium", "Navigation submenu interaction" }
            NavigationMenu {
              class: "mt-3",
              "aria-label": "Solutions",
              // Controlled, as is the inner menu, which ignores closing.
              value: mega_active(),
              on_value_change: move |value: String| {
                if !value.is_empty() {
                  mega_sub.set("web".to_string());
                }
                mega_active.set(value);
              },
              NavigationMenuList {
                NavigationMenuItem {
                  value: "solutions",
                  NavigationMenuTrigger { "Solutions" }
                  NavigationMenuContent {
                    NavigationMenu {
                      orientation: NavigationMenuOrientation::Vertical,
                      "aria-label": "Solution areas",
                      value: mega_sub(),
                      on_value_change: move |value: String| {
                        if !value.is_empty() {
                          mega_sub.set(value);
                        }
                      },
                      NavigationMenuList {
                        NavigationMenuItem {
                          value: "web",
                          NavigationMenuTrigger { "Web" }
                        }
                        NavigationMenuItem {
                          value: "mobile",
                          NavigationMenuTrigger { "Mobile" }
                        }
                      }
                      NavigationMenuContent { value: "web",
                        NavigationMenuLink { href: "#mega-dashboards", "Dashboards" }
                        NavigationMenuLink { href: "#mega-stores", "Stores" }
                      }
                      NavigationMenuContent { value: "mobile",
                        NavigationMenuLink { href: "#mega-ios", "iOS apps" }
                        NavigationMenuLink { href: "#mega-android", "Android apps" }
                      }
                    }
                  }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "tabs",
            "data-value": "{tabs_value}",
            h2 { class: "text-sm font-medium", "Tabs interaction" }
            // Uncontrolled: the root holds the tab and reports it.
            Tabs {
              class: "mt-3",
              default_value: "account",
              on_value_change: move |value: String| tabs_value.set(value),
              TabsList { "aria-label": "Account settings",
                TabsTrigger { value: "account", "Account" }
                TabsTrigger {
                  value: "password",
                  disabled: true,
                  "Password"
                }
                TabsTrigger { value: "billing", "Billing" }
                TabsTrigger { value: "team", "Team" }
              }
              TabsContent { value: "account", "Account settings" }
              TabsContent { value: "password", "Password settings" }
              TabsContent { value: "billing", "Billing settings" }
              TabsContent { value: "team", "Team settings" }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "tabs-vertical",
            "data-value": "{settings_tab}",
            h2 { class: "text-sm font-medium", "Vertical tabs interaction" }
            Tabs {
              class: "mt-3",
              activation: TabsActivation::Manual,
              orientation: TabsOrientation::Vertical,
              // Controlled: the app holds the tab.
              value: settings_tab(),
              on_value_change: move |value: String| settings_tab.set(value),
              TabsList {
                TabsTrigger { value: "general", "General" }
                TabsTrigger { value: "security", "Security" }
                TabsTrigger { value: "notifications", "Notifications" }
              }
              TabsContent { value: "general", "General settings" }
              TabsContent { value: "security", "Security settings" }
              TabsContent { value: "notifications", "Notification settings" }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "radio-group",
            "data-value": radio_value().unwrap_or_else(|| "none".to_string()),
            h2 { class: "text-sm font-medium", "Radio group interaction" }
            RadioGroup {
              class: "mt-3",
              value: radio_value(),
              "aria-label": "Size",
              on_value_change: move |value: String| radio_value.set(Some(value)),
              RadioGroupItem { id: "interaction-radio-small", value: "small" }
              Label { r#for: "interaction-radio-small", "Small" }
              RadioGroupItem {
                value: "medium",
                "aria-label": "Medium",
                disabled: true,
              }
              RadioGroupItem {
                value: "large",
                "aria-label": "Large",
              }
              RadioGroupItem {
                value: "x-large",
                "aria-label": "Extra large",
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "progress",
            h2 { id: "interaction-progress-label", class: "text-sm font-medium", "Upload" }
            Progress {
              class: "mt-3",
              value: 60.0,
              "aria-labelledby": "interaction-progress-label",
              "aria-valuetext": "3 of 5 files",
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "toggle-group",
            "data-value": if toggle_value().is_empty() { "none".to_string() } else { toggle_value() },
            h2 { class: "text-sm font-medium", "Toggle group interaction" }
            ToggleGroup {
              class: "mt-3",
              "aria-label": "Text style",
              orientation: NavigationOrientation::Horizontal,
              // Uncontrolled: the group owns the pressed item and the fixture records it.
              on_value_change: move |value| toggle_value.set(value),
              ToggleGroupItem { value: "bold", "Bold" }
              ToggleGroupItem { value: "italic", "Italic" }
              ToggleGroupItem { value: "strike", disabled: true, "Strike" }
              ToggleGroupItem { value: "underline", "Underline" }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "switch",
            "data-switch": "{wifi_enabled}",
            "data-checkbox": "{terms_accepted}",
            "data-disabled-changes": "{disabled_changes}",
            "data-mixed-items": "{mixed_items()[0]}-{mixed_items()[1]}",
            "data-stays-mixed-changes": "{stays_mixed_changes}",
            "data-stays-mixed-request": "{stays_mixed_request():?}",
            h2 { class: "text-sm font-medium", "Switch and checkbox interaction" }
            div { class: "mt-3 flex items-center gap-2",
              Switch {
                id: "interaction-switch-wifi",
                checked: wifi_enabled(),
                on_checked_change: move |checked| wifi_enabled.set(checked),
              }
              Label { r#for: "interaction-switch-wifi", "Wi-Fi" }
            }
            div { class: "mt-3 flex items-center gap-2",
              Switch {
                "aria-label": "Airplane mode",
                checked: true,
                disabled: true,
                on_checked_change: move |_| disabled_changes += 1,
              }
            }
            div { class: "mt-3 flex items-center gap-2",
              Checkbox {
                id: "interaction-checkbox-terms",
                name: "terms",
                checked: terms_accepted(),
                on_checked_change: move |checked| terms_accepted.set(checked),
              }
              Label { r#for: "interaction-checkbox-terms", "Accept terms" }
            }
            div { class: "mt-3 flex items-center gap-2",
              Checkbox {
                "aria-label": "Newsletter",
                disabled: true,
                on_checked_change: move |_| disabled_changes += 1,
              }
            }
            div { class: "mt-3 flex items-center gap-2",
              Checkbox {
                "aria-label": "Select all",
                checked: mixed_items().iter().all(|item| *item),
                indeterminate: mixed_items().iter().any(|item| *item)
                  && !mixed_items().iter().all(|item| *item),
                on_checked_change: move |checked| mixed_items.set([checked, checked]),
              }
              Checkbox {
                "aria-label": "Item one",
                checked: mixed_items()[0],
                on_checked_change: move |checked| mixed_items.with_mut(|items| items[0] = checked),
              }
              Checkbox {
                "aria-label": "Item two",
                checked: mixed_items()[1],
                on_checked_change: move |checked| mixed_items.with_mut(|items| items[1] = checked),
              }
              Checkbox {
                "aria-label": "Stays mixed",
                checked: true,
                indeterminate: true,
                on_checked_change: move |checked| {
                  stays_mixed_changes += 1;
                  stays_mixed_request.set(Some(checked));
                },
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "form-controls",
            "data-clicks": "{button_clicks}",
            "data-bold": "{bold_pressed}",
            "data-email": "{email_value}",
            "data-notes": "{notes_value}",
            h2 { class: "text-sm font-medium", "Form control interaction" }
            div { class: "mt-3 flex items-center gap-2",
              Button {
                r#type: "button",
                name: "save",
                onclick: move |_| button_clicks += 1,
                "Save"
              }
              Button {
                r#type: "button",
                disabled: true,
                onclick: move |_| button_clicks += 1,
                "Archive"
              }
              Toggle {
                "aria-label": "Bold",
                pressed: bold_pressed(),
                on_pressed_change: move |pressed| bold_pressed.set(pressed),
                "B"
              }
            }
            div { class: "mt-3 grid gap-2",
              Label { r#for: "interaction-input-email", "Email" }
              Input {
                id: "interaction-input-email",
                r#type: "email",
                name: "email",
                value: email_value(),
                on_value_change: move |value| email_value.set(value),
              }
              Label { r#for: "interaction-textarea-notes", "Notes" }
              Textarea {
                id: "interaction-textarea-notes",
                name: "notes",
                rows: "3",
                value: notes_value(),
                on_value_change: move |value| notes_value.set(value),
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "slider",
            "data-volume": "{volume}",
            "data-balance": "{balance}",
            "data-locked-changes": "{locked_slider_changes}",
            h2 { class: "text-sm font-medium", "Slider interaction" }
            Label { id: "interaction-slider-volume-label", "Volume" }
            Slider {
              class: "mt-3",
              "aria-labelledby": "interaction-slider-volume-label",
              step: 5.0,
              value: volume(),
              on_value_change: move |value| volume.set(value),
            }
            Slider {
              class: "mt-3",
              "aria-label": "Locked",
              value: 30.0,
              disabled: true,
              on_value_change: move |_| locked_slider_changes += 1,
            }
            div { class: "mt-3 h-32",
              Slider {
                "aria-label": "Balance",
                orientation: SliderOrientation::Vertical,
                step: 10.0,
                value: balance(),
                on_value_change: move |value| balance.set(value),
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "diff",
            "data-position": "{diff_position}",
            h2 { class: "text-sm font-medium", "Diff interaction" }
            Diff {
              class: "mt-3 h-24",
              position: diff_position(),
              on_position_change: move |value| diff_position.set(value),
              DiffBefore { class: "grid place-items-center bg-muted text-sm", "Before" }
              DiffAfter { class: "grid place-items-center bg-primary text-sm text-primary-foreground", "After" }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "rating",
            "data-rating": "{rating}",
            h2 { class: "text-sm font-medium", "Rating interaction" }
            Rating {
              class: "mt-3",
              "aria-label": "Product rating",
              value: rating(),
              on_value_change: move |value| rating.set(value),
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "number-input",
            "data-value": "{quantity}",
            h2 { class: "text-sm font-medium", "Number input interaction" }
            NumberInput {
              class: "mt-3 max-w-40",
              "aria-label": "Quantity",
              min: 0.0,
              max: 10.0,
              value: quantity(),
              on_value_change: move |value| quantity.set(value),
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "tags-input",
            "data-tags": "{topics().join(\"|\")}",
            h2 { class: "text-sm font-medium", "Tags input interaction" }
            TagsInput {
              class: "mt-3",
              "aria-label": "Topics",
              tags: topics(),
              on_tags_change: move |tags| topics.set(tags),
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "file-input",
            "data-files": "{chosen_files}",
            h2 { class: "text-sm font-medium", "File input interaction" }
            Label { r#for: "interaction-file-input", "Attachments" }
            FileInput {
              id: "interaction-file-input",
              class: "mt-2",
              multiple: true,
              onchange: move |event: FormEvent| {
                let names = event.files().iter().map(|file| file.name()).collect::<Vec<_>>();
                chosen_files.set(names.join("|"));
              },
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "swap",
            "data-active": "{menu_swapped}",
            h2 { class: "text-sm font-medium", "Swap interaction" }
            Swap {
              class: "mt-3 size-9 border border-border",
              "aria-label": "Menu",
              effect: SwapEffect::Rotate,
              active: menu_swapped(),
              on_active_change: move |active| menu_swapped.set(active),
              on: rsx! { span { "data-swap-layer": "on", "×" } },
              off: rsx! { span { "data-swap-layer": "off", "≡" } },
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "fab",
            "data-open": "{fab_open}",
            "data-action": "{fab_action}",
            h2 { class: "text-sm font-medium", "Fab interaction" }
            Fab {
              class: "mt-3 w-fit",
              fixed: false,
              "aria-label": "Create",
              on_open_change: move |open| fab_open.set(open),
              icon: rsx! { "+" },
              FabAction { label: "Photo", onclick: move |_| fab_action.set("photo".to_string()), "P" }
              FabAction { label: "Note", onclick: move |_| fab_action.set("note".to_string()), "N" }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "collapsible-select",
            "data-open": "{details_open}",
            "data-size": "{size_value}",
            h2 { class: "text-sm font-medium", "Collapsible and native select interaction" }
            // Uncontrolled: the root owns open and the fixture only records it.
            Collapsible {
              class: "mt-3",
              on_open_change: move |open| details_open.set(open),
              title: "Details section",
              CollapsibleTrigger { title: "Show details", "Details" }
              CollapsibleContent {
                role: "region",
                "aria-label": "Details content",
                "More information"
              }
            }
            div { class: "mt-3 grid gap-2",
              Label { r#for: "interaction-native-select-size", "Size" }
              NativeSelect {
                id: "interaction-native-select-size",
                name: "size",
                on_value_change: move |value| size_value.set(value),
                NativeSelectOption { value: "sm", selected: size_value() == "sm", "Small" }
                NativeSelectOption { value: "md", selected: size_value() == "md", "Medium" }
                NativeSelectOption { value: "lg", selected: size_value() == "lg", "Large" }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "input-otp",
            "data-code": "{otp_code}",
            h2 { class: "text-sm font-medium", "Input OTP interaction" }
            Label { id: "interaction-otp-label", "Verification code" }
            // Uncontrolled: the root owns the code and the fixture records it.
            InputOtp {
              class: "mt-3",
              title: "Verification code slots",
              length: 6,
              on_value_change: move |value| otp_code.set(value),
              InputOtpGroup {
                for index in 0..6_usize {
                  InputOtpSlot { key: "{index}", index }
                }
              }
              InputOtpHiddenInput { "aria-labelledby": "interaction-otp-label", name: "code" }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "pagination",
            "data-page": "{results_page}",
            "data-disabled-clicks": "{disabled_link_clicks}",
            h2 { class: "text-sm font-medium", "Pagination interaction" }
            Pagination { class: "mt-3",
              PaginationContent {
                PaginationItem {
                  PaginationPrevious {
                    disabled: results_page() == 1,
                    onclick: move |_| results_page -= 1,
                  }
                }
                for number in 1..=5 {
                  PaginationItem { key: "{number}",
                    PaginationLink {
                      active: results_page() == number,
                      title: "Results page {number}",
                      onclick: move |_| results_page.set(number),
                      "{number}"
                    }
                  }
                }
                PaginationItem {
                  PaginationNext {
                    disabled: results_page() == 5,
                    onclick: move |_| results_page += 1,
                  }
                }
              }
            }
            Pagination { class: "mt-3",
              PaginationContent {
                PaginationItem {
                  PaginationPrevious {
                    href: "#results-0",
                    disabled: true,
                    onclick: move |_| disabled_link_clicks += 1,
                  }
                }
                PaginationItem {
                  PaginationLink { href: "#results-1", active: true, "1" }
                }
                PaginationItem {
                  PaginationNext { href: "#results-2", target: "_self" }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "carousel",
            "data-index": "{carousel_index}",
            h2 { class: "text-sm font-medium", "Carousel interaction" }
            // Controlled: the fixture owns the index and the root steps it.
            Carousel {
              class: "mt-3",
              "aria-label": "Featured products",
              count: 3,
              index: carousel_index(),
              on_index_change: move |index| carousel_index.set(index),
              CarouselViewport { width: "240px",
                CarouselContent {
                  for slide in 0..3_usize {
                    CarouselItem { key: "{slide}", index: slide, "aria-label": "{slide + 1} of 3",
                      "Product {slide + 1}"
                    }
                  }
                }
              }
              CarouselPrevious { title: "Previous product", "‹" }
              CarouselNext { title: "Next product", "›" }
              for slide in 0..3_usize {
                CarouselIndicator {
                  key: "{slide}",
                  index: slide,
                  "aria-label": "Show product {slide + 1}",
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "resizable",
            "data-sizes": "{resizable_panels().0.size}-{resizable_panels().1.size}",
            h2 { class: "text-sm font-medium", "Resizable interaction" }
            ResizablePanelGroup {
              class: "mt-3",
              orientation: LayoutOrientation::Horizontal,
              width: "300px",
              height: "80px",
              ResizablePanel {
                id: "interaction-resizable-files",
                size: resizable_panels().0.size,
                min_size: 20.0,
                max_size: 80.0,
                "Files"
              }
              ResizableHandle {
                orientation: LayoutOrientation::Horizontal,
                "aria-controls": "interaction-resizable-files",
                "aria-label": "Resize files",
                value: resizable_panels().0.size,
                min: 20.0,
                max: 80.0,
                on_resize: move |delta| {
                  let (first, second) = resizable_panels();
                  resizable_panels.set(resizable_resize_pair(first, second, delta));
                },
              }
              ResizablePanel {
                size: resizable_panels().1.size,
                min_size: 20.0,
                max_size: 80.0,
                "Editor"
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "menu",
            "data-page": "{menu_page}",
            h2 { class: "text-sm font-medium", "Menu interaction" }
            nav { class: "mt-3 w-56", "aria-label": "Workspace pages",
              Menu {
                MenuTitle { "Workspace" }
                MenuItem {
                  active: menu_page() == "overview",
                  onclick: move |_| menu_page.set("overview"),
                  "Overview"
                }
                MenuItem { disabled: true, onclick: move |_| menu_page.set("billing"), "Billing" }
                MenuGroup {
                  label: rsx! { "Reports" },
                  open: menu_reports_open(),
                  on_open_change: move |open| menu_reports_open.set(open),
                  MenuItem {
                    active: menu_page() == "sales",
                    onclick: move |_| menu_page.set("sales"),
                    "Sales"
                  }
                  MenuItem {
                    active: menu_page() == "traffic",
                    onclick: move |_| menu_page.set("traffic"),
                    "Traffic"
                  }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "theme-controller",
            "data-theme-choice": "{chosen_theme().as_str()}",
            h2 { class: "text-sm font-medium", "Theme controller interaction" }
            if controller_mounted() {
              ThemeController {
                theme: chosen_theme(),
                storage_key: "dxui-preview-theme",
                on_theme_change: move |stored| chosen_theme.set(stored),
              }
            }
            div { class: "mt-3 flex flex-wrap gap-2",
              for (option, label) in [
                (Theme::System, "System"),
                (Theme::Light, "Light"),
                (Theme::Dark, "Dark"),
                (Theme::Preset("nord".to_string()), "Nord"),
              ]
              {
                button {
                  key: "{label}",
                  class: "{secondary_button_class}",
                  "aria-pressed": "{chosen_theme() == option}",
                  onclick: move |_| chosen_theme.set(option.clone()),
                  "{label}"
                }
              }
              button {
                class: "{secondary_button_class}",
                onclick: move |_| controller_mounted.toggle(),
                if controller_mounted() { "Unmount controller" } else { "Mount controller" }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "range-slider",
            "data-low": "{price().0}",
            "data-high": "{price().1}",
            h2 { class: "text-sm font-medium", "Range slider interaction" }
            div { class: "mt-6 w-72",
              RangeSlider {
                "aria-label": "Price",
                value: price(),
                step: 5.0,
                min_steps_between: 2,
                on_value_change: move |next| price.set(next),
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "sidebar-mobile",
            "data-collapsed": "{shell_collapsed}",
            "data-mobile-open": "{shell_mobile_open}",
            h2 { class: "text-sm font-medium", "Off-canvas sidebar interaction" }
            // Controlled: items close the panel. The sidebar's own id needs a
            // matching `aria-controls` on the trigger.
            SidebarProvider {
              collapsed: shell_collapsed(),
              on_collapsed_change: move |next| shell_collapsed.set(next),
              off_canvas: true,
              mobile_open: shell_mobile_open(),
              on_mobile_open_change: move |next| shell_mobile_open.set(next),
              SidebarTrigger { "aria-controls": "interaction-sidebar-mobile", "Toggle navigation" }
              Sidebar { id: "interaction-sidebar-mobile", "aria-label": "Navigation", shortcut: 'b',
                SidebarContent {
                  SidebarItem { onclick: move |_| shell_mobile_open.set(false), "Projects" }
                  SidebarItem { onclick: move |_| shell_mobile_open.set(false), "Reports" }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "sidebar",
            "data-collapsed": "{sidebar_collapsed}",
            "data-section": "{sidebar_section}",
            "data-disabled-clicks": "{sidebar_disabled_clicks}",
            h2 { class: "text-sm font-medium", "Sidebar interaction" }
            // Uncontrolled: the provider owns collapsed and links the trigger.
            SidebarProvider { on_collapsed_change: move |next| sidebar_collapsed.set(next),
              SidebarTrigger { "Toggle sidebar" }
              Sidebar { "aria-label": "Workspace",
                SidebarContent {
                  SidebarGroup { role: "group", "aria-labelledby": "interaction-sidebar-group",
                    SidebarGroupLabel { id: "interaction-sidebar-group", "Sections" }
                    SidebarItem {
                      active: sidebar_section() == "inbox",
                      onclick: move |_| sidebar_section.set("inbox"),
                      "Inbox"
                    }
                    SidebarItem {
                      active: sidebar_section() == "drafts",
                      onclick: move |_| sidebar_section.set("drafts"),
                      "Drafts"
                    }
                    SidebarItem { href: "#sidebar-settings", title: "Open settings", "Settings" }
                    SidebarItem {
                      href: "#sidebar-archive",
                      disabled: true,
                      onclick: move |_| sidebar_disabled_clicks += 1,
                      "Archive"
                    }
                    SidebarItem {
                      disabled: true,
                      onclick: move |_| sidebar_disabled_clicks += 1,
                      "Trash"
                    }
                    SidebarItem { title: "Help wrapper", "Help" }
                  }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "accordion",
            "data-value": if accordion_value().is_empty() { "none".to_string() } else { accordion_value() },
            h2 { class: "text-sm font-medium", "Accordion interaction" }
            Accordion {
              class: "mt-3",
              // Uncontrolled: the root owns the open item and the fixture records it.
              on_value_change: move |value| accordion_value.set(value),
              AccordionItem { value: "shipping",
                AccordionTrigger { "Shipping" }
                AccordionContent { "Ships in two days." }
              }
              AccordionItem { value: "returns",
                AccordionTrigger { disabled: true, "Returns" }
                AccordionContent { "Returns within 30 days." }
              }
              AccordionItem { value: "warranty",
                AccordionTrigger { "Warranty" }
                AccordionContent { "Covered for one year." }
              }
              AccordionItem { value: "support",
                AccordionTrigger { "Support" }
                AccordionContent { "Email us any time." }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "context-menu",
            "data-action": "{context_action}",
            "data-bookmarked": "{context_bookmarked}",
            h2 { class: "text-sm font-medium", "Context menu interaction" }
            ContextMenu {
              ContextMenuTrigger {
                class: "mt-3 h-24 border border-dashed border-input",
                "data-interaction-control": "context-area",
                "Right-click here"
              }
              ContextMenuContent {
              ContextMenuItem { onclick: move |_| context_action.set("back"), "Back" }
              ContextMenuCheckboxItem {
                checked: context_bookmarked(),
                onclick: move |_| context_bookmarked.toggle(),
                "Bookmark"
              }
              ContextMenuItem {
                disabled: true,
                onclick: move |_| context_action.set("forward"),
                "Forward"
              }
              ContextMenuSub {
                ContextMenuSubTrigger { "More tools" }
                ContextMenuSubContent {
                  ContextMenuItem { onclick: move |_| context_action.set("save"), "Save page" }
                }
              }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "toast",
            "data-state": if toast_open() { "open" } else { "closed" },
            "data-reason": "{toast_reason}",
            h2 { class: "text-sm font-medium", "Toast interaction" }
            button {
              class: "{secondary_button_class} mt-3",
              "data-interaction-control": "toast-trigger",
              onclick: move |_| toast_open.set(true),
              "Show toast"
            }
            ToastViewport {
              ToastRoot {
                open: toast_open(),
                duration_ms: 1500,
                on_dismiss: move |reason| {
                  toast_reason.set(toast_dismiss_reason_attribute(reason));
                  toast_open.set(false);
                },
                ToastTitle { "Changes saved" }
                ToastAction {
                  on_dismiss: move |reason| {
                    toast_reason.set(toast_dismiss_reason_attribute(reason));
                    toast_open.set(false);
                  },
                  "Undo"
                }
                ToastClose {
                  on_dismiss: move |reason| {
                    toast_reason.set(toast_dismiss_reason_attribute(reason));
                    toast_open.set(false);
                  },
                  "×"
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "sonner",
            "data-reason": "{sonner_reason}",
            h2 { class: "text-sm font-medium", "Sonner interaction" }
            button {
              class: "{secondary_button_class} mt-3",
              "data-interaction-control": "sonner-trigger",
              onclick: move |_| sonner_open.set(true),
              "Show sonner"
            }
            SonnerViewport {
              if sonner_open() {
                SonnerToast {
                  variant: SonnerVariant::Success,
                  duration_ms: 1500,
                  on_dismiss: move |reason| {
                    sonner_reason.set(sonner_dismiss_reason_attribute(reason));
                    sonner_open.set(false);
                  },
                  SonnerContent {
                    SonnerTitle { "Event created" }
                  }
                  SonnerClose {
                    on_dismiss: move |reason| {
                      sonner_reason.set(sonner_dismiss_reason_attribute(reason));
                      sonner_open.set(false);
                    },
                    "×"
                  }
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "tooltip",
            "data-state": if tooltip_open() { "open" } else { "closed" },
            h2 { class: "text-sm font-medium", "Tooltip interaction" }
            Tooltip {
              on_open_change: move |open| tooltip_open.set(open),
              TooltipTrigger { class: "{secondary_button_class} mt-3", "Hover for tooltip" }
              TooltipContent { "Saved 2 minutes ago" }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "hover-card",
            "data-state": if hover_card_open() { "open" } else { "closed" },
            h2 { class: "text-sm font-medium", "Hover card interaction" }
            HoverCard {
              on_open_change: move |open| hover_card_open.set(open),
              // A link in a line of text, which the touch target size exempts.
              p { class: "mt-3 text-sm",
                "Built with "
                HoverCardTrigger { href: "#hover-card-dioxus", class: "font-medium underline", "@dioxus" }
              }
              HoverCardContent {
                HoverCardHeader {
                  HoverCardTitle { "Dioxus" }
                  HoverCardDescription { "Fullstack app framework for Rust." }
                }
                a {
                  class: "mt-2 inline-block text-sm underline",
                  href: "#hover-card-profile",
                  "View profile"
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "alert-dialog",
            "data-state": if alert_dialog_open() { "open" } else { "closed" },
            "data-result": "{alert_dialog_result}",
            h2 { class: "text-sm font-medium", "Alert dialog interaction" }
            // Uncontrolled: the root owns `open` and the article follows it
            // through `on_open_change`.
            AlertDialog { on_open_change: move |open| alert_dialog_open.set(open),
              AlertDialogTrigger {
                class: "{secondary_button_class} mt-3",
                "data-interaction-control": "alert-dialog-trigger",
                "Delete project"
              }
              AlertDialogOverlay {}
              AlertDialogContent { "aria-label": "Confirm deletion",
                AlertDialogTitle { "Delete project?" }
                AlertDialogDescription { "This cannot be undone." }
                AlertDialogCancel { "Cancel" }
                AlertDialogAction {
                  variant: AlertDialogActionVariant::Destructive,
                  onclick: move |_| alert_dialog_result.set("confirmed"),
                  "Delete"
                }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "dialog",
            "data-state": if dialog_open() { "open" } else { "closed" },
            h2 { class: "text-sm font-medium", "Dialog interaction" }
            button {
              class: "{secondary_button_class} mt-3",
              "data-interaction-control": "dialog-trigger",
              onclick: move |_| dialog_open.set(true),
              "Open dialog"
            }
            // Controlled: the app's button above opens it.
            Dialog { open: dialog_open(), on_open_change: move |open| dialog_open.set(open),
              DialogOverlay {
                dismiss: DismissBehavior { outside_pointer: true, ..DismissBehavior::dialog_default() },
              }
              DialogContent {
                DialogTitle { "Rename project" }
                DialogDescription { "Focus stays inside until the dialog closes." }
                input {
                  class: "rounded-md border border-border px-2 py-1 text-sm",
                  "aria-label": "Project name",
                  "data-interaction-control": "dialog-input",
                }
                DialogClose { "Cancel" }
              }
            }
          }
          // Table rule lines take the border token in both themes.
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "table-borders",
            h2 { class: "text-sm font-medium", "Table border interaction" }
            Table {
              TableHeader { TableRow { TableHead { "Plan" } TableHead { "Seats" } } }
              TableBody {
                TableRow { TableCell { "Team" } TableCell { "5" } }
                TableRow { TableCell { "Business" } TableCell { "20" } }
              }
              TableFooter { TableRow { TableCell { "Total" } TableCell { "25" } } }
            }
          }
          // Content that is a sibling of its trigger in a flex row leaves the
          // row before the trigger is measured.
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "flex-row-overlay",
            h2 { class: "text-sm font-medium", "Flex row overlay interaction" }
            div { class: "mt-3 flex items-center justify-end gap-2",
              Dropdown {
                DropdownTrigger { class: "{secondary_button_class}", "Account" }
                DropdownContent {
                  DropdownItem { "Profile" }
                  DropdownItem { "Sign out" }
                }
              }
              button { class: "{secondary_button_class}", "Help" }
            }
          }
          // A Command mounted inside a closed Dialog reads keys once it opens,
          // and again after a close and a reopen.
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "command-dialog",
            h2 { class: "text-sm font-medium", "Command dialog interaction" }
            Dialog {
              DialogTrigger { class: "{secondary_button_class} mt-3", "Open palette" }
              DialogOverlay {}
              DialogContent {
                DialogTitle { "Palette" }
                Command {
                  CommandInput { placeholder: "Search palette..." }
                  CommandList {
                    CommandItem { id: "palette-new", value: "new", "New file" }
                    CommandItem { id: "palette-open", value: "open", "Open file" }
                  }
                }
                DialogClose { "Done" }
              }
            }
          }
          // One Escape closes only the overlay opened last.
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "nested-layers",
            h2 { class: "text-sm font-medium", "Nested layer interaction" }
            Dialog {
              DialogTrigger { class: "{secondary_button_class} mt-3", "Open settings" }
              DialogOverlay {}
              DialogContent {
                DialogTitle { "Settings" }
                Popover {
                  PopoverTrigger { class: "{secondary_button_class}", "Sharing" }
                  PopoverContent { PopoverTitle { "Share link" } }
                }
                DialogClose { "Done" }
              }
            }
          }
          article {
            class: "rounded-md border border-border p-4",
            "data-interaction-target": "action-parts",
            "data-last-action": "{last_part_action}",
            "data-submits": "{part_form_submits}",
            h2 { class: "text-sm font-medium", "Action part interaction" }
            div { class: "mt-3 flex flex-wrap items-center gap-3",
              ButtonGroup { aria_label: "Formatting",
                ButtonGroupItem { onclick: move |_| last_part_action.set("button-group".to_string()), "Bold" }
                ButtonGroupItem {
                  disabled: true,
                  onclick: move |_| last_part_action.set("disabled".to_string()),
                  "Italic"
                }
              }
              InputGroup { class: "max-w-56",
                InputGroupControl {
                  // An element the app renders takes the density itself (RFC 0078).
                  input {
                    class: "h-10 w-full px-3 outline-none {density_control_class(use_density())}",
                    "aria-label": "Part search",
                  }
                }
                InputGroupAction { onclick: move |_| last_part_action.set("input-group".to_string()), "Clear" }
              }
              AttachmentTrigger { onclick: move |_| last_part_action.set("attachment-trigger".to_string()), "Attach" }
              AttachmentAction { onclick: move |_| last_part_action.set("attachment-action".to_string()), "Remove" }
              Combobox {
                ComboboxTrigger {
                  class: "max-w-48",
                  "aria-label": "Fruit",
                  onclick: move |_| last_part_action.set("combobox".to_string()),
                  "Pick a fruit"
                }
                ComboboxContent {
                  ComboboxList { ComboboxItem { value: "apple", "Apple" } }
                }
              }
              Tooltip {
                TooltipTrigger {
                  class: secondary_button_class.clone(),
                  onclick: move |_| last_part_action.set("tooltip".to_string()),
                  "Copy link"
                }
                TooltipContent { "Copied" }
              }
              Field { class: "max-w-56",
                FieldLabel { r#for: "action-part-email", "Contact email" }
                Input { id: "action-part-email", r#type: "email" }
              }
              // A jump button inside a form must not submit it.
              form {
                onsubmit: move |event| {
                  event.prevent_default();
                  part_form_submits += 1;
                },
                MessageScrollerJumpButton {
                  visible: true,
                  onclick: move |_| last_part_action.set("jump".to_string()),
                  "Jump to latest"
                }
              }
            }
          }
        }
        section {
          class: "grid grid-cols-1 gap-3 md:grid-cols-2 xl:grid-cols-3",
          "data-preview-panel": "inventory",
          section {
            class: "grid grid-cols-1 gap-2 md:col-span-2 xl:col-span-3",
            "aria-label": "Rendered component coverage targets",
            for target in COMPONENT_PREVIEW_TARGETS {
              article {
                key: "{target.test_id}",
                class: "rounded-md border border-border bg-muted p-3",
                "data-component-preview": "{target.test_id}",
                "data-component": "{target.component}",
                "data-component-panel": "{target.panel}",
                "data-component-coverage": "{target.coverage_level}",
                h2 { class: "text-sm font-medium text-foreground", "{target.label}" }
                p { class: "mt-1 text-xs text-muted-foreground", "{target.notes}" }
              }
            }
          }
          for state in states {
            article {
              key: "{state.label}",
              class: "rounded-md border border-border bg-background p-4 shadow-sm",
              "data-preview-state": "{state.label}",
              h2 { class: "text-sm font-medium text-foreground", "{state.label}" }
              code {
                class: "mt-3 block overflow-hidden text-ellipsis whitespace-nowrap rounded bg-muted px-2 py-1 text-xs text-muted-foreground",
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
        marker_class: "text-primary",
        chart_id: "revenue",
        chart_label: "Revenue",
        chart_points: vec![
          ChartPoint::new(0.0, 12.0),
          ChartPoint::new(1.0, 18.0),
          ChartPoint::missing(2.0),
          ChartPoint::new(3.0, 24.0),
        ],
        chart_width: 640.0,
        chart_height: 320.0,
        direction: TextDirection::Rtl,
        direction_class: "block",
        collapsible_open: false,
        collapsible_class: "max-w-sm",
      },
      // Mobile reuses the Desktop states: both render in a native WebView shell.
      PreviewTarget::Desktop | PreviewTarget::Mobile => Self {
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
        jump_button_class: "text-destructive",
        marker_variant: MarkerVariant::Separator,
        marker_class: "text-destructive",
        chart_id: "cost",
        chart_label: "Cost",
        chart_points: vec![
          ChartPoint::new(0.0, 8.0),
          ChartPoint::new(1.0, 11.0),
          ChartPoint::new(2.0, 10.0),
          ChartPoint::new(3.0, 13.0),
        ],
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

// (group, value, label, disabled)
const INTERACTION_COMMANDS: [(&str, &str, &str, bool); 6] = [
  ("Suggestions", "calendar", "Calendar", false),
  ("Suggestions", "emoji", "Search Emoji", false),
  ("Suggestions", "calculator", "Calculator", true),
  ("Settings", "profile", "Profile", false),
  ("Settings", "billing", "Billing", false),
  ("Settings", "settings", "Settings", false),
];

const INTERACTION_FRUITS: &[(&str, &str, bool)] = &[
  ("apple", "Apple", false),
  ("apricot", "Apricot", true),
  ("banana", "Banana", false),
  ("blueberry", "Blueberry", false),
  ("cherry", "Cherry", false),
];

fn result_count_text(count: usize) -> String {
  match count {
    0 => "No results".to_string(),
    1 => "1 result".to_string(),
    count => format!("{count} results"),
  }
}

fn fruit_label(value: &str) -> &'static str {
  INTERACTION_FRUITS
    .iter()
    .find(|(fruit, _, _)| *fruit == value)
    .map(|(_, label, _)| *label)
    .unwrap_or("")
}

fn iso_date(date: CalendarDate) -> String {
  format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
}
