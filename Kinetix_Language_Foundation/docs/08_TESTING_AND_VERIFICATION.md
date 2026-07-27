# Testing and Verification Strategy

## 1. Test pyramid

### Unit tests

- lexer;
- parser utilities;
- type rules;
- layout calculations;
- lowering helpers;
- runtime primitives.

### Golden tests

- AST output;
- diagnostics;
- KX IR;
- MLIR/LLVM IR;
- selected assembly patterns.

Avoid overspecifying irrelevant IR formatting.

### Execution tests

Compile and run KX programs, compare exit code and output.

### Negative tests

Every invalid semantic rule needs an expected diagnostic test.

### Differential tests

Compare KX operations against C++, NumPy, MATLAB where available, and high-precision references.

### Fuzzing

- lexer/parser;
- type deserializer;
- package manifest;
- C header importer;
- ROS interface generator;
- Python buffer conversion metadata.

### Hardware tests

- x86-64;
- AArch64;
- selected ARM Cortex-M;
- selected RISC-V;
- robotics hardware-in-the-loop.

## 2. Numerical correctness

Each algorithm defines:

- reference implementation;
- input domain;
- absolute/relative tolerance;
- exceptional values;
- conditioning limits;
- deterministic/non-deterministic behavior.

## 3. ABI tests

Generate corresponding C and KX types, then compare:

- `sizeof`;
- alignment;
- field offsets;
- enum representation;
- call/return values;
- symbol visibility;
- callback invocation.

## 4. ROS tests

Use cross-language nodes and middleware matrices. Test QoS mismatch intentionally.

## 5. Sanitizers and analysis

- AddressSanitizer;
- UndefinedBehaviorSanitizer;
- ThreadSanitizer where feasible;
- memory leak checks;
- clang-tidy;
- static analysis;
- dependency vulnerability scanning.

## 6. Reproducibility

Record:

- KX compiler revision;
- LLVM version;
- host and target triples;
- dependencies and lockfiles;
- build flags;
- environment variables affecting code generation;
- CPU features.

## 7. Release gates

A release cannot ship when:

- compiler crashes on valid inputs;
- known miscompilation is unresolved;
- ABI tests fail;
- security-critical regression exists;
- benchmark claims cannot be reproduced;
- language behavior changed without migration notes.
