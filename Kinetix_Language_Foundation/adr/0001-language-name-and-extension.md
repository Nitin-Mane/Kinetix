# ADR 0001: Language name and source extension

- Status: Accepted for foundation phase
- Decision: Use **Kinetix**, shorthand **KX**, and source extension `.kx`.

## Context

`.kt` is widely associated with Kotlin. `K++` implies a relationship or compatibility with C++ that the project does not provide and creates search/tooling problems.

## Consequences

- Compiler command: `kxc`.
- Package command: `kx`.
- Existing documents must not describe `.kt` as the primary extension.
