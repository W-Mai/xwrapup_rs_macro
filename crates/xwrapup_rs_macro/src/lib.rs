//! A widget.
//! ```
//! use xwrapup_rs_macro::ui;
//!
//! ui! {
//!     :(parent: parent)
//!
//!     Widget (attr0: value0, attr1: value1, attr2: value2, ...) {
//!         ChildWidget0 (attr0: value0, attr1: value1, attr2: value2, ...) {
//!            ...
//!         }
//!         ChildWidget1 (attr0: value0, attr1: value1, attr2: value2, ...) {
//!           ...
//!         }
//!         if (cond) {
//!            ...
//!         }
//!     }
//! }
//!
//! ```

extern crate proc_macros;

pub use proc_macros::ui;
