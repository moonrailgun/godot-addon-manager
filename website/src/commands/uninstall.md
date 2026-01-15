# uninstall

Remove an installed addon from your project.

## Usage

```bash
gdam uninstall <name>
```

## Description

The `uninstall` command removes an addon from your project by:

1. Deleting the addon folder from `addons/`
2. Removing the addon entry from `gdam.yaml`
3. Cleaning up the `.gitignore` entry

## Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `name` | Yes | The addon name or source URL to uninstall |

## Examples

### Uninstall by name

```bash
gdam uninstall my-addon
```

Output:
```
Uninstalling addon: my-addon
Removed addon folder
Updated gdam.yaml
Addon 'my-addon' uninstalled successfully!
```

### Uninstall by source URL

```bash
gdam uninstall https://github.com/user/my-addon
```

## Notes

- The addon name must match exactly as shown in `gdam list`
- Uninstalling removes all files in the addon folder
- The operation cannot be undone - reinstall with `gdam install` if needed
