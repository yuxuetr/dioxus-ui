# Files

A file manager: a folder tree, breadcrumbs for the open folder, an upload
area that takes dropped or picked files, and the folder's files in a table
with a context menu.

```bash
dxui add files
```

Components: Breadcrumb, Context Menu, Data Table, Tree.

## Behavior

- `FilesBlock` takes `on_upload`, called with the `FileEntry`s (`name`,
  `folder`, `size`, `modified`) added to a folder, and `on_delete`, called
  with a deleted file.
- Choosing a folder in the tree, or an ancestor in the breadcrumbs, opens it;
  opening a folder from the breadcrumbs also opens its parents in the tree.
- Files dropped on the upload area, which shows a dashed primary outline
  while files are dragged over it, or picked with Upload, are added to the
  open folder.
- Right-clicking a file name opens a context menu that deletes that file.
- The folders come from `FOLDERS` (id, name, parent) and the files from
  `sample_files`. The block reads only each file's name and size: store the
  bytes in `on_upload`.

## Accessibility Notes

The tree is named "Folders" and follows the [Tree](../components/tree.md)
keyboard model. The breadcrumbs are a navigation whose ancestors are buttons
and whose open folder is the current page. The upload area is a region named
"Upload area"; Upload is a labeled native file picker, which keyboard users
reach instead of dropping. The table has a caption naming the folder. The
context menu opens on a right-click only: its trigger takes no focus, so
give keyboard users another way to delete, such as a row action button,
before shipping the screen.
