# Memory and Real-Time Model

## 1. Objective

KX should offer predictable systems behavior without forcing all code into raw-pointer programming.

## 2. Storage classes

- compile-time constants;
- static storage;
- stack values;
- explicitly allocated owned values;
- borrowed views;
- foreign memory;
- volatile/MMIO memory;
- device memory.

## 3. Ownership model

Recommended v1 compromise:

- values own their inline data;
- heap objects have a single explicit owner unless wrapped in an explicit shared type;
- `view<T>` and `mut_view<T>` are non-owning lexical borrows;
- stack-backed views cannot escape their valid lexical region;
- raw pointers require unsafe operations;
- no tracing garbage collector.

This is less ambitious than Rust’s full borrow system but safer than unrestricted C pointers.

## 4. Allocation visibility

Potentially allocating APIs must be identifiable through:

- return type such as `owned<T>`;
- allocator argument;
- function effect metadata;
- compiler remark/audit tools.

A matrix expression should not create a chain of hidden temporary heap objects.

## 5. Effects

Future function effects may include:

```text
allocates
blocks
locks
io
unsafe
foreign
realtime_safe
```

Do not overdesign a full effect system in v0.1. Begin with attributes and tooling, then formalize based on real use.

## 6. Real-time function example

```kx
@realtime
fn control_step(
    state: &RobotState,
    command: &mut MotorCommand,
    workspace: &mut ControlWorkspace
) -> ControlStatus {
    // no allocation, blocking, or unbounded recursion
}
```

The compiler can reject known forbidden operations. It cannot prove every OS or foreign-library behavior, so FFI functions need declared effects and trust boundaries.

## 7. Atomics

KX must define memory ordering explicitly:

- relaxed;
- acquire;
- release;
- acquire-release;
- sequentially consistent.

Defaults should be safe, but performance-sensitive code can choose weaker orderings explicitly.

## 8. Volatile and MMIO

Volatile prevents inappropriate elision/reordering of accesses according to language rules; it is not a synchronization primitive. Hardware register APIs must separate volatile access from atomic/thread synchronization.

## 9. Interrupts

Embedded profile should specify:

- interrupt function ABI;
- allowed operations;
- static allocation;
- priority and nesting assumptions;
- lock-free communication with main code;
- panic/fault behavior.

## 10. Failure behavior

No language exceptions. Profiles may choose:

- result return;
- controlled panic/abort;
- trap;
- user-defined fault handler;
- compile-time rejection.

Real-time and embedded profiles must avoid unwinding unless a specific platform implementation is proven and documented.
