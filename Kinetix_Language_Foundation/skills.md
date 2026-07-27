# Kinetix Project Skills Matrix

This file defines the technical capabilities needed to build Kinetix and can also be used to assign work to human or AI contributors.

## Skill 1 — Programming-language semantics

### Required knowledge

- lexical structure and grammars;
- static type systems;
- name resolution and scopes;
- value categories and mutability;
- explicit conversions;
- overflow and numerical semantics;
- module systems;
- error propagation;
- unsafe boundaries;
- language-version compatibility.

### Expected outputs

- specification sections;
- accepted/rejected examples;
- diagnostics requirements;
- semantic test suites;
- RFC review.

### Completion test

A contributor can explain exactly what a valid program means without relying on implementation accidents.

---

## Skill 2 — Compiler front end

### Required knowledge

- UTF-8 lexing;
- recursive-descent or Pratt parsing;
- AST design;
- source spans;
- error recovery;
- symbol tables;
- type checking;
- compile-time evaluation;
- deterministic diagnostics.

### Expected outputs

- lexer and parser;
- AST printer;
- semantic analyzer;
- expected-error tests;
- language server parsing support.

---

## Skill 3 — LLVM and MLIR

### Required knowledge

- LLVM IR and target triples;
- data layout;
- optimization pipelines;
- debug information;
- object generation and linking;
- MLIR operations, types, dialects, regions, and passes;
- Linalg, Vector, Arith, Func, MemRef, SCF, CF, LLVM, and Transform dialects;
- dialect conversion and legality;
- IR verification.

### Expected outputs

- KX/KIR dialect definitions;
- lowering passes;
- vectorized math code;
- target-specific code generation;
- IR and assembly regression tests.

---

## Skill 4 — Numerical computing

### Required knowledge

- dense and sparse linear algebra;
- numerical conditioning;
- floating-point behavior;
- BLAS/LAPACK interfaces;
- SIMD and cache-aware kernels;
- matrix layouts and strides;
- broadcasting and reductions;
- automatic differentiation as a later research topic.

### Expected outputs

- matrix semantics;
- backend-selection policy;
- correctness tolerances;
- reference implementation tests;
- benchmark kernels.

---

## Skill 5 — Robotics mathematics

### Required knowledge

- rigid-body transformations;
- SO(2), SO(3), SE(2), and SE(3);
- quaternions and rotation representations;
- spatial vectors;
- forward and inverse kinematics;
- Jacobians;
- trajectory generation;
- dynamics and control;
- uncertainty and covariance;
- coordinate-frame conventions.

### Expected outputs

- frame-safe types;
- transform-composition rules;
- kinematics libraries;
- reference robot examples;
- comparison against established robotics libraries.

---

## Skill 6 — Memory, concurrency, and real-time systems

### Required knowledge

- stack, heap, arenas, pools, and static storage;
- aliasing and lifetimes;
- atomics and memory ordering;
- volatile memory and MMIO;
- interrupts;
- lock-free and bounded queues;
- priority inversion;
- real-time operating-system constraints;
- deterministic scheduling;
- allocation and blocking audits.

### Expected outputs

- memory model;
- unsafe operations specification;
- executor/runtime design;
- embedded profile;
- latency and jitter tests.

---

## Skill 7 — C and native ABI interoperability

### Required knowledge

- platform calling conventions;
- object formats;
- symbol visibility;
- structure layout and alignment;
- C header generation;
- dynamic libraries;
- CMake and linkers;
- cross-platform ABI testing.

### Expected outputs

- `extern "C"` implementation;
- binding generator;
- ABI conformance tests;
- C export/import examples.

---

## Skill 8 — Python and NumPy interoperability

### Required knowledge

- CPython extension and embedding APIs;
- reference counting;
- GIL behavior;
- buffer protocol;
- NumPy arrays, dtype, shape, strides, ownership, and C API;
- wheel packaging;
- Python stub generation.

### Expected outputs

- generated Python modules;
- zero-copy array bridges;
- safe lifetime handling;
- packaging and compatibility tests.

---

## Skill 9 — MATLAB interoperability

### Required knowledge

- MEX functions;
- MATLAB Data API;
- MATLAB Engine API for C/C++;
- array layout and type conversion;
- licensed CI constraints;
- release compatibility.

### Expected outputs

- MEX wrapper generator;
- MATLAB array bridge;
- MATLAB validation examples;
- optional SDK discovery in build tooling.

---

## Skill 10 — ROS 2 integration

### Required knowledge

- ROS 2 nodes, topics, services, actions, parameters, and lifecycle;
- `rcl`, `rmw`, rosidl, and type support;
- executors and callback groups;
- DDS QoS concepts;
- ament/colcon packaging;
- tracing and real-time concerns;
- cross-language interoperability.

### Expected outputs

- `rclkx` client library;
- message/service/action generator;
- KX package support in ROS workspaces;
- C++/Python/KX integration tests.

---

## Skill 11 — Tooling and developer experience

### Required knowledge

- CLI design;
- formatter architecture;
- Language Server Protocol;
- debugger metadata;
- package managers and lockfiles;
- reproducible builds;
- cross-compilation;
- diagnostics UX.

### Expected outputs

- `kx`, `kxc`, `kxfmt`, and `kxls`;
- editor support;
- package manifest and lockfile;
- documentation generator;
- installer/release packages.

---

## Skill 12 — Verification, testing, and benchmarking

### Required knowledge

- unit, integration, snapshot, and differential tests;
- compiler fuzzing;
- property-based testing;
- sanitizers;
- reproducibility;
- statistical benchmarking;
- numerical tolerance design;
- hardware-in-the-loop testing.

### Expected outputs

- layered test strategy;
- benchmark harness;
- fuzz targets;
- release qualification report;
- regression dashboards.

---

## Suggested contributor tracks

### Compiler track

Skills 1, 2, 3, 7, and 12.

### Robotics and math track

Skills 4, 5, 6, 10, and 12.

### Interoperability track

Skills 7, 8, 9, 10, and 11.

### Embedded and real-time track

Skills 3, 6, 7, 10, and 12.

## Minimum skills for the first prototype

A credible first prototype needs only:

1. language semantics;
2. compiler front end;
3. LLVM/MLIR lowering;
4. fixed-size numerical computing;
5. C ABI;
6. testing.

Do not attempt ROS, Python, MATLAB, GPU, embedded, and package management simultaneously.
