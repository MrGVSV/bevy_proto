//! Contains tree structures and accessors for [prototypes].
//!
//! [prototypes]: crate::proto::Prototypical

pub use access::*;
pub(crate) use builder::*;
pub use entity_tree::*;
pub(crate) use proto_tree::*;

mod access;
mod builder;
mod entity_tree;
mod proto_tree;
