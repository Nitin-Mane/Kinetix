//! Breakpoint management for the Kinetix debugger.

use std::path::PathBuf;
use std::collections::HashMap;

/// Unique breakpoint identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BreakpointId(pub u32);

/// A source breakpoint.
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id:      BreakpointId,
    pub file:    PathBuf,
    pub line:    u32,
    pub column:  Option<u32>,
    pub enabled: bool,
    /// Optional condition expression (evaluated in the VM when hit).
    pub condition: Option<String>,
    /// Hit count — how many times this breakpoint has been triggered.
    pub hit_count: u32,
}

impl Breakpoint {
    pub fn new(id: BreakpointId, file: PathBuf, line: u32) -> Self {
        Self {
            id,
            file,
            line,
            column: None,
            enabled: true,
            condition: None,
            hit_count: 0,
        }
    }
}

/// The breakpoint table — maps (file, line) to breakpoints.
#[derive(Debug, Default)]
pub struct BreakpointTable {
    breakpoints: HashMap<BreakpointId, Breakpoint>,
    next_id:     u32,
    /// Reverse index: (file, line) → breakpoint IDs
    index:       HashMap<(PathBuf, u32), Vec<BreakpointId>>,
}

impl BreakpointTable {
    pub fn new() -> Self { Self::default() }

    /// Add a breakpoint and return its ID.
    pub fn add(&mut self, file: PathBuf, line: u32) -> BreakpointId {
        let id = BreakpointId(self.next_id);
        self.next_id += 1;
        let bp = Breakpoint::new(id, file.clone(), line);
        self.index.entry((file, line)).or_default().push(id);
        self.breakpoints.insert(id, bp);
        id
    }

    /// Remove a breakpoint by ID.
    pub fn remove(&mut self, id: BreakpointId) -> Option<Breakpoint> {
        if let Some(bp) = self.breakpoints.remove(&id) {
            if let Some(ids) = self.index.get_mut(&(bp.file.clone(), bp.line)) {
                ids.retain(|&i| i != id);
            }
            Some(bp)
        } else {
            None
        }
    }

    /// Check whether execution should pause at (file, line).
    pub fn should_break(&mut self, file: &PathBuf, line: u32) -> bool {
        let key = (file.clone(), line);
        if let Some(ids) = self.index.get(&key) {
            for id in ids.clone() {
                if let Some(bp) = self.breakpoints.get_mut(&id) {
                    if bp.enabled {
                        bp.hit_count += 1;
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get(&self, id: BreakpointId) -> Option<&Breakpoint> {
        self.breakpoints.get(&id)
    }

    pub fn all(&self) -> impl Iterator<Item = &Breakpoint> {
        self.breakpoints.values()
    }

    pub fn clear(&mut self) {
        self.breakpoints.clear();
        self.index.clear();
    }
}
