//! The core of prototypes.

#[cfg(feature = "bevy_render")]
pub use color::*;
pub use commands::*;
pub use component::*;
pub use config::*;
pub use error::*;
pub use prototypes::*;
pub use prototypical::*;
pub(crate) use storage::*;

#[cfg(feature = "bevy_render")]
mod color;
mod commands;
mod component;
mod config;
mod error;
mod prototypes;
mod prototypical;
mod storage;
