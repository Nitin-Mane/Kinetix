//! # kinetix_gc
//!
//! Garbage collection layer for the Kinetix runtime.
//!
//! Kinetix uses **reference counting** (via `Arc<T>`) as its primary memory
//! management strategy. This crate adds a **cycle detector** on top to
//! handle reference cycles (e.g., objects that reference each other).
//!
//! ## How it works
//!
//! 1. All heap objects are wrapped in `Arc<Mutex<T>>`.
//! 2. The reference count is maintained by Rust's `Arc`.
//! 3. Periodically, or when memory pressure is detected, the cycle
//!    detector runs a tri-color mark-and-sweep over the live object graph.
//! 4. Cyclic garbage is freed by breaking the cycle.
//!
//! In Phase 1, cycles are rare (the type system discourages them), so
//! this crate mostly serves as documentation and a hook for future work.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use log::debug;

/// Handle to a GC-managed object slot.
pub type GcHandle = u64;

/// GC statistics (for monitoring / debug).
#[derive(Debug, Default, Clone)]
pub struct GcStats {
    pub allocations:   u64,
    pub deallocations: u64,
    pub cycle_collections: u64,
    pub live_objects:  usize,
}

/// The Kinetix garbage collector.
pub struct Gc {
    next_handle: GcHandle,
    stats:       GcStats,
}

impl Gc {
    pub fn new() -> Self {
        Self { next_handle: 0, stats: GcStats::default() }
    }

    /// Allocate a new GC handle (reference-counting happens automatically
    /// via `Arc`; this just tracks allocation statistics).
    pub fn alloc(&mut self) -> GcHandle {
        let h = self.next_handle;
        self.next_handle += 1;
        self.stats.allocations += 1;
        self.stats.live_objects += 1;
        h
    }

    /// Notify the GC that an object was deallocated (Arc dropped to zero).
    pub fn dealloc(&mut self) {
        self.stats.deallocations += 1;
        self.stats.live_objects = self.stats.live_objects.saturating_sub(1);
    }

    /// Run the cycle detector. In Phase 1 this is a no-op placeholder.
    pub fn collect_cycles(&mut self) {
        debug!("GC: cycle collection (no-op in Phase 1)");
        self.stats.cycle_collections += 1;
    }

    pub fn stats(&self) -> &GcStats { &self.stats }
}

impl Default for Gc {
    fn default() -> Self { Self::new() }
}
