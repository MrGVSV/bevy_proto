//! This example covers some more details about the derive macro.
//!
//! > The example itself doesn't actually do anything interesting,
//! > it's just meant to give some more details on the derive macro.
//!
//! For schematics to be useful, they need to be serializable.
//! Unfortunately, there are times when we want our type to contain data that can't
//! effectively be serialized (asset handles, entities, etc.).
//!
//! This is where the [`Schematic::Input`] comes into play.
//! The [`Schematic`] trait takes an `Input` type that we can use as an
//! intermediate value from which we can deserialize and base our schematics on.
//! By default, this is determined to just be `Self`.
//!
//! However, when we apply a schematic field attribute,
//! the derive macro will then generate a new input type to accommodate
//! those fields that require that intermediate step.
//!
//! For more details, check out the documentation on the derive macro.

use bevy::prelude::*;
use bevy::reflect::TypePath;
use std::marker::PhantomData;

use bevy_proto::prelude::*;
use bevy_proto_backend::schematics::FromSchematicInput;

fn main() {
    println!("This example doesn't do anything...");
}

/// This struct will generate its own input type since it has at least one field
/// marked with a `#[schematic]` attribute.
///
/// To see what this generated input type looks like, scroll to the bottom of this file.
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
struct Foo<T: Reflect + TypePath> {
    /// Assets can be loaded by marking any `Handle<T>` field like so:
    #[schematic(asset)]
    lazy_asset: Handle<Image>,
    /// By default, assets are lazy loaded— they don't get loaded until
    /// the schematic is applied to an entity.
    /// To preload the asset as a dependency of the prototype,
    /// we can give it the `preload` argument:
    #[schematic(asset(preload))]
    preloaded_asset: Handle<Image>,
    /// Assets can also be inlined directly into the prototype.
    /// This is useful for rapid prototyping.
    /// We can opt into this ability by using the `inline` argument:
    #[schematic(asset(inline))]
    inlinable_asset: Handle<Mesh>,
    /// An inlined asset will result in a single asset being generated.
    /// However, there may be cases where we want a new asset to be generated
    /// every time we apply the `Schematic`.
    /// To create a new asset every time, we can use the `unique` argument:
    #[schematic(asset(inline, unique))]
    unique_inlinable_asset: Handle<Mesh>,
    /// Entities can also be handled succinctly using an attribute.
    /// To reference any entity within the prototype's hierarchy
    /// (the `EntityTree`), we can use the following attribute:
    #[schematic(entity)]
    entity: Entity,
    /// We can also easily convert to any type `U` from type `T`
    /// where `T` implements `From<U>` by using the `from` attribute:
    #[schematic(from=[f32;3])]
    simple_from: Color,
    /// For more advanced conversions, we can use [`FromSchematicInput`].
    /// This provides a lot more context since it's actually called during
    /// schematic application.
    /// It also uses the `from` attribute:
    #[schematic(from=String)]
    complex_from: EntityGroup,
    /// To pass attributes to the generated input type,
    /// we can use the `attr` argument on both fields and the container itself:
    #[reflect(ignore)]
    #[schematic(attr(reflect(ignore)))]
    _phantom: PhantomData<T>,
}

#[derive(Reflect, Clone)]
struct EntityGroup(Vec<Entity>);

// This implementation allows us to get a group of entities from the world
// that all share the name provided by `String`.
impl FromSchematicInput<String> for EntityGroup {
    fn from_input(input: String, _id: SchematicId, context: &mut SchematicContext) -> Self {
        let world = context.world_mut();
        let mut query = world.query::<(Entity, &Name)>();
        let group = query
            .iter(world)
            .filter(|(_, name)| name.as_str() == input)
            .map(|(entity, _)| entity)
            .collect();

        Self(group)
    }
}

/// We can also use the `from` attribute on our entire container type
/// in order to designate an existing type as the input.
///
/// Keep in mind that because this is defining a custom input type already,
/// we are not able to use the schematic attributes on our fields—
/// that would require that we use the custom input type _and_ generate a new one!
#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
#[schematic(from = String)]
struct Bar(String);

impl<T: ToString> From<T> for Bar {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}

// Below is the generated input type for `Foo`.
//
// As you can see, it's contained within an anonymous const block so that we don't
// leak this type into the current scope.
//
// If it's desirable to have access to the generated type,
// you can use `vis` argument in the `#[schematic(input)]` container attribute:
// `#[schematic( input( vis = pub(crate) ) )]`.
// If we do this, we can even give it a custom name like:
// `#[schematic( input( name = MyCustomInput ) )]`.
//
// -----------------------------------------------------------------------
// const _: () = {
//     #[derive(::bevy::reflect::Reflect)]
//     struct FooInput<T: Reflect + TypePath> {
//         lazy_asset: bevy_proto::backend::assets::ProtoAsset<Image>,
//         preloaded_asset: bevy_proto::backend::assets::ProtoAsset<Image>,
//         inlinable_asset: bevy_proto::backend::assets::InlinableProtoAsset<Mesh>,
//         unique_inlinable_asset: bevy_proto::backend::assets::InlinableProtoAsset<Mesh>,
//         entity: bevy_proto::backend::tree::EntityAccess,
//         simple_from: [f32; 3],
//         complex_from: String,
//         #[reflect(ignore)] _phantom: PhantomData<T>,
//         #[reflect(ignore)] __phantom_ty__: ::core::marker::PhantomData<fn() -> ( T )>,
//     }
//     impl<T: Reflect + TypePath> bevy_proto::backend::schematics::FromSchematicInput<FooInput<T>> for Foo<T> {
//         fn from_input(__input__: FooInput<T>, __id__: bevy_proto::backend::schematics::SchematicId, __context__: &mut bevy_proto::backend::schematics::SchematicContext) -> Self {
//             Self {
//                 lazy_asset: bevy_proto::backend::schematics::FromSchematicInput::from_input(__input__.lazy_asset, __id__.next(315625636512882385058417582115465912573u128), __context__),
//                 preloaded_asset: bevy_proto::backend::schematics::FromSchematicInput::from_input(__input__.preloaded_asset, __id__.next(274759083279124148346925450125939914065u128), __context__),
//                 inlinable_asset: bevy_proto::backend::schematics::FromSchematicInput::from_input(__input__.inlinable_asset, __id__.next(256227455035346537265969956879862545554u128), __context__),
//                 unique_inlinable_asset: bevy_proto::backend::schematics::FromSchematicInput::from_input(__input__.unique_inlinable_asset, __id__.next(::bevy::utils::Uuid::new_v4()), __context__),
//                 entity: bevy_proto::backend::schematics::FromSchematicInput::from_input(__input__.entity, __id__.next(12442042730015606647024197521135919140u128), __context__),
//                 simple_from: bevy_proto::backend::schematics::FromSchematicInput::from_input(__input__.simple_from, __id__.next(255894236492372814614208583312628750442u128), __context__),
//                 complex_from: bevy_proto::backend::schematics::FromSchematicInput::from_input(__input__.complex_from, __id__.next(158834411646179188492737822212966092653u128), __context__),
//                 _phantom: __input__._phantom,
//             }
//         }
//     }
//     impl<T: Reflect + TypePath> bevy_proto::backend::schematics::Schematic for Foo<T> {
//         type Input = FooInput<T>;
//         fn apply(__input__: &Self::Input, __id__: bevy_proto::backend::schematics::SchematicId, __context__: &mut bevy_proto::backend::schematics::SchematicContext) {
//             let __input__ = <Self::Input as ::bevy::reflect::FromReflect>::from_reflect(&*::bevy::reflect::Reflect::clone_value(__input__)).unwrap_or_else(|| { panic!("{} should have a functioning `FromReflect` impl", std::any::type_name::<Self::Input>()) });
//             let __input__ = <Self as bevy_proto::backend::schematics::FromSchematicInput<Self::Input>>::from_input(__input__, __id__.next(1698037882055909450320189415164410880u128), __context__);
//             __context__.entity_mut().unwrap_or_else(|| panic!("schematic `{}` expected entity", std::any::type_name::<Self>())).insert(__input__);
//         }
//         fn remove(__input__: &Self::Input, __id__: bevy_proto::backend::schematics::SchematicId, __context__: &mut bevy_proto::backend::schematics::SchematicContext) { __context__.entity_mut().unwrap_or_else(|| panic!("schematic `{}` expected entity", std::any::type_name::<Self>())).remove::<Self>(); }
//         fn preload_dependencies(__input__: &mut Self::Input, __id__: bevy_proto::backend::schematics::SchematicId, __context__: &mut bevy_proto::backend::deps::DependenciesBuilder) {
//             __input__.preloaded_asset = {
//                 let __temp__ = <bevy_proto::backend::assets::ProtoAsset<Image> as ::bevy::reflect::FromReflect>::from_reflect(&*::bevy::reflect::Reflect::clone_value(&__input__.preloaded_asset)).unwrap_or_else(|| { panic!("{} should have a functioning `FromReflect` impl", ::std::any::type_name::<Image>()) });
//                 bevy_proto::backend::assets::ProtoAsset::Handle(bevy_proto::backend::schematics::FromSchematicPreloadInput::from_preload_input(__temp__, __id__.next(143686424131491485342302652144513517898u128), __context__))
//             };
//         }
//     }
// };
// -----------------------------------------------------------------------
