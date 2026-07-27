//! HIR top-level item nodes.

use smol_str::SmolStr;
use kinetix_types::Ty;
use crate::expr::HirBlock;

#[derive(Debug, Clone)]
pub enum HirItem {
    Fn(HirFn),
    // Struct, Enum, Impl etc. to be added in Phase 1b
}

#[derive(Debug, Clone)]
pub struct HirFn {
    pub name:     SmolStr,
    pub params:   Vec<HirParam>,
    pub ret_ty:   Ty,
    pub body:     Option<HirBlock>,
    pub is_async: bool,
}

#[derive(Debug, Clone)]
pub struct HirParam {
    pub name: SmolStr,
    pub ty:   Ty,
}
