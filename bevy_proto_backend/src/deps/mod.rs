//! Items for adding dependencies to [prototypes].
//!
//! A dependency is any non-[child] and non-[template] asset.
//! This is used to store (and thus, keep loaded) assets during
//! the lifetime of the prototype.
//!
//! [prototypes]: crate::proto::Prototypical
//! [child]: crate::children
//! [template]: crate::templates

pub use collection::*;

mod collection;
