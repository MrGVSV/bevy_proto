use crate::prelude::Prototypes;

/// Run condition that returns true if the [prototype] with the given
/// ID is loaded and ready to be used.
///
/// [prototype]: crate::prelude::Prototype
pub fn prototype_ready<I: ToString>(id: I) -> impl Fn(Prototypes<'_>) -> bool {
    let id = id.to_string();
    move |prototypes: Prototypes| prototypes.is_ready(&id)
}
