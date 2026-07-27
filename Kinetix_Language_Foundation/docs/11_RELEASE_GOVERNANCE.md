# Release and Governance Plan

## 1. Versioning

- `0.x`: unstable language; migration notes required.
- `1.x`: stable source language and declared standard-library APIs.
- Compiler and package-tool versions move together initially.

## 2. Editions

Use editions only when necessary to preserve old syntax/semantics while enabling major improvements. Do not invent yearly editions as marketing.

## 3. Release cadence

Recommended after initial stabilization:

- patch: as needed for defects/security;
- minor: approximately quarterly;
- major: rare and migration-driven.

LLVM upgrades do not automatically require a KX major version, but code-generation changes must be qualified.

## 4. Release artifacts

- signed source archive;
- compiler binaries;
- checksums/signatures;
- SBOM;
- release notes;
- migration guide;
- benchmark report;
- supported-target matrix;
- known issues;
- reproducible build instructions.

## 5. Support policy

Define one current stable line and one previous stable line once adoption exists. Before adoption, do not promise long support windows the project cannot staff.

## 6. Deprecation

- warning introduced;
- documented replacement;
- automated migration where possible;
- removal only in a permitted compatibility window.
