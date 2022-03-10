//! Provides the core abstractions [`Prototypical`] and [`Prototype`] for implementing prototypical structs.
use bevy::ecs::prelude::Commands;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::{AssetServer, Res};
use indexmap::IndexSet;
use std::fmt::Debug;
use std::slice::Iter;

use crate::components::ComponentList;
use crate::prelude::TemplateList;
use crate::{
    components::ProtoComponent, data::ProtoCommands, data::ProtoData, utils::handle_cycle,
};

/// Allows access to a prototype's name and components so that it can be spawned in
pub trait Prototypical: 'static + Send + Sync {
    /// The name of the prototype
    ///
    /// This should be unique amongst all prototypes in the world
    fn name(&self) -> &str;

    /// The names of the parent templates (if any).
    fn templates(&self) -> Option<&TemplateList> {
        None
    }

    /// Returns an iterator of [`ProtoComponent`] trait objects.
    fn iter_components(&self) -> Iter<'_, Box<dyn ProtoComponent>>;

    /// Creates [`ProtoCommands`] used to modify the given entity.
    ///
    /// # Arguments
    ///
    /// * `entity`: The entity commands
    /// * `data`: The prototype data in this world
    ///
    fn create_commands<'w, 's, 'a, 'p>(
        &'p self,
        entity: EntityCommands<'w, 's, 'a>,
        data: &'p Res<ProtoData>,
    ) -> ProtoCommands<'w, 's, 'a, 'p>;

    /// Spawns an entity with this prototype's component structure.
    ///
    /// # Arguments
    ///
    /// * `commands`: The world `Commands`
    /// * `data`: The prototype data in this world
    /// * `asset_server`: The asset server
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_proto::prelude::{ProtoData, Prototype, Prototypical};
    ///
    /// fn setup_system(mut commands: Commands, data: Res<ProtoData>, asset_server: &Res<AssetServer>) {
    ///     let proto: Prototype = serde_yaml::from_str(r#"
    ///     name: My Prototype
    ///     components:
    ///       - type: SomeMarkerComponent
    ///       - type: SomeComponent
    ///         value:
    ///           - speed: 10.0
    ///     "#).unwrap();
    ///
    ///     let entity = proto.spawn(&mut commands, &data, &asset_server).id();
    ///
    ///     // ...
    /// }
    ///
    /// ```
    fn spawn<'w, 's, 'a, 'p>(
        &'p self,
        commands: &'a mut Commands<'w, 's>,
        data: &Res<ProtoData>,
        asset_server: &Res<AssetServer>,
    ) -> EntityCommands<'w, 's, 'a> {
        let entity = commands.spawn();
        self.insert(entity, data, asset_server)
    }

    /// Inserts this prototype's component structure to the given entity.
    ///
    /// __Note:__ This _will_ override existing components of the same type.
    ///
    /// # Arguments
    ///
    /// * `entity`: The `EntityCommands` for a given entity
    /// * `data`: The prototype data in this world
    /// * `asset_server`: The asset server
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_proto::prelude::{ProtoData, Prototype, Prototypical};
    ///
    /// #[derive(Component)]
    /// struct Player(pub Entity);
    ///
    /// fn setup_system(mut commands: Commands, data: Res<ProtoData>, asset_server: &Res<AssetServer>, player: Query<&Player>) {
    ///     let proto: Prototype = serde_yaml::from_str(r#"
    ///     name: My Prototype
    ///     components:
    ///       - type: SomeMarkerComponent
    ///       - type: SomeComponent
    ///         value:
    ///           - speed: 10.0
    ///     "#).unwrap();
    ///
    ///     // Get the EntityCommands for the player entity
    ///     let entity = commands.entity(player.single().0);
    ///
    ///     // Insert the new components
    ///     let entity = proto.insert(entity, &data, &asset_server).id();
    ///
    ///     // ...
    /// }
    ///
    /// ```
    fn insert<'w, 's, 'a, 'p>(
        &'p self,
        entity: EntityCommands<'w, 's, 'a>,
        data: &Res<ProtoData>,
        asset_server: &Res<AssetServer>,
    ) -> EntityCommands<'w, 's, 'a> {
        let mut proto_commands = self.create_commands(entity, data);

        spawn_internal(
            self.name(),
            self.templates(),
            self.iter_components(),
            &mut proto_commands,
            data,
            asset_server,
            &mut IndexSet::default(),
        );

        proto_commands.into()
    }
}

/// Internal method used for recursing up the template hierarchy and spawning components
/// from the top to the bottom
fn spawn_internal<'a>(
    name: &'a str,
    templates: Option<&TemplateList>,
    components: Iter<'a, Box<dyn ProtoComponent>>,
    proto_commands: &mut ProtoCommands,
    data: &'a Res<ProtoData>,
    asset_server: &Res<AssetServer>,
    traversed: &mut IndexSet<&'a str>,
) {
    // We insert first on the off chance that someone made a prototype its own template...
    traversed.insert(name);

    if let Some(templates) = templates {
        for template in templates.iter_inheritance_order() {
            if traversed.contains(template.as_str()) {
                // ! === Found Circular Dependency === ! //
                handle_cycle!(
                    template,
                    traversed,
                    "For now, the rest of the spawn has been skipped."
                );

                continue;
            }

            // === Spawn Template === //
            if let Some(parent) = data.get_prototype(template) {
                spawn_internal(
                    parent.name(),
                    parent.templates(),
                    parent.iter_components(),
                    proto_commands,
                    data,
                    asset_server,
                    traversed,
                );
            }
        }
    }

    // === Spawn Self === //
    for component in components {
        component.insert_self(proto_commands, asset_server);
    }
}

/// The default prototype object, providing the basics for the prototype system.
#[derive(Debug, PartialEq)]
pub struct Prototype {
    /// The name of this prototype.
    pub name: String,
    /// The names of this prototype's templates (if any).
    ///
    /// See [`TemplateListDeserializer`](crate::serde::TemplateListDeserializer) to
    /// find out how these names are deserialized.
    pub templates: TemplateList,
    /// The components belonging to this prototype.
    pub components: ComponentList,
}

impl Prototypical for Prototype {
    fn name(&self) -> &str {
        &self.name
    }

    fn templates(&self) -> Option<&TemplateList> {
        Some(&self.templates)
    }

    fn iter_components(&self) -> Iter<'_, Box<dyn ProtoComponent>> {
        self.components.iter()
    }

    fn create_commands<'w, 's, 'a, 'p>(
        &'p self,
        entity: EntityCommands<'w, 's, 'a>,
        data: &'p Res<ProtoData>,
    ) -> ProtoCommands<'w, 's, 'a, 'p> {
        data.get_commands(self, entity)
    }
}
