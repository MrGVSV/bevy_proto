//! Utilities for handling [prototype] cycles.
//!
//! A prototype cycle is where a prototype recursively depends on itself.
//! This means it can be found within its own template inheritance chain
//! or within its own children hierarchy.
//!
//! For example, take the following set of prototypes:
//!
//! ```ignore
//! name: A,
//! templates: [B],
//! children: [C]
//! ---
//! name: B,
//! templates: [D],
//! children: []
//! ---
//! name: C,
//! templates: [],
//! children: [A]
//! ---
//! name: D,
//! templates: [],
//! children: [B]
//! ```
//!
//! If we try to load `A`, we come across two cycles.
//!
//! One of them is in `A`'s children hierarchy:
//!
//! ```ignore
//! A contains C which contains A
//! ```
//!
//! The other is in `A`'s template inheritance chain:
//!
//! ```ignore
//! "A" inherits "B" which inherits "D" which contains "B"
//! ```
//!
//! These prototypes will need to be redesigned in order to avoid the cycle.
//!
//! Alternatively, if this behavior is expected— or even intended—
//! the cycle can be ignored with the proper [`CycleResponse`].
//!
//! [prototype]: crate::proto::Prototypical

pub use cycle::*;
pub(crate) use node::*;
pub use response::*;

mod cycle;
mod node;
mod response;
