# Settings

A settings page with Profile and Notifications tabs, checked profile fields,
and Save and Reset buttons that follow unsaved changes.

```bash
dxui add settings
```

Components: Button, Card, Field, Input, Label, Native Select, Switch, Tabs,
Textarea.

## Behavior

- `SettingsBlock` starts from `initial` (a `Settings` value, sample values by
  default) and calls `on_save` with the edited settings.
- Save and Reset are enabled only while something changed; a status message
  says whether there are unsaved changes.
- Save checks that the name is not empty and the email contains `@`; a failed
  check shows the field's error and switches to the Profile tab. Reset
  restores the last saved settings.
- Notification switches carry their descriptions through
  `aria-describedby`.

## Accessibility Notes

Every field has a label, errors and hints are linked with
`aria-describedby`, and the tabs follow the Tabs keyboard pattern. The
save status is a `status` message, announced when it changes.
