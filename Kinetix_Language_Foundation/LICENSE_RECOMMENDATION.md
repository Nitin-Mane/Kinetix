# License Recommendation

Do not choose a license casually after code has accumulated from multiple contributors.

## Recommended default

**Apache License 2.0 with LLVM exception where appropriate for compiler/runtime components**, or a project-wide Apache-2.0 approach after legal review.

Reasons:

- permissive commercial and academic use;
- explicit patent grant;
- familiar in compiler ecosystems;
- suitable for adoption in robotics and industry.

## Alternative

Apache-2.0 for compiler/runtime and MIT for small generated bindings/templates. Multiple licenses increase administrative burden and should be avoided unless necessary.

## Warning

Do not copy GPL or proprietary code into the compiler unless the project intentionally accepts the resulting obligations. MATLAB headers and binaries must not be redistributed.

This file is a technical recommendation, not legal advice. Add an actual `LICENSE` only after the project owner makes the decision.
