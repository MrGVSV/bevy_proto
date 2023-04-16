//! Items relating to the main [`Prototype`] struct.

pub use child::*;
pub(crate) use de::*;
pub use error::*;
pub use prototype::*;

mod child;
mod de;
mod error;
mod prototype;
