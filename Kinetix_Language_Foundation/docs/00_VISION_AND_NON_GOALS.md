# Vision and Non-Goals

## Vision

Kinetix is a language for engineers who need numerical expressiveness and systems-level control in the same program. It should make robotics mathematics readable while keeping allocation, data layout, synchronization, and generated machine behavior visible.

## Core design pillars

### 1. Mathematical structure is part of the type system

A `mat<f64, 6, 7>` is not merely a pointer plus metadata. Its dimensions can guide validation, storage, unrolling, vectorization, and ABI generation.

### 2. Robotics semantics are compile-time information

Coordinate frames, physical units, transform directions, clock domains, and joint dimensions should prevent invalid operations before deployment.

### 3. Systems behavior is explicit

Heap allocation, blocking calls, raw pointers, volatile I/O, atomic ordering, device transfers, and unsafe operations must be identifiable in source code and tooling.

### 4. Interoperability precedes ecosystem replacement

KX should call existing C libraries, expose shared libraries, connect to Python/NumPy, produce MATLAB MEX wrappers, and participate in ROS 2. Rewriting mature ecosystems is wasteful.

### 5. Performance claims are empirical

The language should enable optimization, not market fantasy. C++ can match any machine-level strategy KX uses when equivalent information and effort are supplied.

## Non-goals for 1.0

- replacing all C++ use cases;
- implementing a new robotics middleware;
- replacing MATLAB/Simulink;
- replacing Python as a general scripting language;
- providing a garbage-collected application platform;
- direct compatibility with arbitrary C++ classes/templates;
- unrestricted metaprogramming;
- browser and mobile application development;
- a custom GPU driver stack;
- self-hosting for prestige.

## Success definition

KX is successful if it makes selected robotics systems:

- harder to write incorrectly;
- easier to inspect and integrate;
- no slower than optimized native alternatives within declared tolerances;
- faster on selected kernels where domain-specific specialization is demonstrably responsible;
- more deterministic under real-time constraints;
- easier to bridge between ROS, Python, MATLAB, and native firmware.
