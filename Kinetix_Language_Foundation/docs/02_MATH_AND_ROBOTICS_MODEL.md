# Mathematical and Robotics Type Model

## 1. Why this is the real differentiator

Alternative number-base notation will not make KX valuable. Compiler-visible mathematics and robotics semantics might.

## 2. Static vector and matrix types

```kx
let p: vec<f64, 3> = [1.0, 2.0, 3.0];
let j: mat<f64, 6, 7>;
let h: mat<f64, 4, 4>;
```

Compile-time dimensions allow:

- immediate shape errors;
- stack storage for small objects;
- unrolled kernels;
- SIMD planning;
- better register allocation;
- specialized inverse/determinant paths;
- ABI-stable layouts.

## 3. Dynamic matrices

```kx
let image: matrix<f32, dyn, dyn> = matrix.alloc(rows, cols, allocator);
```

Dynamic matrices must expose ownership, layout, dimensions, and allocator. A matrix operation must not allocate a result invisibly unless the API explicitly returns an owned value and the allocation policy is visible.

## 4. Layout and memory spaces

```kx
mat<f32, 4, 4, row_major>
mat<f32, 4, 4, column_major>
tensor<f16, 1, 3, 224, 224, device<Cuda0>>
```

Layout conversion and host/device transfer are explicit.

## 5. Views

```kx
let block: mat_view<f64, 3, 3> = matrix.block<0, 0, 3, 3>();
```

A view contains or proves:

- element type;
- shape;
- strides;
- alignment;
- mutability;
- lifetime;
- memory space.

## 6. Broadcasting

KX should support broadcasting only with deterministic, documented rules. For static shapes, the compiler resolves compatibility. For dynamic shapes, runtime checks are explicit or inserted with clear failure behavior.

## 7. Coordinate frames

```kx
frame World;
frame Base;
frame Tool;

let world_to_base: transform<f64, World, Base>;
let base_to_tool: transform<f64, Base, Tool>;
let world_to_tool = world_to_base * base_to_tool;
```

The reverse order must fail if frames do not align.

## 8. Geometric categories

A point is not the same as a direction. A normal vector may transform differently from a direction under non-rigid transforms. KX should distinguish:

- `point<T, Frame, Unit>`;
- `direction<T, Frame>`;
- `normal<T, Frame>`;
- `rotation<T, From, To>`;
- `transform<T, From, To>`;
- `pose<T, Frame>`;
- `twist<T, Frame>`;
- `wrench<T, Frame>`.

## 9. Units

```kx
let length: quantity<f64, m> = 1.5m;
let angle: quantity<f64, rad> = 0.5rad;
let speed: quantity<f64, m / s> = 2.0m/s;
```

Units must be erased or normalized at compile time where possible. Unit safety cannot impose per-operation runtime object overhead.

## 10. Rotations

Supported representations:

- 2D angle;
- 3×3 rotation matrix;
- unit quaternion;
- angle-axis;
- Lie algebra tangent vector.

Conversions should be explicit and state numerical edge cases. A quaternion used as a rotation must satisfy normalization policy: checked, normalized on construction, or explicitly unchecked.

## 11. Kinematics

Proposed types:

```kx
robot_model<JointCount>
joint_state<T, JointCount>
jacobian<T, TaskDim, JointCount>
trajectory<T, JointCount, Clock>
```

Compiler benefits include dimension checking and specialization for fixed robot models.

## 12. Optimization strategy

- Small fixed matrices: inline, unroll, scalar replacement, SIMD.
- Medium dense matrices: tiled MLIR Linalg/Vector lowering.
- Large dense matrices: dispatch to optimized BLAS/LAPACK.
- Sparse matrices: specialized storage and kernels only after stable semantics.
- Repeated kinematics: cache model constants and precompute symbolic structure.
- Target-specific dispatch: generated from declared CPU features or runtime selection.

## 13. Numerical modes

- `strict`: preserves specified floating-point behavior;
- `fast`: enables reassociation and approximate operations;
- `deterministic`: constrains parallel reduction and backend choices;
- `realtime`: bans unbounded allocation and selected blocking operations.

The mode must be visible in build metadata and benchmark reports.

## 14. Required comparisons

KX math and robotics benchmarks must compare against optimized implementations such as:

- compiler-optimized C/C++;
- Eigen or equivalent matrix libraries;
- BLAS/LAPACK backends;
- established rigid-body/kinematics libraries;
- Python/NumPy and MATLAB for productivity and boundary overhead, not only kernel throughput.

Beating a naive triple loop is not evidence that the language outperforms C++.
