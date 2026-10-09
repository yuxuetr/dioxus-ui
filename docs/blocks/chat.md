# Chat

A chat screen: a conversation list, the open conversation's messages with
bubbles and attachments, and a composer that takes typed text and files
picked or dropped on it.

```bash
dxui add chat
```

Components: Attachment, Avatar, Bubble, Button, File Input, Item, Message,
Message Scroller, Textarea.

## Behavior

- `ChatBlock` takes `on_send`, called with a `ChatSend` (`conversation`,
  `text`, and `files`, each a `ChatFile` with `name` and `size`) for every
  message sent.
- Enter sends and Shift+Enter starts a new line; Send does the same. A
  message needs text or at least one file.
- Files come from Attach, a native file picker, or from a drop anywhere on
  the composer, a `FileDropzone` that marks itself while files are dragged
  over it. Each pending file has a remove button until the message is sent.
- The transcript is a reversed column, so the newest message stays in view
  as messages arrive, with no scroll script.
- The block reads only each file's name and size. Upload the files in
  `on_send`, and replace `sample_conversations` with your data.

## Accessibility Notes

The conversation list is a navigation named "Conversations" whose open
conversation has `aria-current`; it is hidden on narrow screens, where the
open conversation fills the screen. The transcript is a `log` named
"Messages", so screen readers announce new messages without moving focus.
The composer is a form named "Message composer"; its text area is named
"Message", the hidden file input is labeled by Attach and shows a focus ring
on the label, pending files are a group named "Files to send", and each
remove button names its file.
