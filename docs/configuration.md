# Configuration Reference

Tidy uses TOML for configuration. The default config location is `~/.config/tidy/config.toml`.

## Example Configuration

```toml
[general]
log_level = "info"
log_file = "~/.local/share/tidy/tidy.log"
dry_run = false
debounce_seconds = 2

# Watch folders
[[watch]]
path = "~/Downloads"
recursive = false

[[watch]]
path = "~/Desktop"
recursive = false
rules = ["screenshots"]  # Only apply specific rules

# Rules
[[rule]]
name = "Move PDFs"
enabled = true

[rule.condition]
extension = "pdf"

[rule.action]
type = "move"
destination = "~/Documents/PDFs"

[[rule]]
name = "screenshots"
enabled = true

[rule.condition]
name_matches = "Screenshot*.png"

[rule.action]
type = "move"
destination = "~/Pictures/Screenshots"

[[rule]]
name = "Clean old files"
enabled = true

[rule.condition]
age_days_greater_than = 30
extensions = ["tmp", "log", "bak"]

[rule.action]
type = "trash"
```

## General Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `log_level` | string | `"info"` | Logging level (trace, debug, info, warn, error) |
| `log_file` | string | none | Path to log file |
| `dry_run` | bool | `false` | Preview actions without executing |
| `debounce_seconds` | int | `2` | Wait time before processing a file |

## Watch Configuration

```toml
[[watch]]
path = "~/Downloads"
recursive = false
rules = []  # Empty = all rules apply
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `path` | string | required | Directory to watch |
| `recursive` | bool | `false` | Watch subdirectories |
| `rules` | array | `[]` | Rule names to apply (empty = all) |

## Conditions

All conditions must match for a rule to apply. Omit conditions you don't need.

### File Name

```toml
[rule.condition]
name_matches = "Screenshot*.png"  # Glob pattern
name_regex = "^invoice_\\d+\\.pdf$"  # Regex pattern
```

### File Type

```toml
[rule.condition]
extension = "pdf"  # Single extension
extensions = ["jpg", "jpeg", "png", "gif"]  # Multiple extensions
is_directory = false  # true/false
is_hidden = true  # Files starting with .
```

### File Size

```toml
[rule.condition]
size_greater_than = 10485760  # > 10 MB (in bytes)
size_less_than = 1048576  # < 1 MB (in bytes)
```

### File Age

```toml
[rule.condition]
age_days_greater_than = 30  # Older than 30 days
age_days_less_than = 7  # Newer than 7 days
```

## Actions

### Move

```toml
[rule.action]
type = "move"
destination = "~/Documents/PDFs"
create_destination = true  # Create folder if missing
overwrite = false  # Don't overwrite existing files
```

### Copy

```toml
[rule.action]
type = "copy"
destination = "~/Backup"
create_destination = true
overwrite = false
```

### Rename

```toml
[rule.action]
type = "rename"
pattern = "{date}_{name}.{ext}"
```

Available pattern variables:
- `{name}` - Filename without extension
- `{filename}` - Full filename
- `{ext}` - Extension
- `{path}` - Full path
- `{dir}` - Parent directory
- `{date}` - Current date (YYYY-MM-DD)
- `{datetime}` - Current datetime (YYYY-MM-DD_HH-MM-SS)
- `{date:FORMAT}` - Custom date format

### Trash

```toml
[rule.action]
type = "trash"
```

### Delete

```toml
[rule.action]
type = "delete"
```

⚠️ **Warning**: This permanently deletes files!

### Run Command

```toml
[rule.action]
type = "run"
command = "convert"
args = ["{path}", "-resize", "50%", "{dir}/{name}_small.{ext}"]
```

### Archive

```toml
[rule.action]
type = "archive"
destination = "~/Archives"
delete_original = false
```

### Nothing

```toml
[rule.action]
type = "nothing"
```

Useful for testing conditions without performing actions.

## Rule Options

```toml
[[rule]]
name = "My Rule"
enabled = true
stop_processing = false  # If true, don't apply further rules to matched files

[rule.condition]
# ...

[rule.action]
# ...
```
