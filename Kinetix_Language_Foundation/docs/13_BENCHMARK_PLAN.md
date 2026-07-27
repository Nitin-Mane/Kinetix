# Formal Benchmark Plan

## 1. Purpose

Determine where KX is competitive, not manufacture a favorable headline.

## 2. Baselines

- C compiled with Clang at equivalent optimization and target settings;
- C++ with Clang and optimized libraries;
- Eigen for fixed and dynamic dense matrices;
- optimized BLAS/LAPACK;
- established robotics libraries for transformations and kinematics;
- Python/NumPy and MATLAB for end-to-end interoperability/productivity comparisons;
- ROS 2 C++ and Python client libraries for communication tests.

## 3. Benchmark groups

### Scalar and bit operations

- integer arithmetic modes;
- bit extraction/packing;
- CRC/checksum;
- finite-state machine;
- parsing binary sensor packets.

### Small fixed matrices

- 2×2, 3×3, 4×4, 6×6;
- multiplication;
- transpose;
- determinant/inverse where numerically appropriate;
- Cholesky/QR through selected implementations.

### Dynamic matrices

- common dense sizes;
- memory-layout variants;
- contiguous and strided views;
- BLAS dispatch thresholds.

### Robotics

- transform composition;
- point-cloud transform;
- quaternion normalization/composition;
- forward kinematics;
- Jacobian;
- damped least-squares inverse-kinematics step;
- state-estimation update.

### Interop

- C function call overhead;
- Python extension call overhead;
- NumPy zero-copy and copy paths;
- MATLAB MEX call overhead;
- ROS 2 pub/sub throughput and latency.

### Real-time

- control-loop latency;
- jitter distribution;
- allocation count;
- lock contention;
- callback deadline miss rate.

## 4. Method

Record:

- hardware model and microcode;
- CPU governor and thermal state;
- OS/kernel;
- compiler versions;
- LLVM version;
- exact flags;
- library versions;
- input data and seed;
- warmup;
- repetitions;
- median, p95/p99 where relevant, and dispersion;
- correctness tolerance;
- allocation count;
- code size;
- compile time separately from runtime.

## 5. Fairness rules

- Same precision and algorithm.
- Equivalent safety checks reported.
- Include setup/transfer time when the application pays it.
- Report both kernel-only and end-to-end results when useful.
- Use optimized baseline code reviewed by someone competent in that ecosystem.
- Publish failures and regressions.
- Do not remove unfavorable workloads without explanation.

## 6. Acceptance thresholds

KX need not win every test. Suggested interpretation:

- within ±5%: effectively comparable for many workloads;
- >5% slower: investigate generated code and abstraction overhead;
- >10% faster: require assembly/IR explanation and repeatability;
- lower average but lower jitter: potentially superior for real-time use;
- same speed with compile-time frame/unit safety: meaningful engineering benefit.

## 7. Reporting template

```text
Benchmark:
KX revision:
Baseline revision:
Hardware:
OS:
Compilers and flags:
Libraries:
Correctness check:
Warmup and repetitions:
Median:
P95/P99:
Memory allocations:
Code size:
Generated-code explanation:
Limitations:
```
