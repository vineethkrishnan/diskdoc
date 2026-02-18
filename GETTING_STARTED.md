# Getting Started with DiskDoctor (dru) 🩺

Welcome to DiskDoctor! This guide will help you get up and running quickly.

## 📋 Prerequisites

Before you begin, ensure you have the following installed:
- **Rust Toolchain**: You need `cargo` and `rustc`. Install them from [rustup.rs](https://rustup.rs/).
- **Docker** (Optional): To use the Docker analysis features.

## 📥 Installation

### Build from Source
Clone the repository and install the binary locally:

```bash
git clone https://github.com/vineethkrishnan/diskdoc.git
cd diskdoc
cargo install --path .
```

This will install the `diskdoc` command into your `~/.cargo/bin` directory. Ensure this directory is in your `PATH`.

## 🚀 Quick Start

Launch DiskDoctor by just typing the command:

```bash
# Analyze the current directory
diskdoc

# Analyze a specific path
diskdoc ~/Downloads
```

### Main Views

1. **Dashboard (Press [1])**: See a high-level summary of reclaimable space (Logs, Caches, Docker).
2. **File Browser (Press [2])**: Navigate the file system and identify large files/directories.

## ⌨️ Controls & Shortcuts

| Key | Action |
|-----|--------|
| `1` | Switch to Dashboard |
| `2` | Switch to File Browser |
| `↑/↓` (or `k/j`) | Navigate list |
| `Enter` | Enter directory / Open details |
| `Esc` / `Backspace` | Go up / Go back |
| `s` | Toggle Sort (Size Asc/Desc) |
| `d` | Delete selected item (with confirmation) |
| `?` | Show help / About |
| `q` | Quit application |

## 🛠️ Development Guide

If you want to contribute to DiskDoctor, follow these steps:

### Running Tests
We maintain a suite of unit and integration tests:
```bash
cargo test
```

### Pre-commit Hooks
We use a pre-commit hook to ensure code quality (formatting, linting, tests). Install it using:
```bash
./.hooks/install.sh
```

### Architecture
DiskDoctor follows a **Hexagonal Architecture**. 
- `src/domain`: Core entities and ports.
- `src/application`: Application logic and state management.
- `src/infrastructure`: Adapters for filesystem and Docker.
- `src/interface`: Terminal UI implementation.

---
*For more detailed information, check out the [Architecture.md](_docs/intial-iteration/architecture.md).*
