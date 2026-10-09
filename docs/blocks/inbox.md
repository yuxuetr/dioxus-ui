# Inbox

A mail screen: folders with unread counts, a searchable message list, and a
reading pane, with a handle between the list and the pane.

```bash
dxui add inbox
```

Components: Avatar, Badge, Button, Input, Item, Resizable, Scroll Area,
Separator.

## Behavior

- `InboxBlock` takes `on_reply`, called with the open message's id when
  Reply is pressed.
- A folder button shows that folder's messages and its unread count. The
  search narrows the list by sender, subject, or body as you type.
- Opening a message shows it in the pane and marks it read; Archive moves it
  to the Archive folder and closes the pane.
- The handle resizes the list between 25 and 60 percent by drag or arrow
  keys. On narrow screens the folders move above the list.
- The messages come from `sample_mail`; replace it with your data and
  connect Reply and Archive to your backend.

## Accessibility Notes

The folders are a navigation named "Folders" whose current folder has
`aria-current="page"`. The list is named "Messages"; each message is a
button with `aria-current` when it is open, and an unread message says
"Unread" to screen readers before its subject. The pane is a region named
"Reading pane" headed by the subject. The resize handle is a separator
named "Resize message list" that controls the list.
