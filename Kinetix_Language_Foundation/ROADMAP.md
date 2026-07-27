# Kinetix Complete Project Roadmap

## 1. Outcome

Build a compiled language and toolchain that can:

1. compile scalar, bit-level, matrix, vector, and robotics code to native machine code;
2. provide predictable memory and real-time behavior;
3. integrate through a stable C ABI;
4. generate Python extension modules with NumPy-compatible arrays;
5. generate MATLAB MEX functions and call MATLAB Engine APIs where licensed;
6. act as a ROS 2 client language through `rcl` and generated ROS interface types;
7. produce auditable LLVM IR, assembly, object files, shared libraries, firmware images, and ROS packages;
8. demonstrate measurable advantages on selected robotics and numerical workloads.

## 2. Reality-based schedule

A solo developer working part-time should expect **three to five years** to reach a credible 1.0 toolchain. A focused team of four to six experienced compiler, robotics, and tooling engineers could plausibly reach a usable 1.0 in **18 to 30 months**. Anything much faster would likely be a syntax demo rather than a dependable language.

## 3. Delivery principles

- Freeze semantics before expanding syntax.
- Build the smallest vertical slice through parser, type checker, IR, code generation, linker, and tests.
- Use the C ABI as the universal interoperability anchor.
- Use MLIR for structured mathematical lowering and LLVM for machine-code generation.
- Make allocations and synchronization visible.
- Require benchmarks against optimized libraries, not naive C++.
- Maintain deterministic compiler tests from the first commit.
- Reject features that cannot be explained with precise semantics.

---

## Phase 0 — Charter, constraints, and repository bootstrap

**Duration:** 2–4 weeks  
**Release:** `0.0.0-charter`

### Deliverables

- language charter and non-goals;
- source extension and naming decision;
- compiler licensing decision;
- repository structure;
- RFC and architecture-decision process;
- coding style, contribution guide, security policy;
- CI for Linux, Windows, and macOS builds;
- pinned LLVM/MLIR toolchain policy;
- benchmark ethics policy forbidding misleading claims.

### Exit criteria

- Every core contributor signs off on the mission and non-goals.
- The build produces a placeholder `kxc --version` on all supported host platforms.
- CI and formatting checks are mandatory.

---

## Phase 1 — Scalar language vertical slice

**Duration:** 8–12 weeks  
**Release:** `0.1.0-scalar`

### Language scope

- modules;
- fixed-width signed and unsigned integers;
- `f32` and `f64`;
- booleans;
- binary, octal, decimal, and hexadecimal literals;
- immutable `let`, mutable `var`, compile-time `const`;
- functions;
- `if`, `while`, `loop`, `break`, `continue`, `return`;
- arithmetic, comparison, logical, and bitwise operations;
- explicit casts;
- checked, wrapping, and saturating arithmetic;
- fixed-size arrays;
- basic structures.

### Compiler scope

- UTF-8 source reader;
- lexer with source spans;
- handwritten recursive-descent parser;
- AST;
- symbol table and name resolution;
- basic type checking;
- diagnostic engine with line/column excerpts;
- KIR or initial MLIR lowering;
- LLVM IR generation;
- native executable linking;
- `--emit=ast|kir|llvm-ir|asm|object`;
- unit, parser, diagnostic, and code-generation tests.

### Exit criteria

- At least 200 positive language tests and 200 expected-error tests.
- Deterministic output from repeated builds.
- Scalar benchmark output matches C reference implementations.
- Sanitizer-clean compiler test suite.

---

## Phase 2 — Memory, views, errors, and C ABI

**Duration:** 10–14 weeks  
**Release:** `0.2.0-systems`

### Language scope

- references and mutable references;
- lexical views;
- explicit raw pointers inside `unsafe`;
- `volatile` and atomic operations;
- stack allocation;
- optional allocator interfaces;
- `Result<T, E>`-style error values;
- tagged unions/enums;
- `defer` or deterministic cleanup blocks;
- ABI layout annotations;
- `extern "C"`, `@export`, and `@repr(C)`.

### Interoperability scope

- import C functions from headers through a constrained binding generator;
- export KX functions as C-callable shared-library symbols;
- generate C header files;
- verify structure size, alignment, and field offsets against C;
- establish ABI compatibility tests on x86-64 and AArch64.

### Exit criteria

- KX calls a C library and C calls a KX shared library.
- ABI conformance tests pass on Linux and Windows.
- Raw pointer use is impossible outside explicit unsafe regions.
- No hidden heap allocation in the core language.

---

## Phase 3 — Static vectors, matrices, tensors, and units

**Duration:** 12–18 weeks  
**Release:** `0.3.0-math`

### Type-system scope

- `vec<T, N>`;
- `mat<T, R, C>`;
- `tensor<T, D0, D1, ...>`;
- dynamic dimensions represented explicitly as `dyn`;
- slices/views with stride metadata;
- compile-time shape checking;
- row-major and column-major layouts;
- explicit host/device memory spaces;
- physical units as zero-runtime-cost type metadata;
- optional numerical contracts such as finite-only or normalized vectors.

### Operation scope

- elementwise operations and broadcasting rules;
- dot product, outer product, matrix multiplication;
- transpose, reshape, slicing, concatenation;
- reductions;
- decompositions through backend libraries;
- small fixed-size kernel specialization;
- BLAS/LAPACK fallback for large dense operations;
- sparse matrix representation design, initially experimental.

### Compiler scope

- KX Math dialect in MLIR or direct lowering to Linalg/Vector dialects;
- shape inference and shape diagnostics;
- alias analysis for views;
- loop tiling, fusion, unrolling, and vectorization;
- cost model choosing inline kernels versus external BLAS;
- target-feature dispatch for SIMD.

### Exit criteria

- Shape-invalid programs fail at compile time.
- Fixed 3×3, 4×4, and 6×6 kernels produce vectorized assembly where applicable.
- Large dense operations match a trusted BLAS result within documented numerical tolerances.
- Benchmark reports compare against Eigen and optimized BLAS, not handwritten naive loops.

---

## Phase 4 — Robotics semantic types

**Duration:** 12–18 weeks  
**Release:** `0.4.0-robotics`

### Core types

- coordinate-frame tags;
- `point`, `direction`, `rotation`, `quaternion`;
- `transform<From, To, T>`;
- `pose`, `twist`, `wrench`;
- Jacobians with compile-time dimensions;
- joint vectors and joint limits;
- trajectories with time domains;
- spatial algebra primitives;
- covariance and uncertainty wrappers.

### Safety goals

The following must be compile-time errors:

- adding positions expressed in incompatible frames without a transform;
- multiplying matrices with incompatible dimensions;
- adding metres to radians;
- applying a transform in the wrong direction;
- silently treating degrees as radians;
- assigning dynamic data to fixed storage without a checked conversion.

### Libraries

- `kx.math`;
- `kx.geometry`;
- `kx.kinematics`;
- `kx.trajectory`;
- `kx.control`;
- `kx.signal`;
- `kx.time`.

### Exit criteria

- Implement forward kinematics for a reference manipulator.
- Implement Jacobian computation and a damped least-squares inverse-kinematics example.
- Compare correctness and performance against Eigen plus a recognized robotics library.
- Demonstrate frame mismatch and unit mismatch diagnostics.

---

## Phase 5 — ROS 2 client library

**Duration:** 14–22 weeks  
**Release:** `0.5.0-ros2`

### Architecture

Implement `rclkx` over the ROS 2 C client layer `rcl`, not by reimplementing DDS and not by binding only to `rclcpp`. Generate KX message, service, and action types from ROS interface definitions.

### Features

- node lifecycle;
- publishers and subscriptions;
- services and clients;
- actions;
- parameters;
- timers;
- logging;
- clocks and time;
- QoS profiles;
- executors;
- callback groups;
- loaned-message path where supported;
- ROS package generation through `ament` integration;
- launch-file interoperability initially through existing Python/XML/YAML launch systems.

### Real-time direction

- static executor option;
- bounded queues;
- explicit callback priorities;
- allocation audit mode;
- tracing hooks;
- deterministic callback-order tests.

### Exit criteria

- KX talker/listener interoperates with C++ and Python ROS 2 nodes.
- Custom messages, services, and actions work across languages.
- QoS compatibility tests pass.
- At least one robot simulation package runs using KX nodes.
- CI targets the current ROS 2 LTS and Rolling, with version pins documented.

---

## Phase 6 — Python and NumPy integration

**Duration:** 8–14 weeks  
**Release:** `0.6.0-python`

### Capabilities

- generate CPython extension modules from exported KX functions;
- import Python modules through an explicit embedding boundary;
- convert scalar and structure values;
- zero-copy NumPy array views where layout, lifetime, alignment, and mutability allow;
- copied conversion when zero-copy cannot be proven safe;
- release the Python GIL around pure native kernels where legal;
- package wheels for supported platforms;
- stub generation for editor type checking.

### Command examples

```bash
kx py build package.kx
kx py wheel --python 3.14
```

### Exit criteria

- Python calls a KX matrix kernel.
- KX accepts a NumPy array without copying when constraints are satisfied.
- Lifetime violation tests fail safely.
- Generated wheels pass isolated-install tests.

---

## Phase 7 — MATLAB integration

**Duration:** 8–14 weeks  
**Release:** `0.7.0-matlab`

### Capabilities

- generate MEX entry points from selected KX functions;
- map MATLAB arrays to KX matrix/tensor views;
- create copied fallback conversions for unsupported layouts;
- call MATLAB Engine APIs through C/C++ wrappers when MATLAB is installed and licensed;
- produce MATLAB help text and function signatures;
- test on explicitly supported MATLAB releases.

### Constraints

MATLAB is proprietary. KX cannot redistribute MATLAB headers or libraries. CI may require a licensed private runner. “MATLAB integration” must therefore be an optional package, not a dependency of the core compiler.

### Exit criteria

- MATLAB calls a KX MEX matrix function.
- KX-generated MEX output matches MATLAB reference results.
- Build failure clearly explains missing MATLAB SDK/license requirements.

---

## Phase 8 — Deterministic real-time and embedded targets

**Duration:** 16–24 weeks  
**Release:** `0.8.0-realtime`

### Features

- no-heap and bounded-heap profiles;
- freestanding compilation;
- startup code and linker-script integration;
- interrupt-safe APIs;
- memory-mapped register declarations;
- atomics and memory-order semantics;
- WCET-friendly subset documentation;
- allocation and lock analysis;
- ARM Cortex-M and RISC-V initial targets;
- firmware outputs: ELF, raw binary, Intel HEX where applicable;
- hardware-in-the-loop test harness.

### Exit criteria

- Blink, timer interrupt, UART, and motor-control examples on reference boards.
- No hidden dynamic allocation in freestanding profile.
- Reproducible firmware images.
- Fault-injection and peripheral-register tests.

---

## Phase 9 — Heterogeneous acceleration

**Duration:** 16–28 weeks  
**Release:** `0.9.0-accelerated`

### Direction

- multithreaded CPU backend;
- SIMD auto-vectorization and explicit vector types;
- GPU kernel subset;
- CUDA or HIP integration through external ABI first;
- MLIR GPU/SCF/Linalg/Vector lowering experimentation;
- explicit memory transfer and synchronization;
- target-specialized matrix kernels;
- optional OpenCL/SYCL interoperability rather than immediate native reimplementation.

### Exit criteria

- End-to-end benchmark includes transfer time.
- CPU and GPU correctness agree within tolerance.
- Kernel dispatch decisions are inspectable.
- No benchmark excludes compilation, setup, or transfer cost without explicitly saying so.

---

## Phase 10 — Tooling, IDE, package ecosystem, and self-hosting

**Duration:** 16–30 weeks, overlapping earlier phases  
**Release:** `1.0.0`

### Tooling

- formatter `kxfmt`;
- language server `kxls`;
- debugger integration through DWARF/PDB;
- package manager and lockfile;
- documentation generator;
- benchmark harness;
- profiler/tracing integration;
- cross-compilation presets;
- stable compiler plugin boundary only if a real use case exists.

### Self-hosting

Self-hosting is not required for 1.0. A compiler written in its own language is symbolic, not automatically better. Begin self-hosting only after KX supports the features needed to implement the front end without weakening maintainability.

### 1.0 acceptance criteria

- language specification versioned and frozen for the 1.x series;
- current ROS 2 LTS support;
- C, Python/NumPy, and optional MATLAB interoperability;
- x86-64 and AArch64 production targets;
- documented real-time subset;
- reproducible benchmark suite;
- package signing and supply-chain policy;
- at least three nontrivial reference projects;
- no known critical correctness defects.

---

## 4. Reference projects required before 1.0

1. **Six-axis manipulator control stack**  
   Forward/inverse kinematics, Jacobian, trajectory generation, ROS 2 topics/actions, Python visualization.

2. **Mobile robot localization component**  
   Matrix-heavy estimation, fixed-size covariance operations, ROS 2 messages, deterministic callback execution.

3. **Embedded motor-control firmware**  
   Memory-mapped I/O, fixed-point or floating-point control loop, bounded memory, hardware-in-the-loop tests.

4. **MATLAB validation bridge**  
   Compare KX robotics kernels with MATLAB reference scripts through generated MEX functions.

## 5. Staffing model

Minimum serious team:

- compiler/front-end engineer;
- LLVM/MLIR/code-generation engineer;
- robotics and numerical-computing engineer;
- runtime/FFI/ROS engineer;
- tooling and developer-experience engineer;
- test/release engineer, potentially shared.

A solo project must reduce scope. The correct solo target is a research-grade compiler supporting fixed-size matrices, frame-safe transforms, C ABI, and basic ROS 2 pub/sub—not a complete replacement for C++, MATLAB, and Python.
