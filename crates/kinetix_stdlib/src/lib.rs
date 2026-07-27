//! # kinetix_stdlib
//!
//! The Kinetix Standard Library.
//!
//! Each module in the stdlib registers native Rust functions that are
//! callable from Kinetix programs.
//!
//! ## Modules
//!
//! | Module        | Description                                      |
//! |---------------|--------------------------------------------------|
//! | `std::io`     | Console I/O, file I/O                            |
//! | `std::math`   | Trigonometry, logarithms, constants              |
//! | `std::matrix` | Matrix creation, decompositions, linear algebra  |
//! | `std::vec`    | Vec operations: sort, reverse, unique, ...       |
//! | `std::map`    | Map creation and manipulation                    |

pub mod io;
pub mod math;
pub mod matrix;
pub mod vec_module;
pub mod map_module;

use std::sync::Arc;
use kinetix_vm::{NativeFn, Value, Vm};

/// Register all standard library modules into a VM.
pub fn register_all(vm: &mut kinetix_vm::vm::Vm) {
    io::register(vm);
    math::register(vm);
    matrix::register(vm);
    vec_module::register(vm);
}
