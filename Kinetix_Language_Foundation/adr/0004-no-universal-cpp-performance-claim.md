# ADR 0004: No universal C++ performance claim

- Status: Accepted
- Decision: KX will not claim to outperform C++ universally.

## Rationale

Equivalent C++ can use the same algorithms, target instructions, LLVM backend, and external libraries. Universal superiority is not technically credible.

## Allowed claims

Claims tied to published workloads, hardware, flags, correctness checks, and optimized baselines.

## Project target

Comparable native performance plus measurable benefits from shape/frame/unit safety, domain specialization, lower integration friction, or deterministic real-time behavior.
