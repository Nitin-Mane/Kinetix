# Kinetix Development Roadmap

## Phase 1a — Foundation (Completed)
- [x] Language specification (LANGUAGE_SPEC.md)
- [x] Workspace structure (Cargo.toml)
- [x] Lexer (kinetix_lexer)
- [x] AST definitions (kinetix_ast)
- [x] Parser — Pratt parser (kinetix_parser)

## Phase 1b — Type System & IR (Completed)
- [x] Type definitions and Hindley-Milner inference (kinetix_types)
- [x] High-level IR lowering (kinetix_hir)
- [x] Mid-level IR + optimization passes (kinetix_mir)

## Phase 1c — Execution (Completed)
- [x] Bytecode format (kinetix_bytecode)
- [x] VM register-based interpreter (kinetix_vm)
- [x] GC — reference counting + cycle detection (kinetix_gc)
- [x] Code generator HIR→bytecode (kinetix_codegen)
- [x] `kxr` REPL — run .kx files interactively

## Phase 1d — JIT Compilation (Completed)
- [x] JIT backend using Cranelift (kinetix_jit)
- [x] `kxc` compiler CLI — compile .kx to bytecode/binary
- [x] Benchmark suite (examples/fibonacci.kx)

## Phase 1e — Standard Library (Completed)
- [x] std::io — console, file I/O
- [x] std::math — trig, exp, log, constants
- [x] std::matrix — full MATLAB-like matrix ops
- [x] std::vec — growable arrays
- [x] std::map — hash maps (stub)

## Phase 1f — IDE & Tooling (Completed)
- [x] LSP server (kinetix_lsp) — diagnostics, document sync
- [x] DAP debugger (kinetix_debugger) — breakpoints, stepping, inspect
- [x] Syntax highlighting bridge (kinetix_syntax)
- [x] Native desktop IDE engine (kinetix_ide)
- [x] Code formatter (kxfmt)
- [x] Package manager (kxpkg) — create, init, add, build, run & check Kinetix packages

## Phase 2 — Self-Hosting
- [ ] Rewrite kinetix_lexer in Kinetix → lex.kx
- [ ] Rewrite kinetix_parser in Kinetix → parse.kx
- [ ] Rewrite kinetix_codegen in Kinetix → codegen.kx
- [ ] Bootstrap: compile Kinetix compiler with itself
- [ ] Remove Rust bootstrap dependency

## Phase 3 — Ecosystem
- [ ] Package registry (packages.kinetix-lang.org)
- [ ] Web playground (play.kinetix-lang.org)
- [ ] Documentation site (docs.kinetix-lang.org)
- [ ] VS Code extension (syntax + LSP)
- [ ] Scientific computing library (kinetix_sci)
- [ ] GPU compute bindings (kinetix_gpu)
