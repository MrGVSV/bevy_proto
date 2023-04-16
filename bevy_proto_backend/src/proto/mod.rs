//! The core of prototypes.

pub use assets::*;
pub use commands::*;
pub use component::*;
pub use config::*;
pub use error::*;
pub use prototypes::*;
pub use prototypical::*;
pub(crate) use storage::*;

mod assets;
mod commands;
mod component;
mod config;
mod error;
mod prototypes;
mod prototypical;
mod storage;
