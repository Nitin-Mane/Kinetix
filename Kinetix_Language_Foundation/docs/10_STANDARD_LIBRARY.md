# Standard Library Plan

## 1. Layering

### `core`

No allocator or OS dependency:

- primitive traits;
- result/option;
- fixed arrays;
- views;
- numeric operations;
- atomics;
- memory utilities;
- compile-time facilities.

### `alloc`

Optional allocation:

- owned buffers;
- vectors;
- strings;
- arenas;
- pools;
- shared ownership only where explicitly requested.

### `std`

Hosted facilities:

- files;
- processes;
- threads;
- sockets;
- clocks;
- environment;
- dynamic libraries.

## 2. Engineering libraries

### `kx.math`

Vectors, matrices, tensors, decompositions, sparse structures, random generators, statistics.

### `kx.geometry`

Frames, rotations, transforms, points, directions, spatial vectors.

### `kx.kinematics`

Robot models, joint state, forward/inverse kinematics, Jacobians.

### `kx.dynamics`

Rigid-body dynamics, inertia, forces, inverse/forward dynamics. This should come after kinematics is stable.

### `kx.control`

PID, state-space, filters, observers, trajectory control, constrained control interfaces.

### `kx.signal`

FFT wrappers, filters, windows, resampling, streaming buffers.

### `kx.time`

Durations, monotonic/system/ROS clock domains, timestamps, deadlines.

### `kx.io`

Serial, CAN, network, files, and platform abstractions. Hardware APIs should be packages rather than bloating the language core.

## 3. Backend policy

The standard library may use optimized external libraries behind stable KX APIs. Reimplementing BLAS, LAPACK, FFT, and every robotics algorithm before the language is usable would be irrational.

Backend choice must be inspectable and configurable.

## 4. Stability

Only a small set of APIs should be stable at 1.0. Experimental modules remain versioned separately or clearly marked.
