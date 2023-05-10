use crate::config::ProtoConfig;
use crate::prelude::Prototypes;
use crate::proto::Prototype;
use bevy_proto_backend::proto::Config;
use std::marker::PhantomData;

/// Run condition that returns true if the [prototype] with the given
/// ID is loaded and ready to be used.
///
/// [prototype]: Prototype
pub fn prototype_ready<I: ToString>(id: I) -> impl Fn(Prototypes<'_>) -> bool {
    ProtoCondition::prototype_ready(id)
}

/// A collection of common run conditions for use with custom [`Config`] types.
pub struct ProtoCondition<C: Config<Prototype> = ProtoConfig>(PhantomData<C>);

impl<C: Config<Prototype>> ProtoCondition<C> {
    /// Run condition that returns true if the [prototype] with the given
    /// ID is loaded and ready to be used.
    ///
    /// [prototype]: Prototype
    pub fn prototype_ready<I: ToString>(id: I) -> impl Fn(Prototypes<'_, C>) -> bool {
        let id = id.to_string();
        move |prototypes: Prototypes<C>| prototypes.is_ready(&id)
    }
}
