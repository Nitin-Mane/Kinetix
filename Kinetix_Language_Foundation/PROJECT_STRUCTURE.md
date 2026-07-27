# Recommended Repository Structure

```text
kinetix/
├── CMakeLists.txt
├── CMakePresets.json
├── README.md
├── ROADMAP.md
├── agent.md
├── skills.md
├── CONTRIBUTING.md
├── SECURITY.md
├── GOVERNANCE.md
├── CHANGELOG.md
├── LICENSE
├── cmake/
├── compiler/
│   ├── include/kx/
│   │   ├── Basic/
│   │   ├── Lex/
│   │   ├── Parse/
│   │   ├── AST/
│   │   ├── Sema/
│   │   ├── IR/
│   │   ├── Lowering/
│   │   ├── CodeGen/
│   │   └── Diagnostics/
│   ├── lib/
│   └── tools/kxc/
├── mlir/
│   ├── include/kx/Dialect/
│   ├── lib/Dialect/
│   ├── include/kx/Transforms/
│   └── lib/Transforms/
├── runtime/
│   ├── core/
│   ├── math/
│   ├── platform/
│   └── embedded/
├── stdlib/
│   ├── core/
│   ├── math/
│   ├── geometry/
│   ├── kinematics/
│   ├── control/
│   ├── signal/
│   └── time/
├── interop/
│   ├── c/
│   ├── python/
│   ├── matlab/
│   └── ros2/
├── tools/
│   ├── kx/
│   ├── kxfmt/
│   ├── kxls/
│   ├── kxdoc/
│   └── bindgen/
├── grammar/
│   └── Kinetix.ebnf
├── docs/
├── adr/
├── rfcs/
├── examples/
├── benchmarks/
│   ├── scalar/
│   ├── matrix/
│   ├── kinematics/
│   ├── ros2/
│   └── realtime/
├── tests/
│   ├── Lexer/
│   ├── Parser/
│   ├── Sema/
│   ├── Diagnostics/
│   ├── IR/
│   ├── CodeGen/
│   ├── Execution/
│   ├── ABI/
│   ├── Interop/
│   └── Fuzz/
└── .github/
    ├── workflows/
    └── ISSUE_TEMPLATE/
```

## Dependency boundaries

- `Basic` cannot depend on parser, AST, semantic analysis, or code generation.
- `Lex` may depend on `Basic` and diagnostics.
- `Parse` may depend on lexer, AST, and diagnostics.
- `Sema` may depend on AST, type system, and diagnostics.
- Lowering cannot repair invalid programs; semantic analysis must reject them first.
- Runtime libraries cannot depend on compiler internals.
- Standard library APIs must be implementable without exposing LLVM types.
- Interop packages must remain optional.

## Build profiles

- `dev`: assertions, IR verification, fast incremental build;
- `asan`: AddressSanitizer and UndefinedBehaviorSanitizer;
- `release`: optimized compiler build;
- `coverage`: test coverage;
- `embedded`: cross-compilation support;
- `docs`: documentation generation.
