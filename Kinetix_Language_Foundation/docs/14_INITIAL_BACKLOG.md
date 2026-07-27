# Initial Engineering Backlog

## Epic A — Repository and build

- Create CMake project and presets.
- Detect supported LLVM/MLIR installation.
- Add `kxc --version`.
- Configure clang-format, clang-tidy, sanitizers, and CI.
- Add test harness and FileCheck-style IR tests.

## Epic B — Source and diagnostics

- Source manager.
- Line/column mapping.
- Diagnostic engine.
- Error codes and severity.
- Source excerpt renderer.

## Epic C — Lexer

- Identifiers and keywords.
- punctuation/operators.
- base-prefixed integer literals.
- suffix validation.
- strings/chars.
- comments.
- fuzz target.

## Epic D — Parser

- modules/imports.
- declarations.
- types.
- statements.
- Pratt expression parser.
- error recovery.
- AST dump.

## Epic E — Semantic analysis

- scopes and symbols.
- primitive types.
- type checking.
- constant expressions.
- conversions.
- overflow modes.
- structures and arrays.

## Epic F — First backend

- KX MLIR module/function lowering.
- scalar arithmetic and control flow.
- LLVM dialect conversion.
- target machine and object generation.
- system linker invocation.
- execution tests.

## Epic G — C ABI

- `extern "C"` declarations.
- exported functions.
- `@repr(C)` structures.
- header generation.
- ABI test program.

## Epic H — Static math

- `vec<T,N>` and `mat<T,R,C>` types.
- shape rules.
- elementwise operations.
- matmul lowering.
- vectorization remarks.
- benchmark baseline.

## First 12 issues

1. Bootstrap CMake project with pinned LLVM detection.
2. Implement source manager and source locations.
3. Implement diagnostics renderer.
4. Implement tokens and keyword table.
5. Parse integer literals with all four bases.
6. Parse modules and functions.
7. Parse expressions using Pratt precedence.
8. Implement primitive type table.
9. Type-check arithmetic and explicit casts.
10. Lower integer-returning `main` to LLVM IR.
11. Produce and link native executable.
12. Add differential scalar tests against C.
