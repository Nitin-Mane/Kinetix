//! # kinetix_debugger
//!
//! Debug Adapter Protocol (DAP) implementation for the Kinetix language.
//!
//! This allows IDEs that support DAP (VS Code, kinetix_ide, Neovim, etc.)
//! to provide full debugging capabilities:
//!
//! - Set/remove breakpoints
//! - Step over / step into / step out
//! - Inspect local variables and the call stack
//! - Evaluate expressions at a breakpoint

pub mod breakpoint;
pub mod session;

pub use breakpoint::{Breakpoint, BreakpointId};
pub use session::{DebugSession, DebugEvent, DebugCommand};
