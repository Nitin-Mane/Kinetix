# ADR 0002: C ABI as interoperability anchor

- Status: Accepted for foundation phase
- Decision: Use a versioned C-compatible ABI as the stable bridge for native interoperability.

## Rationale

C ABIs are broadly supported by C++, Python extension systems, MATLAB MEX/Engine adapters, ROS 2 `rcl`, operating systems, and embedded toolchains. Direct arbitrary C++ ABI support would introduce compiler- and standard-library-specific complexity.

## Consequences

- KX exports can generate C headers.
- C++ libraries require a C facade for stable integration.
- Python/MATLAB/ROS bindings are generated around stable native representations.
