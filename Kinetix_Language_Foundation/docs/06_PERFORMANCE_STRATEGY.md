# Performance Strategy

## 1. What “outperform C++” can and cannot mean

A language does not beat C++ merely because its compiler uses LLVM or because it has matrix syntax. Optimized C++ can use identical instructions and libraries.

Defensible KX goals:

- equal or better generated code for equivalent scalar kernels;
- less effort to reach optimized matrix and robotics kernels;
- compile-time specialization using shapes, frames, units, sparsity, and target metadata;
- lower boundary overhead than Python/MATLAB for native kernels;
- deterministic memory and scheduling behavior;
- clearer optimization diagnostics.

Indefensible goal:

- “KX is faster than C++ for all programs.”

## 2. Sources of possible advantage

### Static dimensions

Generate dedicated kernels for 3×3, 4×4, 6×6, and robot-specific Jacobians.

### Alias information

Views and mutable references provide stronger alias facts than arbitrary C++ pointers.

### Domain operations

Preserve matrix multiplication, transform composition, and reductions in high-level IR long enough for fusion and layout planning.

### Units and frames

Mostly a correctness feature, but can remove repeated runtime validation inside trusted code.

### Real-time profiles

Ban hidden allocation and unbounded behavior, improving predictability rather than average throughput.

### Backend dispatch

Choose unrolled code, MLIR-generated kernels, BLAS/LAPACK, or accelerators using a documented cost model.

## 3. Performance contract

Each operation documents:

- asymptotic complexity;
- allocation behavior;
- vectorization expectations;
- thread behavior;
- backend selection;
- determinism mode;
- numerical mode;
- possible copies or transfers.

## 4. Initial measurable targets

These are engineering targets, not promises:

- scalar code within 5% of Clang-optimized equivalent C++ on the same LLVM major;
- fixed small-matrix kernels no slower than Eigen’s optimized fixed-size path on the reference benchmark set;
- selected frame-safe kinematics kernels within 5% of optimized C++ baselines while eliminating classes of frame/dimension errors;
- Python calls achieving near-native kernel throughput with boundary overhead separately reported;
- ROS 2 pub/sub latency comparable to C++ under equivalent executor and QoS settings;
- no dynamic allocation inside declared `@realtime` steady-state loops.

A 10–30% win may be possible in selected kernels, but it must emerge from measurements and explainable specialization.

## 5. Optimization remarks

`kxc` should explain decisions:

```text
remark: specialized matmul<4x4x4> using AVX2 unrolled kernel
remark: selected external dgemm because dimensions exceed local-kernel threshold
remark: inserted copy: NumPy input is non-contiguous
remark: vectorization blocked by possible alias between output and input
```

## 6. Profiles

- `debug`: checks and rich diagnostics;
- `release`: safe optimized defaults;
- `fast`: explicit relaxed floating-point rules;
- `deterministic`: reproducible reductions and scheduling restrictions;
- `realtime`: bounded memory/blocking policy;
- `size`: firmware size optimization.

## 7. Performance anti-patterns

Reject:

- best-of-one timing;
- debug C++ versus release KX;
- naive C++ loop versus KX calling BLAS;
- omitted host-device transfer;
- different numerical precision;
- incorrect results treated as faster;
- benchmark-specific compiler hacks hidden from users;
- comparisons without source and flags.
