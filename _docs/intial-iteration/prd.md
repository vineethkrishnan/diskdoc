# DiskDoctor (diskdoc)
## Product Requirements Document (PRD)
### Version 1 – Codename: dru

---

# 1. Product Overview

## Product Name
DiskDoctor  
CLI Command: `diskdoc`

## Vision
An interactive, cross-platform terminal application that analyzes disk usage, identifies reclaimable space, and safely guides users through cleanup — with a modern and responsive terminal UI.

## Codename
dru  
(Goal: Lift disk pressure from systems.)

---

# 2. Target Users

## Primary Users
- Backend developers
- DevOps engineers
- Docker users
- VPS / cloud maintainers
- macOS developers with heavy build caches

## Secondary Users
- Power users
- Self-hosted hobbyists

---

# 3. Supported Platforms (v1)

- macOS (Intel + Apple Silicon)
- Linux (Ubuntu, Debian, RHEL-based)
- Windows (Not supported in v1)

---

# 4. Core Objectives (dru)

1. Provide real-time interactive disk overview
2. Identify largest directories & files
3. Detect common reclaimable areas:
   - Docker
   - Logs
   - Package caches
   - Build artifacts
4. Offer safe guided cleanup
5. Show progress + estimated reclaim size
6. Never auto-delete without confirmation

---

# 5. Command Behavior

## Default Mode (Interactive TUI)

```bash
diskdoc
