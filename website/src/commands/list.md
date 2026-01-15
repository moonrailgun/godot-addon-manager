# list

Display all installed addons in the current project.

## Usage

```bash
gdam list
```

## Description

The `list` command reads your `gdam.yaml` configuration and displays all tracked addons with their details.

## Output Format

For each addon, the following information is displayed:

- **Name**: The addon folder name
- **Version**: Version from `plugin.cfg`
- **Source**: Git repository URL
- **Checksum**: Git commit hash

## Examples

### List installed addons

```bash
gdam list
```

Output:
```
Installed addons:
  - my-addon (1.0.0)
    Source: https://github.com/user/my-addon
    Checksum: a1b2c3d4...

  - another-addon (2.0.0)
    Source: https://github.com/user/another-addon
    Checksum: e5f6g7h8...
```

### Empty project

```bash
gdam list
```

Output:
```
No addons installed.
```

## Notes

- Requires `gdam.yaml` to exist (run `gdam init` first)
- Shows only gdam-managed addons, not manually installed ones
