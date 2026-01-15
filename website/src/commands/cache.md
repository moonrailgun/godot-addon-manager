# cache

Manage the gdam cache directory.

## Usage

```bash
# Show cache path
gdam cache path

# Clear the cache
gdam cache clear
```

## Description

gdam caches cloned Git repositories to speed up subsequent installations and upgrades. The `cache` command provides utilities to manage this cache.

## Subcommands

### path

Print the full path to the cache directory.

```bash
gdam cache path
```

Output:
```
/Users/username/.cache/gdam
```

The cache location varies by operating system:
- **Linux**: `~/.cache/gdam`
- **macOS**: `~/Library/Caches/gdam`
- **Windows**: `%LOCALAPPDATA%\gdam\cache`

### clear

Remove all cached repositories to free up disk space.

```bash
gdam cache clear
```

Output:
```
Cache cleared! Freed 150.25 MB
```

## Examples

### Check cache location

```bash
gdam cache path
```

### Free up disk space

```bash
gdam cache clear
```

## Notes

- Clearing the cache is safe - repositories will be re-cloned on the next install
- After clearing, the next `gdam install` may take longer as it needs to fetch repositories again
- The cache grows over time as you install addons from different repositories
