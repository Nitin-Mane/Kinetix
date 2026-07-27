# Security Policy

## Security-relevant areas

- compiler memory safety;
- malformed source and package inputs;
- linker and tool invocation;
- package registry and dependency integrity;
- unsafe KX operations;
- generated Python/MATLAB wrappers;
- ROS 2 network-facing code;
- embedded firmware and memory-mapped I/O.

## Reporting

Do not publish exploitable details before maintainers acknowledge the issue. A production repository must configure a private security-reporting channel before public releases.

## Compiler requirements

- Fuzz lexer, parser, deserializer, package manifest, and binding generator.
- Run sanitizer builds in CI.
- Avoid invoking shell commands with unescaped user input.
- Treat package build scripts as untrusted code.
- Verify downloaded toolchains and packages by cryptographic digest/signature.
- Never treat `unsafe` as compiler permission to miscompile; it only shifts certain proof obligations to the programmer.

## Interop requirements

- Validate shape, dtype, alignment, stride, and lifetime at language boundaries.
- Reject incompatible ABI layouts.
- Guard Python reference counts and GIL transitions.
- Keep MATLAB SDK components out of public source distributions.
- Treat ROS messages and parameters as untrusted external input.
