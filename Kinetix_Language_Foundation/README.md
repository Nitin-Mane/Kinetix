# Kinetix (KX) Language Foundation

Kinetix is a proposed compiled systems-and-robotics programming language designed around predictable performance, explicit memory, static mathematical structure, robotics semantics, and practical interoperability.

This repository is a **design and execution package**, not a claim that a working language already exists. It defines what should be built, why it should be built, what must be measured, and what should be rejected.

## Hard truth

KX cannot honestly promise to outperform C++ in every program. C++ can express the same machine instructions, use the same LLVM optimizations, call the same BLAS/CUDA libraries, and receive equally expert hand-tuning. A universal performance claim would be technically indefensible.

KX can aim to outperform ordinary C++ implementations in selected robotics and numerical workloads by giving the compiler information that typical C++ code often hides:

- matrix and tensor shapes;
- coordinate frames;
- physical units;
- sparsity and symmetry;
- aliasing and ownership;
- real-time constraints;
- target hardware and memory spaces;
- acceptable numerical error;
- deterministic execution requirements.

The competitive target is therefore:

> C-like control and interoperability, MATLAB-like mathematical expression, Python-like usability, and compiler-visible robotics semantics—without a garbage collector or hidden allocation.

## Project identity

| Item | Decision |
|---|---|
| Official name | Kinetix |
| Language shorthand | KX |
| Source extension | `.kx` |
| Compiler | `kxc` |
| Package/build tool | `kx` |
| Intermediate representation | KIR / KX MLIR dialect |
| ROS 2 client library | `rclkx` |
| Python module generator | `kx py build` |
| MATLAB MEX generator | `kx mex build` |

Do not use `.kt`; Kotlin already owns that extension in practice. Do not use “K++” as the official searchable name. It falsely implies C++ compatibility and is poor for tooling, package registries, shells, and search engines.

## Documents

- [Project roadmap](ROADMAP.md)
- [AI agent operating instructions](agent.md)
- [Agent discovery file](AGENTS.md) and [Claude project instructions](CLAUDE.md)
- [Required project skills](skills.md)
- [Vision and non-goals](docs/00_VISION_AND_NON_GOALS.md)
- [Language design](docs/01_LANGUAGE_DESIGN.md)
- [Mathematics and robotics model](docs/02_MATH_AND_ROBOTICS_MODEL.md)
- [Compiler architecture](docs/03_COMPILER_ARCHITECTURE.md)
- [Interop design](docs/04_INTEROPERABILITY.md)
- [ROS 2 design](docs/05_ROS2_INTEGRATION.md)
- [Performance plan](docs/06_PERFORMANCE_STRATEGY.md)
- [Memory and real-time model](docs/07_MEMORY_AND_REALTIME.md)
- [Testing and verification](docs/08_TESTING_AND_VERIFICATION.md)
- [Toolchain and developer experience](docs/09_TOOLCHAIN_AND_DX.md)
- [Standard library plan](docs/10_STANDARD_LIBRARY.md)
- [Release and governance](docs/11_RELEASE_GOVERNANCE.md)
- [Risk register](docs/12_RISK_REGISTER.md)
- [Formal benchmark plan](docs/13_BENCHMARK_PLAN.md)
- [Project structure](PROJECT_STRUCTURE.md)
- [Initial EBNF grammar](grammar/Kinetix.ebnf)
- [Architecture decisions](adr/)
- [RFC template](rfcs/0000-template.md)

## Recommended bootstrap stack

The first production compiler should be implemented in **C++23** using LLVM/MLIR, CMake, Ninja, and a handwritten recursive-descent parser. That is not ideological; it is the shortest reliable path to first-class LLVM/MLIR access and diagnostics. The implementation language does not determine the speed of programs produced by the compiler.

A disposable Python prototype may be used to test syntax, but it must not become the production compiler by accident.

## Status

Current phase: **language charter and architecture definition**.

No performance claim is valid until the benchmark protocol in `docs/13_BENCHMARK_PLAN.md` is implemented and results are independently reproducible.
