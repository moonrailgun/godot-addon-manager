# init

Initialize gdam in your current Godot project.

## Usage

```bash
gdam init
```

## Description

The `init` command sets up gdam for your Godot project by:

1. **Creating `gdam.yaml`**: A configuration file to track your addons
2. **Updating `.gitignore`**: Adds `addons/` to ignore managed addons (if no addons folder exists)
3. **Detecting existing addons**: Lists any addons already in your project

## Requirements

- Must be run inside a Godot project directory (containing `project.godot`)

## Examples

### Initialize a new project

```bash
cd my-godot-project
gdam init
```

Output:
```
Created gdam.yaml
Added addons/ to .gitignore
Initialization complete!
```

### Project with existing addons

If your project already has addons, gdam will detect them:

```
Created gdam.yaml
Found existing addons that can be migrated to gdam:
  - existing-addon-1
  - existing-addon-2

Use 'gdam install' to add these addons to gdam management.
Initialization complete!
```

## Notes

- Running `init` on an already initialized project will skip and show a warning
- The command will not overwrite an existing `gdam.yaml` file
