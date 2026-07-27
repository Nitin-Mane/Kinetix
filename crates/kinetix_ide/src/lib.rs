//! # kinetix_ide
//!
//! Embedded IDE engine and visual environment for the Kinetix language.
//!
//! Features:
//! - Code editor state with syntax highlighting, inline diagnostics, and cursor tracking.
//! - Live 2D matrix inspector / visualizer for linear algebra debugging.
//! - Integrated REPL runner using the Kinetix VM.
//! - Debugger state bindings (breakpoints, variables view, call stack).

pub mod editor;
pub mod visualizer;
pub mod repl;

pub use editor::{EditorState, DiagnosticItem};
pub use visualizer::{MatrixView, InspectableValue};
pub use repl::ReplSession;
