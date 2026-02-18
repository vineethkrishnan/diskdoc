# DiskDoctor (dru) Architecture

DiskDoctor follows a strict **Hexagonal Architecture** (also known as Ports and Adapters) and **Clean Architecture** principles. This ensures that the core business logic is isolated from external dependencies like the filesystem, Docker CLI, and the TUI framework.

## 1. Architectural Layers

The project is organized into four distinct layers, with dependencies always pointing inward.

### 1.1 Domain Layer (`src/domain`)
The heart of the application. It contains no external dependencies and defines the "truth" of the system.
- **Entities**: Core data structures like `FileStats`, `FileType`, and `Recommendation`.
- **Ports (Traits)**: Interfaces for external behavior, such as `Scanner`, `Cleaner`, and `Analyzer`.
- **Logic**: Pure business rules for aggregating data and generating recommendations.

### 1.2 Application Layer (`src/application`)
Orchestrates the use cases and manages application state.
- **App State**: The `App` struct holds the current state, navigation history, and active view.
- **Dependency Injection**: Dependencies are injected into the Application layer via boxed traits (ports), allowing for easy mocking and testing.

### 1.3 Infrastructure Layer (`src/infrastructure`)
Contains concrete implementations (Adapters) of the ports defined in the Domain.
- **Scanner**: Uses `jwalk` for high-performance parallel directory walking.
- **Cleaner**: Handles physical file/directory deletion via `std::fs`.
- **Docker Analyzer**: Interacts with the Docker CLI to identify reclaimable space.
- **Heuristics**: Logic for identifying specific file types (Logs, Caches, etc.).

### 1.4 Interface Layer (`src/interface`)
The outer layer responsible for user interaction.
- **TUI (Ratatui)**: Implements the interactive terminal interface.
- **CLI (Clap)**: Handles command-line argument parsing and initialization.

---

## 2. Dependency Direction

> [!IMPORTANT]
> **Interface → Application → Domain**  
> **Infrastructure → Domain**

The Domain layer is completely isolated and does not know about the Infrastructure, Application, or Interface layers.

---

## 3. Module Structure

```text
src/
├── domain/           # Core Entities and Ports
│   ├── entities.rs   # FileStats, Recommendation, etc.
│   └── ports.rs      # Traits (Scanner, Cleaner, Analyzer)
├── application/      # Orchestration and State
│   └── app.rs        # Main App implementation
├── infrastructure/   # Concrete Adapters
│   ├── scanner/      # FS Walking & Heuristics
│   ├── cleaner.rs    # FS Deletion
│   └── docker.rs     # Docker CLI interaction
├── interface/        # Entry points and TUI
│   ├── tui/          # Ratatui implementation
│   └── mod.rs        # Interface root
└── main.rs           # Composition Root (Dependency Injection)
```

---

## 4. Why Hexagonal?

1.  **Testability**: We can test the entire application logic using mocks without ever touching the actual filesystem or Docker.
2.  **Scalability**: Adding a new interface (e.g., a web UI or a REST API) or a new infrastructure adapter (e.g., a cloud storage scanner) requires no changes to the core logic.
3.  **Stability**: Changes in external libraries (like moving from `jwalk` to another walker) only affect a single adapter in the infrastructure layer.
