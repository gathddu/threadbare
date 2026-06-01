# Threadbare

A customizable, lightweight e-mail client for Linux. Build with Rust for speed and reliability.

At its core, it's a modular system:

- Daemon (`threadbare-daemon`): Background service managing e-mail sync, storage and IPC.
- GUI (`threadbare-gui`): GTK4 + libadwaita interface for interactive e-mail management.
- CLI (`threadbare-cli`): Coomand-line interface for headless/scripting workflows.
- Core Library (`threadbare-core`): Shared models, database layer and configuration system.

## Setup

### Requirements

- Rust 1.70+ (via `rustup`)
- GTK4 development libraries
- SQLite3
- OpenSSL

### Development

```bash
# clone the repository
git clone https://github.com/gathddu/threadbare.git
cd threadbare

# using nix
nix flake update
nix develop

# build the binaries
cargo build --release

# run GUI
./target/release/threadbare-gui

# run daemon
./target/release/threadbare-daemon
```

### Configuration

Threadbare uses XDG Base Directory specification:

- Config: `~/.config/threadbare/config.toml`
- Database: `~/.local/share/threadbare/emails.db`
- IPC Socket: `~/.config/threadbare/threadbare.sock`

```text
[daemon]
sync_interval_secs = 300
database_path = "~/.local/share/threadbare/emails.db"

[gui]

[keybindings]
```