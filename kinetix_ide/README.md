# Kinetix IDE

> A native desktop IDE for the **Kinetix** programming language — inspired by C++, Rust, and MATLAB.

![Kinetix IDE](assets/icon.png)

## Overview

Kinetix IDE is a full-featured native desktop integrated development environment (IDE) built in Rust using the `egui`/`eframe` GUI framework. It provides a rich editing experience for the Kinetix scripting language.

### Features

- 🖊️ **Multi-tab code editor** with Kinetix syntax highlighting and line numbers
- 📁 **File explorer** sidebar — open, create, rename, and delete files and folders
- 💻 **Integrated terminal** — PTY-backed shell panel at the bottom
- 📤 **Output panel** — captures build, run, and error output
- 🔍 **Language server stub** — diagnostics, hover info, and auto-completion foundations
- 🎨 **Dark & Light themes** — Kinetix brand palette (deep teal + amber)
- ⚙️ **Settings dialog** — font size, theme, tab size, word wrap

## The Kinetix Language

Kinetix is a dynamically-typed scripting language designed for clarity and expressiveness, drawing from:

- **C++** — structs, methods, type annotations
- **Rust** — `let`/`mut`, pattern matching, `impl` blocks
- **MATLAB** — matrix literals, numerical operations, range expressions

```kinetix
// Kinetix sample
import std::io
import std::math

struct Point {
    x: float,
    y: float,
}

impl Point {
    fn new(x: float, y: float) -> Point {
        Point { x, y }
    }

    fn distance(self, other: Point) -> float {
        let dx = self.x - other.x
        let dy = self.y - other.y
        math::sqrt(dx * dx + dy * dy)
    }
}

fn main() {
    let p1 = Point::new(0.0, 0.0)
    let p2 = Point::new(3.0, 4.0)
    io::println("Distance: {p1.distance(p2)}")

    // Matrix literal (MATLAB-inspired)
    let mat = matrix[[1, 2, 3]; [4, 5, 6]; [7, 8, 9]]
}
```

## Project Structure

```
kinetix_ide/
├── Cargo.toml              # Workspace root
├── build.rs                # Build script (embeds assets)
├── assets/
│   ├── icon.png
│   ├── fonts/              # JetBrains Mono
│   └── themes/             # dark.toml / light.toml
└── crates/
    ├── kinetix_lexer/      # Kinetix tokenizer
    ├── kinetix_syntax/     # egui syntax highlighting
    ├── kinetix_lsp/        # Language server stub
    └── kinetix_ide/        # Main IDE application
```

## Building

### Prerequisites

- **Rust** 1.78+ (install via [rustup](https://rustup.rs))
- On Linux: `libxcb`, `libxkbcommon` dev packages

### Build & Run

```bash
# Development build
cargo build

# Run the IDE
cargo run -p kinetix_ide

# Release build
cargo build --release
```

### Tests

```bash
cargo test -p kinetix_lexer
cargo test -p kinetix_syntax
cargo test              # Run all workspace tests
```

## Architecture

```
┌────────────────────────────────────────────────┐
│                  kinetix_ide                   │  ← Main app, GUI
│  ┌──────────┐  ┌─────────────┐  ┌──────────┐  │
│  │ Sidebar  │  │   Editor    │  │ Terminal │  │
│  │ (file    │  │ (multi-tab, │  │ (PTY)    │  │
│  │  tree)   │  │  highlight) │  │          │  │
│  └──────────┘  └─────────────┘  └──────────┘  │
└────────┬──────────────┬───────────────┬────────┘
         │              │               │
   kinetix_syntax  kinetix_lsp    kinetix_lexer
   (highlighting)  (diagnostics)  (tokenizer)
```

## License

MIT — see [LICENSE](LICENSE)
