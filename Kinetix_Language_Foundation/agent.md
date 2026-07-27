# Kinetix Coding-Agent Instructions

This file is authoritative for AI coding agents working in the Kinetix repository.

## 1. Mission

Build a correct, measurable, interoperable systems-and-robotics language. Do not optimize for impressive demos at the cost of undefined semantics, weak diagnostics, or untested compiler behavior.

## 2. Non-negotiable truths

- Never claim KX is faster than C++ without a reproducible benchmark against an optimized C++ baseline.
- Binary, octal, decimal, and hexadecimal are literal representations, not separate runtime numeric types.
- The source extension is `.kx`, not `.kt`.
- The official language name is Kinetix or KX, not K++.
- C ABI interoperability is the foundation. Python, MATLAB, and ROS integration build on it.
- No hidden heap allocation in core language operations.
- No undefined signed overflow.
- Raw pointer dereference, unchecked indexing, inline assembly, and foreign mutable memory require an explicit `unsafe` boundary.
- Shape, frame, unit, and layout errors should be compile-time errors wherever possible.

## 3. Work-order hierarchy

When instructions conflict, use this order:

1. accepted language specification;
2. accepted RFCs and architecture decisions;
3. tests describing stable behavior;
4. roadmap milestone scope;
5. issue description;
6. implementation convenience.

Do not silently change language behavior to make a test pass. Identify whether the test or implementation contradicts the specification.

## 4. Required workflow for every change

1. Read the relevant specification, ADR, and tests.
2. State the semantic behavior being added or corrected.
3. Add or update negative tests before implementation for diagnostics-related work.
4. Implement the smallest complete vertical change.
5. Run formatting, unit tests, integration tests, and sanitizer builds.
6. Inspect generated IR or assembly when code generation changes.
7. Update user documentation and changelog when behavior changes.
8. Report limitations directly.

## 5. Scope control

Do not add these before their roadmap phase:

- classes or inheritance;
- exceptions;
- unrestricted macros;
- operator overloading;
- garbage collection;
- dynamic reflection;
- custom package registry;
- direct C++ ABI binding;
- GPU-native syntax;
- self-hosting work.

A requested feature outside the current phase must be written as an RFC or deferred issue.

## 6. Compiler invariants

- Every AST and IR node with source origin must retain a source span.
- Diagnostics must identify the primary error and avoid cascades when possible.
- Type checking must finish before lowering to executable IR.
- IR verification must run after every lowering stage in debug/CI builds.
- Generated code must not depend on uninitialized values.
- Structure layout must be deterministic for a target triple.
- ABI-facing structures require explicit representation attributes.
- Optimization must preserve observable volatile, atomic, and FFI behavior.
- Release optimizations must never introduce undefined behavior as an optimization assumption.

## 7. Mathematics invariants

- Matrix multiplication requires compatible dimensions.
- Broadcasting rules must be documented and deterministic.
- Views carry shape, stride, layout, mutability, and lifetime information.
- A zero-copy conversion is allowed only when lifetime, alignment, layout, element type, and mutability are valid.
- Numerical algorithms must document tolerance, conditioning limitations, and backend dependencies.
- Fast-math transformations require an explicit user or build-profile opt-in.

## 8. Robotics invariants

- Coordinate frames are directional types, not comments.
- Transform composition order must be unambiguous.
- Angular units must never be guessed.
- Timestamp and clock domains must be explicit.
- Real-time APIs must state whether they allocate, block, lock, or perform system calls.
- ROS callbacks must not silently execute on arbitrary threads when a deterministic executor is selected.

## 9. Interoperability rules

### C

- Prefer plain functions and `@repr(C)` structures.
- Verify sizes, alignments, field offsets, calling conventions, and symbol names.
- Generate a C header for exported KX APIs.

### Python

- Use the supported CPython extension API.
- Prefer Python’s stable ABI only where it does not block required performance or NumPy integration.
- Manage reference counts and GIL state explicitly.
- Never expose a KX buffer after its owner has been destroyed.

### MATLAB

- Treat MATLAB support as optional and licensed.
- Do not commit or redistribute proprietary MATLAB SDK files.
- Prefer generated MEX wrappers for calling KX from MATLAB.
- Maintain release-compatibility tests on licensed runners.

### ROS 2

- Build `rclkx` on `rcl` and generated type support.
- Do not implement a new DDS/RMW layer merely to create a client language.
- Test against C++ and Python nodes, not only KX-to-KX communication.

## 10. Performance rules

Every performance pull request must include:

- benchmark source;
- hardware and OS details;
- compiler versions and flags;
- warmup policy;
- sample count;
- median and dispersion;
- correctness check;
- optimized baseline;
- generated IR/assembly evidence when relevant;
- memory-allocation and transfer accounting.

Reject benchmarks that compare KX optimized code against unoptimized C++, omit library initialization selectively, or report only the best run.

## 11. Testing commands

The exact commands may evolve, but the repository should converge on:

```bash
cmake --preset dev
cmake --build --preset dev
ctest --preset dev
cmake --build --preset dev --target check-kx
cmake --build --preset dev --target check-format
cmake --build --preset asan
ctest --preset asan
```

For a code-generation change, also run representative commands:

```bash
build/bin/kxc test.kx --emit=llvm-ir
build/bin/kxc test.kx --emit=asm -O3
```

## 12. Commit and pull-request expectations

- One semantic concern per commit where practical.
- No generated binaries in source control.
- No drive-by refactors mixed with behavior changes.
- New syntax requires grammar, parser, formatter, diagnostics, tests, and documentation.
- New target support requires CI or a documented external validation process.
- Mark experimental features explicitly and keep them disabled by default.

## 13. Agent response format

When completing work, report:

1. behavior changed;
2. files changed;
3. tests run and results;
4. generated-code impact;
5. known limitations;
6. follow-up issue if needed.

Do not write motivational filler. Be precise about what is complete and what remains incomplete.
