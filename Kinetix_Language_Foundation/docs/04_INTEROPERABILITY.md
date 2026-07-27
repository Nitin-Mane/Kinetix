# Interoperability Architecture

## 1. Principle

KX should not build four unrelated bridges. It should establish a stable C-compatible core ABI and generate specialized wrappers around it.

```text
                 Python / NumPy
                       |
MATLAB / MEX --- KX C ABI --- C libraries
                       |
                  ROS 2 rcl
```

## 2. C integration

### Import

```kx
extern "C" library "libmotor" {
    fn motor_set_speed(id: i32, rpm: f64) -> i32;
}
```

### Export

```kx
@export("kx_fk")
@repr(C)
fn forward_kinematics(q: view<f64>, out: mut_view<f64>) -> i32;
```

### Binding generator

A constrained C header importer may handle:

- primitive types;
- pointers;
- fixed arrays;
- enums;
- plain structures;
- function declarations;
- selected macros/constants.

It should reject or require manual bindings for:

- compiler-specific extensions;
- complex macro APIs;
- variadic functions without explicit wrappers;
- C++ declarations;
- ambiguous ownership.

## 3. C++ integration

Direct arbitrary C++ ABI integration is not a 1.0 goal. Use an `extern "C"` facade around C++ libraries. This avoids compiler-specific name mangling, templates, exceptions, RTTI, and standard-library ABI differences.

## 4. Python integration

Two directions:

### Python calls KX

- generate CPython extension module;
- expose scalar, string, structure, and array APIs;
- emit `.pyi` type stubs;
- generate wheels.

### KX embeds Python

- explicit runtime initialization;
- explicit interpreter ownership;
- call Python under a foreign/unsafe boundary;
- surface Python exceptions as KX error values;
- document GIL and thread restrictions.

### NumPy bridge

A zero-copy bridge requires:

- compatible dtype;
- compatible endianness;
- valid alignment;
- known shape and strides;
- safe lifetime;
- compatible mutability;
- compatible memory space.

Otherwise, copy and say so. Silent copies are unacceptable in performance-sensitive APIs.

## 5. MATLAB integration

### MATLAB calls KX

Generate a MEX wrapper that:

- validates input count and type;
- maps MATLAB arrays to KX views where legal;
- allocates MATLAB-owned output arrays;
- translates KX errors to MATLAB errors;
- preserves column-major layout or performs an explicit conversion.

### KX calls MATLAB

Use the MATLAB Engine API through a C/C++ adapter. This path:

- requires MATLAB installation and licensing;
- is not appropriate for hard real-time execution;
- should be optional and isolated from core runtime;
- should expose potentially blocking behavior.

## 6. ROS 2 integration

Use `rcl` and rosidl-generated type support. See `05_ROS2_INTEGRATION.md`.

## 7. Data-contract metadata

Generated bindings should state:

- ownership;
- mutability;
- lifetime;
- shape;
- strides;
- units where transport supports metadata;
- frame conventions;
- error model;
- thread safety;
- allocation behavior.

## 8. ABI versioning

- KX-internal ABI may change before 1.0.
- Exported C ABI is controlled by explicit versioned symbols or library major versions.
- Generated binding metadata stores compiler and ABI versions.
- Stable language source compatibility does not automatically guarantee binary compatibility.

## 9. Test matrix

- KX -> C and C -> KX;
- KX -> Python and Python -> KX;
- KX -> MATLAB and MATLAB -> KX where licensed;
- KX ROS node -> C++/Python nodes;
- structure layout on Linux/Windows/macOS;
- x86-64 and AArch64;
- zero-copy and forced-copy paths;
- lifetime and error propagation.
