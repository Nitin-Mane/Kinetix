//! # kinetix_jit
//!
//! JIT compilation backend for the Kinetix language using **Cranelift**.
//!
//! ## Architecture
//!
//! ```text
//! Kinetix Bytecode  →  Cranelift IR  →  Native Machine Code  →  Execute
//!     (Chunk)           (CLIF)          (x86_64 / aarch64)
//! ```
//!
//! The JIT compiler translates Kinetix bytecode chunks into native machine
//! code at runtime using the Cranelift code generator — the same backend
//! used by the Wasmtime WebAssembly runtime.
//!
//! ## Hot-path JIT
//!
//! Functions are initially executed by the bytecode VM.  When a function's
//! execution count exceeds [`JitThreshold::DEFAULT`], it is JIT-compiled
//! and subsequent calls use the native code path.
//!
//! ## Status
//!
//! Phase 1 provides the Cranelift module setup, ISA detection, and a
//! scaffolded function compiler.  Full bytecode → CLIF lowering is
//! implemented in Phase 1d.

use cranelift_codegen::settings::{self, Configurable};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::Module;
use cranelift_native;
use thiserror::Error;

/// How many times a function must be called before it is JIT-compiled.
pub const JIT_THRESHOLD: u32 = 100;

#[derive(Debug, Error)]
pub enum JitError {
    #[error("Cranelift ISA error: {0}")]
    Isa(String),
    #[error("Cranelift module error: {0}")]
    Module(String),
    #[error("JIT compilation error: {0}")]
    Compile(String),
    #[error("function not found: `{0}`")]
    NotFound(String),
}

/// The Kinetix JIT compiler.
pub struct KinetixJit {
    module: JITModule,
}

impl KinetixJit {
    /// Create a new JIT instance targeting the current host CPU.
    pub fn new() -> Result<Self, JitError> {
        // Detect native ISA (x86_64, aarch64, etc.)
        let mut flag_builder = settings::builder();
        flag_builder.set("use_colocated_libcalls", "false")
            .map_err(|e| JitError::Isa(e.to_string()))?;
        flag_builder.set("is_pic", "false")
            .map_err(|e| JitError::Isa(e.to_string()))?;
        flag_builder.set("opt_level", "speed_and_size")
            .map_err(|e| JitError::Isa(e.to_string()))?;

        let isa_builder = cranelift_native::builder()
            .map_err(|e| JitError::Isa(e.to_string()))?;
        let isa = isa_builder
            .finish(settings::Flags::new(flag_builder))
            .map_err(|e| JitError::Isa(e.to_string()))?;

        let builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
        let module = JITModule::new(builder);
        Ok(Self { module })
    }

    /// Returns the host ISA description (for informational display).
    pub fn isa_name(&self) -> String {
        self.module.isa().name().to_owned()
    }

    /// Finalize all pending compilations and make native code executable.
    pub fn finalize(&mut self) {
        self.module.finalize_definitions().ok();
    }
}

/// Per-function execution counter (used to decide when to JIT-compile).
#[derive(Debug, Default)]
pub struct CallCounter {
    counts: std::collections::HashMap<String, u32>,
}

impl CallCounter {
    pub fn increment(&mut self, name: &str) -> u32 {
        let c = self.counts.entry(name.to_owned()).or_insert(0);
        *c += 1;
        *c
    }

    pub fn should_jit(&self, name: &str) -> bool {
        self.counts.get(name).copied().unwrap_or(0) >= JIT_THRESHOLD
    }
}
