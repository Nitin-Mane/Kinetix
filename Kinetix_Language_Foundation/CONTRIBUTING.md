# Contributing to Kinetix

## Before coding

1. Read `README.md`, `ROADMAP.md`, and `agent.md`.
2. Confirm that the work belongs to the current milestone.
3. For syntax, semantics, ABI, memory-model, or interoperability changes, open an RFC first.
4. For local implementation details, an issue and architecture-decision record may be enough.

## Change requirements

- Syntax: grammar, parser, formatter, diagnostics, positive tests, negative tests, and documentation.
- Type-system change: formal rule, rejected examples, inference tests, and compatibility note.
- Code-generation change: IR checks, execution tests, and target coverage.
- Runtime change: allocation/blocking behavior and thread-safety documentation.
- Performance change: benchmark evidence following `docs/13_BENCHMARK_PLAN.md`.
- FFI change: ABI tests on each supported platform.

## Style

- C++ implementation code follows the repository clang-format and clang-tidy configuration.
- Prefer explicit ownership and narrow interfaces.
- Avoid global mutable state.
- Keep compiler diagnostics actionable.
- Do not use abbreviations that obscure language semantics.

## Pull requests

Describe:

- problem;
- semantic behavior;
- implementation approach;
- tests;
- compatibility impact;
- performance impact;
- limitations.

A pull request that adds a feature without tests or documentation is incomplete.
