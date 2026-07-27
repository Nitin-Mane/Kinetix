# ADR 0003: LLVM and MLIR backend

- Status: Accepted for foundation phase
- Decision: Use MLIR for structured mathematical/robotics lowering and LLVM for native code generation.

## Rationale

KX needs to preserve matrix/vector structure long enough for domain-specific optimization while relying on mature target backends.

## Consequences

- Production compiler bootstrap uses C++23.
- LLVM major versions are pinned per KX release.
- A compatibility layer isolates upstream API churn.
- High-level math must not lower to raw loops prematurely.
