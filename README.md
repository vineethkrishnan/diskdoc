# DiskDoctor (dru) 🩺

**DiskDoctor** is an interactive, cross-platform terminal application that analyzes disk usage, identifies reclaimable space, and safely guides you through cleanup. Built in Rust for speed and safety.

[**Get Started Immediately →**](GETTING_STARTED.md)

## 🚀 Features

- **Interactive TUI**: Navigate your file system with a modern, responsive terminal interface.
- **Blazing Fast**: Uses parallel directory walking to scan gigabytes in seconds.
- **Smart Analysis**: Automatically explicitly detects:
    - 🐳 Docker containers, images, and volumes
    - 📦 &lt;package&gt; caches (npm, cargo, etc.)
    - 🪵 Log files
    - 🏗️ Build artifacts (`target/`, `node_modules/`)
- **Safe Cleanup**: Never auto-deletes. Guided confirmation workflow to prevent accidents.

## 📦 Installation

### From Source

```bash
cargo install --path .
```

(Pre-built binaries for macOS and Linux coming soon)

### Running without installation

You can also run the application directly using `cargo`:

```bash
# Run on current directory
cargo run

# Run on a specific directory (note the -- separator)
cargo run -- /var/log
```

## 🛠️ Usage

Simply run the command in your terminal:

```bash
# Scan the current directory
diskdoc

# Scan a specific directory
diskdoc /var/log

# Scan your home directory
diskdoc ~
```

### Controls

- **Arrow Keys / hjkl**: Navigate
- **Enter**: Enter directory
- **Backspace / Esc**: Go up / Back
- **Space**: Select for cleanup
- **d**: Delete selected (with confirmation)
- **q**: Quit

## 🏗️ Architecture

DiskDoctor is built using **Hexagonal Architecture** (Ports and Adapters). This ensures a clean separation between the core logic and external dependencies (Filesystem, Docker, TUI).

See [ARCHITECTURE.md](ARCHITECTURE.md) for a deep dive into the layers and design decisions.

## 🤝 Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
