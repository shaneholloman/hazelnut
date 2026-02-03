# Tidy 🧹

A terminal-based automated file organizer inspired by [Hazel](https://www.noodlesoft.com/).

**Tidy** watches your folders and automatically organizes files based on rules you define — all from your terminal.

## Features

- 📁 **Watch folders** for new and changed files
- 🎯 **Flexible rules** with conditions (name, type, size, date, etc.)
- ⚡ **Actions** like move, rename, delete, archive, and run scripts
- 🖥️ **Beautiful TUI** for managing rules and monitoring activity
- 🔧 **Background daemon** that runs quietly and applies rules in real-time
- 📝 **TOML configuration** that's easy to read and edit

## Installation

```bash
# From source
cargo install --path .

# Or build locally
cargo build --release
```

## Quick Start

1. **Start the daemon** (runs in background):
   ```bash
   tidyd start
   ```

2. **Launch the TUI** to manage rules:
   ```bash
   tidy
   ```

3. **Or edit config directly** at `~/.config/tidy/config.toml`:
   ```toml
   [[watch]]
   path = "~/Downloads"

   [[rule]]
   name = "Organize PDFs"
   
   [rule.condition]
   extension = "pdf"
   
   [rule.action]
   type = "move"
   destination = "~/Documents/PDFs"
   ```

## Example Rules

### Move screenshots to a folder
```toml
[[rule]]
name = "Screenshots"
[rule.condition]
name_matches = "Screenshot*.png"
[rule.action]
type = "move"
destination = "~/Pictures/Screenshots"
```

### Delete old downloads
```toml
[[rule]]
name = "Clean old downloads"
[rule.condition]
age_days = { greater_than = 30 }
[rule.action]
type = "trash"
```

### Rename files with dates
```toml
[[rule]]
name = "Date prefix invoices"
[rule.condition]
name_matches = "invoice*.pdf"
[rule.action]
type = "rename"
pattern = "{date:YYYY-MM-DD}_{name}"
```

## Architecture

- **`tidy`** - TUI for managing rules and monitoring
- **`tidyd`** - Background daemon that watches and acts

They communicate via Unix socket, so you can start/stop/configure the daemon from the TUI.

## Configuration

Config location: `~/.config/tidy/config.toml`

See [docs/configuration.md](docs/configuration.md) for full reference.

## Requirements

- Rust 1.93+
- Linux or macOS

## License

MIT

## Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
