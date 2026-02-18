# DiskDoctor (dru) Architecture

## 1. Technology Choice: Rust 🦀

We have selected **Rust** as the core language for DiskDoctor.

### 1.1 Reasoning
*   **Performance:** Disk traversal is I/O and CPU intensive. Rust's zero-cost abstractions and ability to safely parallelize directory walking (using libraries like `jwalk` or `ignore`) make it significantly faster than Python or Node.js.
*   **Memory Safety:** Handling recursive file structures and large file lists can lead to memory issues. Rust's ownership model ensures memory safety without a garbage collector causing UI stutters.
*   **Binary Portability:** Compiles to a single, static binary. No need for the user to have a specific Python version or Node.js runtime installed. This is crucial for a DevOps tool.
*   **Ecosystem:** The TUI ecosystem in Rust (specifically `ratatui`) is mature, responsive, and capable of creating modern, beautiful terminal interfaces as requested in the PRD.

---

## 2. Global Architecture: Hexagonal (Final)

DiskDoctor has evolved from a simple MVU pattern into a strict **Hexagonal Architecture** (also known as Ports and Adapters). 

### 2.1 The Four Layers
1.  **Domain (Core)**: Traits (Ports) and entities. No dependencies.
2.  **Application (Use Cases)**: State management (`App`) and orchestration.
3.  **Infrastructure (Adapters)**: Scanner, Cleaner, and Docker implementations.
4.  **Interface (Outer Layer)**: TUI (Ratatui) and CLI (Clap).

### 2.2 Dependency Rule
Dependencies only point inward: **Interface -> Application -> Domain <- Infrastructure**.

---

## 3. Data Flow

The application uses an event-driven flow where the **Infrastructure** adapters (like the Scanner) send events to the **Application** layer, which updates the state and triggers the **Interface** (TUI) to re-render.

---

## 4. Module Structure

```text
src/
├── domain/           # Ports (Traits) & Entities
│   ├── entities.rs
│   └── ports.rs
├── application/      # State & Workflow Orchestration
│   └── app.rs
├── infrastructure/   # Adapters (Scanner, Docker, Cleaner)
│   ├── scanner/      # Parallel walking & Heuristics
│   ├── cleaner.rs    # FS Deletion
│   └── docker.rs     # Docker CLI
├── interface/        # TUI/CLI Implementation
│   └── tui/          # Ratatui Views
└── main.rs           # Composition Root & DI
```

---

## 5. Security & Safety Strategy

1.  **Read-Only Default:** The scanner performs no write operations.
2.  **Explicit Confirmation:** The cleaner requires a distinct explicit User Action (confirmation modal) before any `fs::remove` call.
3.  **Scope Restriction:**
    *   Prevent deletion of critical system directories.
    *   Hardcoded "Blocklist" for sensitive OS paths.

---

For the most up-to-date and detailed architectural breakdown, please refer to the primary root [ARCHITECTURE.md](../../ARCHITECTURE.md).
