# Configuration

gdam uses a `gdam.yaml` file to track installed addons in your project.

## File Location

The `gdam.yaml` file is created in the root directory of your Godot project when you run `gdam init`.

## File Format

```yaml
version: 1
addons:
  - name: my-addon
    version: "1.0.0"
    source: https://github.com/user/repo
    checksum: abc123def456...
```

## Fields

### version

The configuration file format version. Currently `1`.

### addons

An array of installed addons, each with the following properties:

| Field | Description |
|-------|-------------|
| `name` | The addon folder name (from the repository's `addons/` directory) |
| `version` | The addon version (parsed from `plugin.cfg`) |
| `source` | The Git repository URL |
| `checksum` | The Git commit hash for version locking |

## Git Integration

gdam automatically manages your `.gitignore` file to exclude installed addons. This ensures that:

- Addon files are not committed to your repository
- Team members can install the same addon versions using `gdam install`
- Your repository stays clean and lightweight

If you have a global `addons/` entry in your `.gitignore`, gdam will skip adding individual addon entries.
