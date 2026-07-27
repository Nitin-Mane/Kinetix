# Toolchain and Developer Experience

## 1. Commands

```bash
kx new robot-controller
kx build
kx run
kx test
kx bench
kx fmt
kx check
kx doc
kx clean
kx bindgen header.h
kx py build
kx mex build
kx ros build
```

The low-level compiler remains available:

```bash
kxc source.kx -O3 --target=aarch64-linux-gnu --emit=asm
```

## 2. Manifest

```toml
[package]
name = "robot-controller"
version = "0.1.0"
edition = "2026"

[target]
triple = "aarch64-linux-gnu"
profile = "release"

[dependencies]
kx-math = "0.3"
rclkx = { version = "0.5", optional = true }
```

The final schema requires an RFC.

## 3. Diagnostics experience

Prioritize clear type, dimension, frame, unit, ownership, and boundary-conversion errors. “Easy to code” comes primarily from good diagnostics, predictable rules, and a small feature set—not syntax shortcuts.

## 4. Formatter

`kxfmt` should have few options. A canonical format reduces style arguments and simplifies generated-code testing.

## 5. Language server

Features:

- diagnostics;
- go to definition;
- references;
- hover types including shapes/frames/units;
- rename;
- completion;
- semantic tokens;
- inlay hints for inferred dimensions and conversions;
- optimization remarks;
- ROS message navigation.

## 6. Debugger

Use standard debug metadata and provide pretty printers for:

- vectors and matrices;
- transforms;
- quaternions;
- quantities;
- result/error values;
- views and ownership metadata.

## 7. Compiler explorer mode

Provide an easy way to inspect:

- KX AST;
- KX/MLIR stages;
- LLVM IR;
- assembly;
- optimization remarks;
- allocation report.

This is central to the language’s auditability claim.

## 8. Package security

- lockfiles;
- checksums;
- signed releases;
- dependency source transparency;
- build-script sandboxing direction;
- offline/vendor mode;
- software bill of materials.
