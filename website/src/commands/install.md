# install

Install addons from Git repositories or restore all addons from configuration.

## Usage

```bash
# Install a specific addon
gdam install <git-url>

# Install all addons from gdam.yaml
gdam install
```

## Description

The `install` command handles addon installation in two modes:

### Install from URL

When provided with a Git repository URL, gdam will:

1. Clone/fetch the repository to the local cache
2. Find all addons in the repository's `addons/` directory
3. Copy each addon to your project's `addons/` folder
4. Parse `plugin.cfg` for version information
5. Update `gdam.yaml` with addon metadata
6. Add entries to `.gitignore`

### Install all dependencies

When run without arguments, gdam will:

1. Read `gdam.yaml` for configured addons
2. Fetch each addon from its source repository
3. Checkout the specific commit (checksum) for version consistency
4. Copy addons to your project

## Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `git-url` | No | Git repository URL containing the addon(s) |

## Examples

### Install from GitHub

```bash
gdam install https://github.com/user/godot-addon
```

Output:
```
Installing addon from: https://github.com/user/godot-addon
Fetching repository...
  Installing: my-addon
Installed 1 addon(s) successfully!
```

### Install all configured addons

```bash
gdam install
```

Output:
```
Installing 3 addon(s)...
  Installing: addon-1 (1.0.0)
  Installing: addon-2 (2.1.0)
  Installing: addon-3 (0.5.0)
All addons installed successfully!
```

## Supported URL Formats

- `https://github.com/user/repo`
- `https://github.com/user/repo.git`
- `git@github.com:user/repo.git`

## Notes

- The repository must contain an `addons/` directory at its root
- Existing addon folders will be replaced during installation
- The commit hash is stored for reproducible installations
