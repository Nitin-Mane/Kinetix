# Governance

## Roles

- **Maintainers:** approve releases, specifications, security actions, and repository access.
- **Language design team:** owns accepted semantics and RFC decisions.
- **Compiler team:** owns front end, IR, optimization, and code generation.
- **Runtime and interoperability team:** owns ABI, standard runtime, Python, MATLAB, and ROS integration.
- **Release manager:** owns release qualification and signed artifacts.

## Decision model

- Minor implementation choices: pull-request consensus.
- Stable semantics, ABI, memory model, or syntax: RFC required.
- Urgent security fixes: maintainers may act before public RFC discussion.
- Disputed changes: documented decision with alternatives and tradeoffs; no silent maintainer fiat.

## Stability levels

- `experimental`: may change without migration support;
- `preview`: expected direction, migration notes required;
- `stable`: compatibility governed by semantic versioning;
- `deprecated`: remains available for a documented transition period.

## Conflict of interest

Benchmark authors must disclose when they also authored the comparison baseline or selected workloads specifically favorable to KX.
