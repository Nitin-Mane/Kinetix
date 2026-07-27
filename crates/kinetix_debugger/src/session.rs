//! Debug session state management.

use std::path::PathBuf;
use crate::breakpoint::BreakpointTable;

/// Commands sent from the IDE to the debugger.
#[derive(Debug, Clone)]
pub enum DebugCommand {
    /// Start/continue execution.
    Continue,
    /// Step over the current line.
    StepOver,
    /// Step into the current function call.
    StepInto,
    /// Step out of the current function.
    StepOut,
    /// Pause execution.
    Pause,
    /// Terminate the debug session.
    Terminate,
    /// Add a breakpoint.
    SetBreakpoint { file: PathBuf, line: u32 },
    /// Remove a breakpoint.
    RemoveBreakpoint { file: PathBuf, line: u32 },
    /// Evaluate expression in the current scope.
    Evaluate { expression: String },
}

/// Events sent from the debugger to the IDE.
#[derive(Debug, Clone)]
pub enum DebugEvent {
    /// Execution stopped at a breakpoint.
    Stopped {
        reason: StopReason,
        file:   PathBuf,
        line:   u32,
    },
    /// Execution completed.
    Terminated { exit_code: i32 },
    /// Output from the program.
    Output { category: String, output: String },
    /// Breakpoint was verified (resolved to an actual code location).
    BreakpointVerified { file: PathBuf, line: u32 },
    /// Evaluation result.
    EvalResult { result: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum StopReason {
    Breakpoint,
    Step,
    Pause,
    Exception(String),
    Entry,
}

/// The active debug session.
pub struct DebugSession {
    pub breakpoints: BreakpointTable,
    pub state:       SessionState,
    /// Stack frames at the current pause point.
    pub stack:       Vec<StackFrame>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    NotStarted,
    Running,
    Paused,
    Terminated,
}

/// A single frame in the call stack.
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub name:   String,
    pub file:   Option<PathBuf>,
    pub line:   u32,
    pub column: u32,
    /// Local variables visible at this frame.
    pub locals: Vec<(String, String)>,  // (name, display_value)
}

impl DebugSession {
    pub fn new() -> Self {
        Self {
            breakpoints: BreakpointTable::new(),
            state:       SessionState::NotStarted,
            stack:       Vec::new(),
        }
    }

    pub fn is_running(&self) -> bool { self.state == SessionState::Running }
    pub fn is_paused(&self) -> bool  { self.state == SessionState::Paused  }
}

impl Default for DebugSession {
    fn default() -> Self { Self::new() }
}
