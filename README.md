# Supervisors

A Multi Process Supervisor written in Rust

A lightweight process supervisor written in Rust. Inspired by [Supervisord](http://supervisord.org), this tool allows you to manage and monitor long-running background processes with optional file-based logging and a simple HTTP API.

---

## ✨ Features

- Start and monitor multiple child processes
- Automatically restart processes if they crash (`autorestart`)
- Log output to files or fallback to terminal
- Health and status HTTP API (`/health`, `/status`)
- Configuration via a simple `TOML` file

---

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024+)
- Unix-like OS (Linux, macOS)

---

## 🚀 Getting Started

### 1. Clone & Build

```bash
git clone https://github.com/toshsan/supervisors.git
cd supervisors
cargo build --release
```

Great question. Here's a **rational, developer-focused comparison** for why you'd build or use **This Rust-based supervisor** instead of existing tools like Supervisor, systemd, or Docker:

---

## 🤔 Why did i build this or Why would you use this instead of alternatives?

### 🔍 1. **I Need a Lightweight, Portable Supervisor**

| Tool                  | Dependencies         | Portability                     |
| --------------------  | -------------------- | ------------------------------- |
| Supervisor            | Python, config files | Unix only                       |
| systemd               | Requires system init | Linux only                      |
| Docker                | Daemon + images      | OS-agnostic, but heavy          |
| ✅ supervisors        | Standalone binary    | Cross-platform, no runtime deps |

This Docker friendly supervisor compiles to a single binary, runs on any platform with zero dependencies, and doesn't rely on system init processes.

---

### 💡 2. **Simple Config, Simple API**

- Supervisor has its own `.ini` format and XML-RPC interface
- Docker requires image building, volumes, and orchestration
- systemd is powerful but complex and Linux-specific

✅ This tool uses:

- **TOML configs** (friendly, human-readable)
- **Minimal HTTP API** (`/status`, `/health`)
- **Easy log configuration** per process

Perfect for use cases like:

- Embedded systems
- Developer tooling
- Internal process orchestration
- Test runners
- Custom deployment pipelines

---

### ⚡ 3. **Fast, Concurrent, Safe — It’s Rust**

Rust gives you:

- **True concurrency** using `tokio` (unlike Python's GIL-bound Supervisor)
- **Memory safety** and strong typing
- **Low overhead**, perfect for running long-lived daemons

You’re not building a general-purpose system init — you're creating a **focused, embeddable process runner**.

---

### 🧩 4. **Extensibility Without Complexity**

This implementation is modular:

- Add status APIs, dashboards, WebSockets, gRPC, etc.
- Embed it into other Rust apps
- Use it as a library or CLI

Try that with systemd 😉

---

### 🚫 When Not to Use This

To be fair, **this is not yet a full replacement** for:

- `systemd` if you're managing critical system services (with dependencies, sockets, targets, etc.)
- `Docker` if you need full process isolation, volume sharing, or container orchestration
- `Supervisor` if you're already deeply invested in its ecosystem and XML-RPC control

---

### ✅ Use This If You Want:

- A lightweight, custom supervisor with full Rust control
- Embeddable process monitoring in a Rust project
- Reliable restarts and logs without external services
- Simplicity over over-engineering
