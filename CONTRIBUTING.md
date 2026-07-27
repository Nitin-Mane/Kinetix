# Contributing to Kinetix

Thank you for your interest in contributing to **Kinetix**! We welcome contributions from developers, researchers, and roboticists.

---

## Getting Started

### Prerequisites
- **Rust Toolchain**: 1.78 or higher (`rustup update stable`)
- **Git**

### Clone & Build

```bash
# Clone the repository
git clone https.github.com/kinetix-lang/kinetix.git
cd kinetix

# Verify workspace compilation
cargo check

# Run unit and integration tests
cargo test --workspace
```

---

## Development Workflow

1. **Fork & Branch**: Create a feature branch off `main` (`git checkout -b feature/my-feature`).
2. **Code Standards**:
   - Run `cargo fmt` to format Rust code.
   - Run `cargo clippy` to check for lints.
   - Run `cargo test --workspace` to ensure all tests pass.
3. **Commit Messages**: Use clean, descriptive commit messages (e.g. `feat(parser): add support for range pattern matching`).
4. **Pull Request**: Open a PR against `main` and describe your changes.

---

## Codebase Architecture

- `crates/kinetix_lexer` — Lexer tokenization
- `crates/kinetix_parser` — Hand-written Pratt parser
- `crates/kinetix_types` — Hindley-Milner type inference & checker
- `crates/kinetix_hir` / `kinetix_mir` — Compiler IR representations
- `crates/kinetix_bytecode` — 4-byte register instruction set
- `crates/kinetix_vm` — Register virtual machine & CallFrame stack
- `crates/kinetix_jit` — Cranelift native code generator
- `crates/kinetix_stdlib` — Standard library (`io`, `math`, `matrix`, `vec`, `map`)
- `tools/` — CLI tools (`kxc`, `kxr`, `kxfmt`, `kxpkg`)

---

## Reporting Issues

Use [GitHub Issues](https://github.com/kinetix-lang/kinetix/issues) to report bugs or request features. Please include:
- Operating system and Rust version
- Minimal reproducible `.kx` sample code
- Full error logs / stack trace
