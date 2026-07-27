# Compiler Architecture

## 1. Toolchain overview

```text
.kx source
  -> source manager
  -> lexer
  -> parser
  -> AST
  -> name resolution
  -> type/shape/frame/unit analysis
  -> KX high-level IR
  -> MLIR Linalg/Vector/SCF/MemRef lowering
  -> LLVM dialect / LLVM IR
  -> target optimization
  -> object file
  -> linker
  -> executable, library, firmware, Python module, MEX, or ROS package
```

## 2. Bootstrap language

Use C++23 for the first compiler because LLVM and MLIR expose their primary APIs there. This does not make KX a C++ derivative and does not constrain generated-code performance.

## 3. Front end

### Source manager

- canonical file identifiers;
- byte offsets and line tables;
- include/import tracking;
- UTF-8 validation;
- source excerpts for diagnostics.

### Lexer

- lossless token spans;
- numeric literal validation;
- nested-comment policy explicitly defined;
- invalid UTF-8 and escape diagnostics;
- fuzz target.

### Parser

Use recursive descent for declarations/statements and Pratt parsing for expressions. This gives direct control over diagnostics and precedence.

### AST

Separate syntax representation from semantic types. Avoid embedding LLVM objects in AST nodes.

### Semantic analysis

Passes:

1. declaration collection;
2. import/module resolution;
3. name lookup;
4. type construction;
5. expression typing;
6. constant evaluation;
7. shape/frame/unit validation;
8. lifetime/view checks;
9. unsafe-operation validation;
10. monomorphization planning where applicable.

## 4. Intermediate representation

### High-level KX dialect

Preserve:

- typed mathematical operations;
- shapes;
- frames and units where useful for diagnostics/optimization;
- ownership and alias facts;
- real-time attributes;
- source locations;
- target memory spaces.

### Lowering strategy

- scalar arithmetic -> MLIR Arith;
- structured control -> SCF;
- functions -> Func;
- memory views -> MemRef;
- matrices/tensors -> Linalg;
- explicit SIMD -> Vector;
- lower-level control -> CF;
- final conversion -> LLVM dialect and LLVM IR.

Do not lower matrices to raw loops too early. Premature lowering destroys structure needed for fusion, tiling, and target selection.

## 5. Optimization pipeline

Suggested stages:

1. canonicalization;
2. constant folding;
3. shape refinement;
4. view/alias simplification;
5. operation fusion;
6. layout planning;
7. backend selection;
8. tiling;
9. vectorization;
10. bufferization;
11. loop lowering;
12. LLVM optimization;
13. target instruction selection.

Each pass needs IR verification and regression tests.

## 6. Backend selection

A cost model should choose among:

- fully unrolled fixed-size kernel;
- generated tiled loop kernel;
- runtime library call;
- BLAS/LAPACK call;
- device kernel.

The selected path should be inspectable using compiler remarks.

## 7. Diagnostics

A KX diagnostic should include:

- concise primary message;
- exact source span;
- expected and actual type/shape/frame/unit;
- actionable correction when unambiguous;
- related declaration locations;
- no irrelevant template backtrace equivalent.

Example:

```text
error[KX2104]: cannot compose transforms
  --> arm.kx:18:22
   |
18 | let t = camera_to_base * world_to_tool;
   |         --------------   ^^^^^^^^^^^^^ expected transform<Base, _>
   |         |
   |         left operand ends in frame Base
help: reverse or provide the missing World-to-Camera transform
```

## 8. Incremental compilation

Not required initially. Design module boundaries and stable hashes so it can be added later. Correctness and deterministic full builds matter first.

## 9. Debug information

Emit standard DWARF/PDB information. Preserve source mapping through MLIR/LLVM lowering. Matrix and robotics types should have debugger-friendly layouts and pretty-printer support.

## 10. Cross-compilation

The compiler must separate:

- host platform running `kxc`;
- target triple;
- target CPU/features;
- system root;
- linker;
- runtime profile;
- ROS/MATLAB/Python integration availability.

## 11. Version strategy

Pin a supported LLVM major for each KX minor release. Do not follow LLVM `main` in stable releases. Upgrade through a dedicated compatibility branch with full code-generation and benchmark regression testing.
