# Risk Register

| Risk | Severity | Probability | Mitigation |
|---|---:|---:|---|
| Scope becomes “replace C++, Python, MATLAB, and ROS” | Critical | High | Enforce milestone boundaries and non-goals |
| Universal performance claims damage credibility | High | High | Reproducible benchmark policy and optimized baselines |
| Compiler team underestimates diagnostics and tooling | High | High | Treat DX as core deliverable, not final polish |
| Full borrow checker stalls project | High | Medium | Begin with lexical views and explicit unsafe boundaries |
| MLIR API churn consumes effort | High | Medium | Pin LLVM major and isolate compatibility layer |
| Matrix syntax lowers too early and loses optimization structure | High | Medium | Preserve high-level operations in KX/MLIR dialect |
| Hidden allocations violate real-time goals | Critical | Medium | Allocation effects, audit tooling, realtime profile |
| ROS integration binds too tightly to rclcpp | High | Medium | Build on rcl and generated type support |
| MATLAB CI unavailable or licensing blocks releases | Medium | High | Optional package and private licensed runner |
| Python zero-copy creates lifetime bugs | Critical | Medium | Strict ownership checks and copied fallback |
| Direct C++ ABI effort explodes | High | High | Use C wrapper ABI for 1.0 |
| Package registry becomes supply-chain liability | High | Medium | Delay registry, signed artifacts, lockfiles, sandboxing |
| Solo developer burnout | Critical | High | Narrow first research target and publish staged results |
| Self-hosting distracts from usability | Medium | Medium | Defer until post-1.0 unless technically required |
| Robotics types become too opinionated | Medium | Medium | RFCs, explicit conventions, runtime boundary validation |
| Numerical results differ across targets | High | Medium | Numerical modes, tolerance policy, deterministic profile |

## Stop conditions

Pause or reduce scope when:

- no end-to-end executable exists after the scalar phase;
- the compiler repeatedly miscompiles code;
- no contributor can maintain the MLIR lowering;
- benchmarks show no advantage and usability is not improved;
- interop layers require unstable hacks rather than supported APIs;
- the project cannot sustain tests across promised platforms.
