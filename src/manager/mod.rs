pub use id::{ProtoId, ProtoIdRef};
pub(crate) use maps::{HandleToName, NameToHandle};
pub(crate) use plugin::ProtoManagerPlugin;
pub use proto_manager::ProtoManager;

mod id;
mod maps;
mod proto_manager;
mod tracking;

mod plugin {
    use std::marker::PhantomData;

    use bevy::app::{App, Plugin};
    use bevy::asset::Asset;

    use crate::Prototypical;

    use super::{tracking, HandleToName, NameToHandle};

    pub(crate) struct ProtoManagerPlugin<T: Prototypical + Asset> {
        phantom: PhantomData<T>,
    }

    impl<T: Prototypical + Asset> Default for ProtoManagerPlugin<T> {
        fn default() -> Self {
            Self {
                phantom: PhantomData::default(),
            }
        }
    }

    impl<T: Prototypical + Asset> Plugin for ProtoManagerPlugin<T> {
        fn build(&self, app: &mut App) {
            app.init_resource::<HandleToName>()
                .init_resource::<NameToHandle>()
                .add_system(tracking::track_prototypes::<T>);
        }
    }
}
