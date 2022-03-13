// use std::any::TypeId;
// use std::ffi::OsStr;
// use std::ops::{Deref, DerefMut};
//
// use crate::{components::ProtoComponent, prototypical::Prototypical, utils::handle_cycle};
// use bevy::asset::{Asset, HandleUntyped};
// use bevy::ecs::prelude::World;
// use bevy::ecs::system::EntityCommands;
// use bevy::prelude::{FromWorld, Handle};
// use bevy::reflect::Uuid;
// use bevy::utils::HashMap;
// use dyn_clone::DynClone;
// use indexmap::IndexSet;
// use serde::{Deserialize, Serialize};
//
// /// A String newtype for a handle's asset path
// #[derive(Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
// pub struct HandlePath(pub String);
//
// impl Deref for HandlePath {
//     type Target = String;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// type UuidHandleMap = HashMap<Uuid, HandleUntyped>;
//
// /// A resource containing data for all prototypes that need data stored
// pub struct ProtoData {
//     /// Maps Prototype Name -> Component Type -> HandlePath -> Asset Type -> HandleUntyped
//     handles: HashMap<
//         String, // Prototype Name
//         HashMap<
//             TypeId, // Component Type
//             HashMap<
//                 String, // Handle Path
//                 UuidHandleMap,
//             >,
//         >,
//     >,
//     prototypes: HashMap<String, Box<dyn Prototypical>>,
// }
//
// impl ProtoData {
//     /// Creates a new, empty instance of [`ProtoData`].
//     pub fn empty() -> Self {
//         Self {
//             handles: HashMap::default(),
//             prototypes: HashMap::default(),
//         }
//     }
//
//     /// Get a loaded prototype with the given name
//     ///
//     /// # Arguments
//     ///
//     /// * `name`: The name of the prototype
//     ///
//     /// returns: Option<&Prototype>
//     pub fn get_prototype(&self, name: &str) -> Option<&dyn Prototypical> {
//         self.prototypes.get(name).map(|b| b.as_ref())
//     }
//
//     /// Store a handle
//     ///
//     /// # Arguments
//     ///
//     /// * `protoytpe`: The Prototype this handle belongs to
//     /// * `component`: The ProtoComponent this handle belongs to
//     /// * `path`: The handle's path
//     /// * `handle`: The handle
//     ///
//     /// returns: ()
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use bevy::prelude::*;
//     /// use bevy_proto::prelude::*;
//     /// use serde::{Deserialize, Serialize};
//     ///
//     /// #[derive(Clone, Deserialize, Serialize, Component, ProtoComponent)]
//     /// struct MyComponent {
//     ///     texture_path: HandlePath
//     /// }
//     ///
//     /// fn some_loader(asset_server: Res<AssetServer>, mut data: ResMut<ProtoData>) {
//     ///     let comp = MyComponent {
//     ///         texture_path: HandlePath(String::from("path/to/texture.png"))
//     ///     };
//     ///     let proto = Prototype {
//     ///         name: String::from("My Prototype"),
//     ///         templates: Vec::default(),
//     ///         components: vec![Box::new(comp.clone())]
//     ///     };
//     ///
//     ///     let handle: Handle<Image> = asset_server.load(comp.texture_path.0.as_str());
//     ///
//     ///     data.insert_handle(&proto, &comp, &comp.texture_path, handle);
//     /// }
//     /// ```
//     pub fn insert_handle<T: Asset>(
//         &mut self,
//         protoytpe: &dyn Prototypical,
//         component: &dyn ProtoComponent,
//         path: &HandlePath,
//         handle: Handle<T>,
//     ) {
//         let proto_map = self
//             .handles
//             .entry(protoytpe.name().to_string())
//             .or_insert_with(HashMap::default);
//         let comp_map = proto_map
//             .entry(component.type_id())
//             .or_insert_with(HashMap::default);
//         let path_map = comp_map
//             .entry(path.to_string())
//             .or_insert_with(HashMap::default);
//         path_map.insert(T::TYPE_UUID, handle.clone_untyped());
//     }
//
//     /// Get a cloned handle
//     ///
//     /// # Arguments
//     ///
//     /// * `protoytpe`: The Prototype this handle belongs to
//     /// * `component`: The ProtoComponent this handle belongs to
//     /// * `path`: The handle's path
//     ///
//     /// returns: Option<Handle<T>>
//     pub fn get_handle<T: Asset>(
//         &self,
//         protoytpe: &dyn Prototypical,
//         component: &dyn ProtoComponent,
//         path: &HandlePath,
//     ) -> Option<Handle<T>> {
//         let handle = self.get_untyped_handle(protoytpe, component, path, T::TYPE_UUID)?;
//         Some(handle.clone().typed::<T>())
//     }
//
//     /// Get a weakly cloned handle
//     ///
//     /// # Arguments
//     ///
//     /// * `protoytpe`: The Prototype this handle belongs to
//     /// * `component`: The ProtoComponent this handle belongs to
//     /// * `path`: The handle's path
//     ///
//     /// returns: Option<Handle<T>>
//     pub fn get_handle_weak<T: Asset>(
//         &self,
//         protoytpe: &dyn Prototypical,
//         component: &dyn ProtoComponent,
//         path: &HandlePath,
//     ) -> Option<Handle<T>> {
//         let handle = self.get_untyped_handle(protoytpe, component, path, T::TYPE_UUID)?;
//         Some(handle.clone_weak().typed::<T>())
//     }
//
//     /// Get a untyped handle reference
//     ///
//     /// # Arguments
//     ///
//     /// * `protoytpe`: The Prototype this handle belongs to
//     /// * `component`: The ProtoComponent this handle belongs to
//     /// * `path`: The handle's path
//     /// * `asset_type`: The asset type
//     ///
//     /// returns: Option<&HandleUntyped>
//     pub fn get_untyped_handle(
//         &self,
//         protoytpe: &dyn Prototypical,
//         component: &dyn ProtoComponent,
//         path: &HandlePath,
//         asset_type: Uuid,
//     ) -> Option<&HandleUntyped> {
//         let proto_map = self.handles.get(protoytpe.name())?;
//         let comp_map = proto_map.get(&component.type_id())?;
//         let path_map = comp_map.get(path.as_str())?;
//         path_map.get(&asset_type)
//     }
//
//     /// Create a [`ProtoCommands`] object for the given prototype
//     ///
//     /// # Arguments
//     ///
//     /// * `prototype`: The associated prototype
//     /// * `commands`: The [`EntityCommands`]
//     ///
//     /// returns: ProtoCommands
//     pub fn get_commands<'w, 's, 'a, 'p>(
//         &'p self,
//         prototype: &'p dyn Prototypical,
//         commands: EntityCommands<'w, 's, 'a>,
//     ) -> ProtoCommands<'w, 's, 'a, 'p> {
//         ProtoCommands {
//             commands,
//             prototype,
//             data: self,
//         }
//     }
//
//     /// Get an iterator over all prototypes
//     pub fn iter(&self) -> impl Iterator<Item = &Box<dyn Prototypical>> {
//         self.prototypes.values()
//     }
//
//     /// Get a mutable iterator over all prototypes
//     pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Prototypical>> {
//         self.prototypes.values_mut()
//     }
// }
//
// impl FromWorld for ProtoData {
//     fn from_world(world: &mut World) -> Self {
//         let mut myself = Self {
//             handles: Default::default(),
//             prototypes: HashMap::default(),
//         };
//
//         let options = world
//             .get_resource::<ProtoDataOptions>()
//             .expect("Expected options for ProtoData")
//             .clone();
//
//         for directory in options.directories {
//             process_path(
//                 world,
//                 &options.extensions,
//                 options.deserializer.as_ref(),
//                 &mut myself,
//                 &directory,
//                 options.recursive_loading,
//             );
//         }
//
//         #[cfg(feature = "analysis")]
//         analyze_deps(&myself);
//
//         myself
//     }
// }
//
// /// Helper function to populate our ProtoData.
// fn process_path(
//     world: &mut World,
//     extensions: &Option<Vec<&str>>,
//     deserializer: &(dyn ProtoDeserializer + Send + Sync),
//     myself: &mut ProtoData,
//     directory: &str,
//     recursive: bool,
// ) {
//     if let Ok(dir) = std::fs::read_dir(directory) {
//         for file_info in dir {
//             if file_info.is_err() {
//                 continue;
//             }
//             let file_info = file_info.unwrap();
//
//             let path = file_info.path();
//
//             if recursive && path.is_dir() {
//                 process_path(
//                     world,
//                     extensions,
//                     deserializer,
//                     myself,
//                     &path.to_str().unwrap().to_string(),
//                     recursive,
//                 );
//             }
//
//             if let Some(filters) = &extensions {
//                 if let Some(ext) = path.extension().and_then(OsStr::to_str) {
//                     if !filters.iter().any(|filter| filter == &ext) {
//                         continue;
//                     }
//                 }
//             }
//
//             if let Ok(data) = std::fs::read_to_string(path) {
//                 if let Some(proto) = deserializer.deserialize(&data) {
//                     for component in proto.iter_components() {
//                         component.prepare(world, proto.as_ref(), myself);
//                     }
//
//                     myself.prototypes.insert(proto.name().to_string(), proto);
//                 }
//             }
//         }
//     }
// }
//
// /// Performs some analysis on the given [`ProtoData`] resource
// fn analyze_deps(data: &ProtoData) {
//     // === Perform Analysis === //
//     for proto in data.iter() {
//         check_for_cycles(proto.as_ref(), data, &mut IndexSet::default());
//     }
//
//     // === Analysis Functions === //
//     fn check_for_cycles<'a>(
//         proto: &'a dyn Prototypical,
//         data: &'a ProtoData,
//         traversed: &mut IndexSet<&'a str>,
//     ) {
//         traversed.insert(proto.name());
//
//         if let Some(templates) = proto.templates() {
//             for template in templates.iter_inheritance_order() {
//                 if traversed.contains(template.as_str()) {
//                     // ! --- Found Circular Dependency --- ! //
//                     handle_cycle!(template, traversed);
//
//                     continue;
//                 }
//
//                 if let Some(parent) = data.get_prototype(template) {
//                     // --- Check Template --- //
//                     check_for_cycles(parent, data, traversed);
//                 }
//             }
//         }
//     }
// }
//
// /// A wrapper around [`EntityCommands`] and [`ProtoData`] for a specified prototype.
// /// This allows [`ProtoData`] to be accessed with the underlying prototype directly,
// /// and grants direct access to the [`EntityCommands`] that spawned that prototype in.
// pub struct ProtoCommands<'w, 's, 'a, 'p> {
//     /// The associated [`EntityCommands`]
//     commands: EntityCommands<'w, 's, 'a>,
//     /// The associated prototype
//     prototype: &'p dyn Prototypical,
//     /// The [`ProtoData`] resource
//     data: &'p ProtoData,
// }
//
// impl<'w, 's, 'a, 'p> ProtoCommands<'w, 's, 'a, 'p> {
//     /// Get raw access to [`EntityCommands`]
//     pub fn raw_commands(&mut self) -> &mut EntityCommands<'w, 's, 'a> {
//         &mut self.commands
//     }
//     /// Get the associated prototype
//     pub fn protoype(&self) -> &dyn Prototypical {
//         self.prototype
//     }
//
//     /// Get raw access to the underlying [`ProtoData`] resource
//     pub fn raw_data(&self) -> &ProtoData {
//         self.data
//     }
//
//     /// Get a cloned handle
//     ///
//     /// # Arguments
//     ///
//     /// * `component`: The ProtoComponent this handle belongs to
//     /// * `path`: The handle's path
//     ///
//     /// returns: Option<Handle<T>>
//     pub fn get_handle<T: Asset>(
//         &self,
//         component: &dyn ProtoComponent,
//         path: &HandlePath,
//     ) -> Option<Handle<T>> {
//         self.data.get_handle(self.prototype, component, path)
//     }
//
//     /// Get a weakly cloned handle
//     ///
//     /// # Arguments
//     ///
//     /// * `component`: The ProtoComponent this handle belongs to
//     /// * `path`: The handle's path
//     ///
//     /// returns: Option<Handle<T>>
//     pub fn get_handle_weak<T: Asset>(
//         &self,
//         component: &dyn ProtoComponent,
//         path: &HandlePath,
//     ) -> Option<Handle<T>> {
//         self.data.get_handle_weak(self.prototype, component, path)
//     }
//
//     /// Get a untyped handle reference
//     ///
//     /// # Arguments
//     ///
//     /// * `component`: The ProtoComponent this handle belongs to
//     /// * `path`: The handle's path
//     /// * `asset_type`: The asset type
//     ///
//     /// returns: Option<&HandleUntyped>
//     pub fn get_untyped_handle(
//         &self,
//         component: &dyn ProtoComponent,
//         path: &HandlePath,
//         asset_type: Uuid,
//     ) -> Option<&HandleUntyped> {
//         self.data
//             .get_untyped_handle(self.prototype, component, path, asset_type)
//     }
// }
//
// impl<'w, 's, 'a, 'p> From<ProtoCommands<'w, 's, 'a, 'p>> for EntityCommands<'w, 's, 'a> {
//     fn from(cmds: ProtoCommands<'w, 's, 'a, 'p>) -> Self {
//         cmds.commands
//     }
// }
//
// impl<'w, 's, 'a, 'p> Deref for ProtoCommands<'w, 's, 'a, 'p> {
//     type Target = EntityCommands<'w, 's, 'a>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.commands
//     }
// }
//
// impl<'w, 's, 'a, 'p> DerefMut for ProtoCommands<'w, 's, 'a, 'p> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.commands
//     }
// }
//
// /// Defines a method for deserializing a prototype file input.
// pub trait ProtoDeserializer: DynClone {
//     /// Deserializes file input (as a string) into a [`Prototypical`] object
//     ///
//     /// # Arguments
//     ///
//     /// * `data`: The file data as a string
//     ///
//     /// returns: Option<Box<dyn Prototypical, Global>>
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// // The default implementation:
//     /// use bevy_proto::prototype::{Prototype, Prototypical};
//     /// fn example_deserialize(data: &str) -> Option<Box<dyn Prototypical>> {
//     ///     if let Ok(value) = serde_yaml::from_str::<Prototype>(data) {
//     ///         Some(Box::new(value))
//     ///     } else {
//     ///         None
//     ///    }
//     /// }
//     /// ```
//     fn deserialize(&self, data: &str) -> Option<Box<dyn Prototypical>>;
// }
//
// dyn_clone::clone_trait_object!(ProtoDeserializer);
//
// /// Options for controlling how prototype data is handled.
// #[derive(Clone)]
// pub struct ProtoDataOptions {
//     /// Directories containing prototype data.
//     pub directories: Vec<String>,
//     /// Whether to load files recursively from the specified top-level directories.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use bevy_proto::data::ProtoDataOptions;
//     ///
//     /// // Recursively loads all yaml files from the "assets/prototypes" directory.
//     /// let opts = ProtoDataOptions {
//     ///     directories: vec![String::from("assets/prototypes")],
//     ///     recursive_loading: true,
//     ///     extensions: Some(vec!["yaml"]),
//     ///     ..Default::default()
//     /// };
//     /// ```
//     pub recursive_loading: bool,
//     /// A custom deserializer for prototypes.
//     pub deserializer: Box<dyn ProtoDeserializer + Send + Sync>,
//     /// An optional collection of file extensions to filter prototypes. These do __not__
//     /// have a dot ('.') prepended to them.
//     ///
//     /// A value of None allows all files to be read.
//     ///
//     /// # Examples
//     ///
//     /// ```
//     /// use bevy_proto::data::ProtoDataOptions;
//     ///
//     /// let opts = ProtoDataOptions {
//     ///     // Only allow .yaml or .json files
//     ///     extensions: Some(vec!["yaml", "json"]),
//     ///     ..Default::default()
//     /// };
//     /// ```
//     pub extensions: Option<Vec<&'static str>>,
// }
//
// impl Default for ProtoDataOptions {
//     fn default() -> Self {
//         Self {
//             directories: Default::default(),
//             recursive_loading: Default::default(),
//             deserializer: Box::new(DefaultProtoDeserializer),
//             extensions: Default::default(),
//         }
//     }
// }
//
// #[derive(Clone)]
// pub(crate) struct DefaultProtoDeserializer;
//
// impl ProtoDeserializer for DefaultProtoDeserializer {
//     fn deserialize(&self, _data: &str) -> Option<Box<dyn Prototypical>> {
//         // if let Ok(value) = serde_yaml::from_str::<Prototype>(data) {
//         // 	Some(Box::new(value))
//         // } else {
//         // 	None
//         // }
//         None
//     }
// }
