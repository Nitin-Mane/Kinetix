//! Hindley-Milner type inference for Kinetix.
//!
//! Uses a union-find structure to unify type variables.

use std::collections::HashMap;
use crate::ty::{Ty, TyVar};

/// The inference context — tracks type variable bindings.
pub struct InferCtx {
    /// Next type variable ID to allocate.
    next_var: u32,
    /// Union-find parent map: var → representative.
    parent: HashMap<u32, u32>,
    /// Substitution: representative → concrete type.
    subst: HashMap<u32, Ty>,
}

impl InferCtx {
    pub fn new() -> Self {
        Self {
            next_var: 0,
            parent: HashMap::new(),
            subst: HashMap::new(),
        }
    }

    /// Allocate a fresh type variable.
    pub fn fresh_var(&mut self) -> Ty {
        let v = self.next_var;
        self.next_var += 1;
        self.parent.insert(v, v);
        Ty::Var(TyVar(v))
    }

    /// Find the representative of a type variable (path compression).
    pub fn find(&mut self, v: u32) -> u32 {
        let parent = *self.parent.get(&v).unwrap_or(&v);
        if parent == v {
            return v;
        }
        let root = self.find(parent);
        self.parent.insert(v, root);
        root
    }

    /// Resolve a type, following all type variable bindings.
    pub fn resolve(&mut self, ty: &Ty) -> Ty {
        match ty {
            Ty::Var(TyVar(v)) => {
                let root = self.find(*v);
                if let Some(resolved) = self.subst.get(&root).cloned() {
                    self.resolve(&resolved)
                } else {
                    Ty::Var(TyVar(root))
                }
            }
            Ty::Vec(inner) => Ty::Vec(Box::new(self.resolve(inner))),
            Ty::Map(k, v) => Ty::Map(Box::new(self.resolve(k)), Box::new(self.resolve(v))),
            Ty::Matrix(t) => Ty::Matrix(Box::new(self.resolve(t))),
            Ty::Tuple(ts) => Ty::Tuple(ts.iter().map(|t| self.resolve(t)).collect()),
            Ty::Optional(t) => Ty::Optional(Box::new(self.resolve(t))),
            other => other.clone(),
        }
    }

    /// Unify two types — returns `Ok(())` on success, `Err` on mismatch.
    pub fn unify(&mut self, a: &Ty, b: &Ty) -> Result<(), String> {
        let a = self.resolve(a);
        let b = self.resolve(b);

        match (&a, &b) {
            _ if a == b => Ok(()),

            (Ty::Var(TyVar(v)), other) | (other, Ty::Var(TyVar(v))) => {
                let root = self.find(*v);
                // Occurs check
                if self.occurs(root, other) {
                    return Err(format!("recursive type: ?{root} = {}", other.describe()));
                }
                self.subst.insert(root, other.clone());
                Ok(())
            }

            (Ty::Vec(a), Ty::Vec(b)) => self.unify(a, b),
            (Ty::Matrix(a), Ty::Matrix(b)) => self.unify(a, b),
            (Ty::Optional(a), Ty::Optional(b)) => self.unify(a, b),
            (Ty::Map(ak, av), Ty::Map(bk, bv)) => {
                self.unify(ak, bk)?;
                self.unify(av, bv)
            }
            (Ty::Tuple(as_), Ty::Tuple(bs)) if as_.len() == bs.len() => {
                for (a, b) in as_.iter().zip(bs.iter()) {
                    self.unify(a, b)?;
                }
                Ok(())
            }

            _ => Err(format!("type mismatch: expected `{}`, found `{}`", a.describe(), b.describe())),
        }
    }

    /// Check whether type variable `v` occurs in `ty` (for occurs check).
    fn occurs(&mut self, v: u32, ty: &Ty) -> bool {
        match ty {
            Ty::Var(TyVar(w)) => self.find(*w) == v,
            Ty::Vec(t) | Ty::Matrix(t) | Ty::Optional(t) => self.occurs(v, t),
            Ty::Map(k, val) => self.occurs(v, k) || self.occurs(v, val),
            Ty::Tuple(ts) => ts.iter().any(|t| self.occurs(v, t)),
            _ => false,
        }
    }
}

impl Default for InferCtx {
    fn default() -> Self { Self::new() }
}
