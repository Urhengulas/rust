//! Outline non-generic part of generic function
//!
//! ```
//! // before
//! fn outer<T: Into<U>>(t: T) {
//!     let u: U = t.into();
//!     u.v():
//!     u.w();
//! }
//!
//! // after
//! fn outer<T: Into<U>>(t: T) {
//!     fn inner(u: U) {
//!         u.v();
//!         u.w();
//!     }
//!     let u: U = t.into();
//!     inner(u);
//! }
//! ```
//!
//! # Logic
//!
//! 1. Check assumptions about `fn outer`
//!     1. Exactly one argument
//!     2. Arguments type is a generic `Into<T>`
//!     3. First line of function is `let u: U = t.into()`
//! 2. Create `fn inner`
//!     1. Create new function `fn inner(u: U)`
//!     2. Configure type signature
//!     3. Copy all code except first line from outer to inner
//! 3. Modify `fn outer`
//!     1. Remove all code except first line
//!     2. Call inner

use crate::MirPass;
use rustc_middle::{mir::*, ty::TyCtxt};

pub struct Outline;

impl<'tcx> MirPass<'tcx> for Outline {
    fn is_enabled(&self, sess: &rustc_session::Session) -> bool {
        sess.mir_opt_level() >= 3
    }

    fn run_pass(&self, _tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        // OutlineGeneric { tcx }.visit_body_preserves_cfg(body);

        let a = match body.basic_blocks_mut().iter_mut().nth(0) {
            Some(a) => a,
            None => return,
        };

        // dummy code just modify MIR somehow
        a.statements.push(a.statements.get(0).unwrap().clone());
    }
}

// struct OutlineGeneric<'tcx> {
//     tcx: TyCtxt<'tcx>,
// }

// impl<'tcx> MutVisitor<'tcx> for OutlineGeneric<'tcx> {
//     #[inline]
//     fn tcx(&self) -> TyCtxt<'tcx> {
//         self.tcx
//     }
// }
