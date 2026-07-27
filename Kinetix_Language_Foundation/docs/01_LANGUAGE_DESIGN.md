# Kinetix Language Design Draft

## 1. Design character

KX should be compact, statically typed, compiled ahead of time, expression-oriented where useful, and explicit around expensive or dangerous behavior.

The language should feel easier than C++ because it removes historical complexity—not because it hides machine costs.

## 2. Example

```kx
module robot.arm;

use kx.geometry::{Frame, Transform, Point};
use kx.units::{m, rad, s};

frame Base;
frame Tool;

fn command(
    target: Point<f64, Base, m>,
    base_to_tool: Transform<f64, Base, Tool>
) -> Result<void, MotionError> {
    let tool_target = base_to_tool.apply(target);
    controller.move_to(tool_target)?;
    return ok();
}
```

## 3. Lexical basics

- UTF-8 source;
- ASCII keywords for v1;
- `//` line comments and `/* ... */` block comments;
- semicolons terminate simple statements;
- braces delimit blocks;
- identifiers are case-sensitive;
- decimal digit separators use `_`.

## 4. Literal forms

```kx
let b: u8 = 0b1010_1100;
let o: u16 = 0o755;
let d: u32 = 1_000_000;
let h: u64 = 0xDEAD_BEEF;
let x: f64 = 3.141592653589793;
```

The bases affect source representation only.

## 5. Primitive types

- `i8`, `i16`, `i32`, `i64`, `i128`;
- `u8`, `u16`, `u32`, `u64`, `u128`;
- `f16`, `f32`, `f64`;
- `bool`, `byte`, `char`;
- `usize`, `isize`;
- `void`, `never`.

Default integer and floating types should be resolved contextually or require explicit policy. v0.1 should prefer explicit numeric types in ABI and performance-critical code.

## 6. Declarations

```kx
const MAX_JOINTS: usize = 12;
let serial: u32 = 42;
var ticks: u64 = 0;
```

- `const`: compile-time value;
- `let`: immutable runtime binding;
- `var`: mutable runtime binding.

## 7. Functions

```kx
fn clamp(value: f32, low: f32, high: f32) -> f32 {
    if value < low { return low; }
    if value > high { return high; }
    return value;
}
```

No implicit exceptions. Recoverable errors use result values.

## 8. Errors

```kx
enum ParseError {
    InvalidToken,
    OutOfRange,
}

fn parse_port(text: str_view) -> Result<u16, ParseError> {
    // ...
}
```

The `?` operator may propagate an error only when the enclosing function returns a compatible result type.

## 9. Structures and enums

```kx
struct MotorState {
    position: f64,
    velocity: f64,
    torque: f64,
}

enum Mode: u8 {
    Idle = 0,
    Position = 1,
    Velocity = 2,
}
```

ABI-facing structures require `@repr(C)`.

## 10. Arrays and views

```kx
let gains: [f32; 3] = [1.0, 0.1, 0.01];
fn sum(values: view<f32>) -> f32 { ... }
fn normalize(values: mut_view<f32>) { ... }
```

Bounds checking is enabled by default. Unchecked indexing requires `unsafe`.

## 11. Memory categories

- value types;
- immutable and mutable references;
- lexical views;
- owned heap values from explicit allocators;
- raw pointers under `unsafe`;
- volatile pointers/registers;
- device memory handles.

KX v1 should not attempt a fully general Rust-style borrow checker. It should implement a tractable lexical view/lifetime model and prohibit obvious dangling stack views. More advanced analysis can be added incrementally.

## 12. Unsafe operations

```kx
unsafe {
    *register = value;
}
```

Unsafe operations include:

- raw pointer dereference;
- unchecked indexing;
- inline assembly;
- foreign mutable buffer adoption;
- constructing a view from an arbitrary address;
- reinterpretation that can violate alignment or type invariants.

## 13. Arithmetic

Normal integer arithmetic has defined behavior. The exact default should be finalized by RFC, but the recommended rule is:

- debug: checked with trap/diagnostic;
- release: still defined, never compiler-level undefined behavior;
- explicit `wrapping_*`, `saturating_*`, and `checked_*` operations available.

Floating-point fast math is opt-in.

## 14. Generics

Generics are required eventually for containers and math, but v0.1 should avoid unrestricted templates. Start with constrained parametric types and compile-time dimensions:

```kx
fn dot<T: Real, const N: usize>(a: vec<T, N>, b: vec<T, N>) -> T;
```

No C++-style template metaprogramming.

## 15. Operator policy

Built-in mathematical types may define a limited, specification-controlled operator set. User-defined arbitrary operator overloading should be deferred because it harms readability and complicates diagnostics.

## 16. Modules and packages

```kx
module kx.kinematics.jacobian;
use kx.math::{mat, vec};
```

Packages are declared in `kinetix.toml`; exact lockfile and registry semantics are deferred.

## 17. Concurrency

The core language needs threads, atomics, and tasks only after memory semantics are stable. Robotics real-time APIs should favor explicit executors and bounded channels over implicit async runtimes.

## 18. Compatibility

Stable 1.x releases follow semantic versioning for:

- source language;
- standard-library stable surface;
- KX C ABI;
- package manifest;
- generated ROS type conventions.

Compiler-internal IR is not a stable public ABI unless explicitly versioned.
