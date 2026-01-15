# upgrade

Upgrade installed addons to their latest versions.

## Usage

```bash
# Upgrade all addons
gdam upgrade

# Upgrade a specific addon
gdam upgrade <addon-name>
```

## Description

The `upgrade` command fetches the latest version of addons from their source repositories:

1. Pulls the latest changes from the remote repository
2. Updates the addon files in your project
3. Updates the checksum in `gdam.yaml` to the new commit

## Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `addon-name` | No | Specific addon to upgrade (upgrades all if omitted) |

## Examples

### Upgrade all addons

```bash
gdam upgrade
```

Output:
```
Upgrading all addons...
  Upgrading: my-addon
    Updated from a1b2c3d to x7y8z9w
  Upgrading: another-addon
    Already up to date
All addons upgraded successfully!
```

### Upgrade specific addon

```bash
gdam upgrade my-addon
```

Output:
```
Upgrading addon: my-addon
  Updated from a1b2c3d to x7y8z9w
Addon upgraded successfully!
```

## Notes

- Upgrades always pull from the default branch (usually `main` or `master`)
- The previous version's checksum is replaced with the new one
- If an addon is already at the latest version, it will be skipped
- Consider committing your `gdam.yaml` after upgrading to track the new versions
