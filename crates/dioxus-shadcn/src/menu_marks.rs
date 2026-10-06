//! Marks for the checkbox and radio items of Dropdown, Context Menu, and
//! Menubar, drawn in the item's inset before its label, and the chevron of
//! their sub triggers.

/// A check mask, the same as the Select and Combobox check mark.
pub const MENU_CHECKBOX_MARK_CLASS: &str = "before:absolute before:start-2 before:size-4 before:bg-current before:[mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27black%27%20stroke-width=%272%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Cpath%20d=%27M3.5%208.5l3%203%206-7%27/%3E%3C/svg%3E)_center/contain_no-repeat]";
pub const MENU_RADIO_MARK_CLASS: &str =
  "before:absolute before:start-3 before:size-2 before:rounded-full before:bg-current";

/// Shows the mark of a checked item and hides that of an unchecked one.
pub fn menu_mark_state_class(checked: bool) -> &'static str {
  if checked { "before:opacity-100" } else { "before:opacity-0" }
}

/// A chevron at the end of a sub trigger, mirrored in right-to-left, and the
/// highlight that stays while its submenu is open.
pub const MENU_SUB_TRIGGER_CLASS: &str = "data-[state=open]:bg-accent after:ms-auto after:size-4 after:shrink-0 after:bg-current rtl:after:rotate-180 after:[mask:url(data:image/svg+xml,%3Csvg%20xmlns=%27http://www.w3.org/2000/svg%27%20viewBox=%270%200%2016%2016%27%20fill=%27none%27%20stroke=%27black%27%20stroke-width=%272%27%20stroke-linecap=%27round%27%20stroke-linejoin=%27round%27%3E%3Cpath%20d=%27M6%203.5l4.5%204.5-4.5%204.5%27/%3E%3C/svg%3E)_center/contain_no-repeat]";
