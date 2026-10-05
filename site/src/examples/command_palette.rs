use dioxus::prelude::*;
use dioxus_ui::{
  Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandLabel, CommandList,
  CommandShortcut, command_matches,
};

const COMMANDS: [(&str, &str, &str, &str); 5] = [
  ("Suggestions", "calendar", "Calendar", ""),
  ("Suggestions", "search", "Search", "Ctrl K"),
  ("Settings", "profile", "Profile", "Ctrl P"),
  ("Settings", "billing", "Billing", ""),
  ("Settings", "settings", "Settings", "Ctrl S"),
];

#[component]
pub fn Demo() -> Element {
  let mut query = use_signal(String::new);
  let mut chosen = use_signal(|| "nothing".to_string());
  let visible = move |label: &str| command_matches(label, &query());

  rsx! {
    Command {
      class: "max-w-sm rounded-md border border-border",
      on_select: move |value: String| chosen.set(value),
      CommandInput {
        value: query(),
        placeholder: "Type a command...",
        oninput: move |event: FormEvent| query.set(event.value()),
      }
      CommandList {
        if !COMMANDS.iter().any(|(_, _, label, _)| visible(label)) {
          CommandEmpty { "No results found." }
        }
        for group in ["Suggestions", "Settings"] {
          if COMMANDS.iter().any(|(item_group, _, label, _)| *item_group == group && visible(label)) {
            CommandGroup { key: "{group}",
              CommandLabel { "{group}" }
              for (_, value, label, shortcut) in COMMANDS
                .iter()
                .filter(|(item_group, _, label, _)| *item_group == group && visible(label))
              {
                CommandItem { key: "{value}", id: "command-palette-{value}", value: *value,
                  "{label}"
                  if !shortcut.is_empty() {
                    CommandShortcut { "{shortcut}" }
                  }
                }
              }
            }
          }
        }
      }
    }
    p { class: "mt-3 text-sm text-muted-foreground", "Selected: {chosen}" }
  }
}
