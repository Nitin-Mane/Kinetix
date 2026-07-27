//! # kinetix_vm
//!
//! Register-based virtual machine for the Kinetix bytecode.

pub mod value;
pub mod frame;
pub mod vm;

pub use value::{Value, KxVec, KxMatrix, KxObj, NativeFn, KxClosure};
pub use frame::CallFrame;
pub use vm::{Vm, VmError, VmResult};

