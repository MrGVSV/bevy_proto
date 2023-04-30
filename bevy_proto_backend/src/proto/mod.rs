//! The core of prototypes.

pub use assets::*;
#[cfg(feature = "bevy_render")]
pub use color::*;
pub use commands::*;
pub use component::*;
pub use config::*;
pub use error::*;
pub use event::*;
pub use prototypes::*;
pub use prototypical::*;
pub(crate) use storage::*;

mod assets;
#[cfg(feature = "bevy_render")]
mod color;
mod commands;
mod component;
mod config;
mod error;
mod event;
mod prototypes;
mod prototypical;
mod storage;
