#![allow(unused_doc_comments)]

use bevy::prelude::*;
use bevy::reflect::FromReflect;
use std::sync::Arc;

use bevy_proto::prelude::*;

#[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
struct ComponentA;

#[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
struct ComponentB;

#[derive(Reflect, FromReflect, Component, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
struct ComponentC;

fn load_prototypes(asset_server: Res<AssetServer>, mut manager: ProtoManager<Prototype>) {
    let handles = asset_server.load_folder("prototypes/config").unwrap();
    manager.add_multiple_untyped(handles);
}

fn main() {
    // The config lets us control aspects of prototype loading and deserialization.
    let mut config = ProtoConfig::default();

    // We can whitelist components. Only these components are allowed.
    config.whitelist::<ComponentA>().whitelist::<ComponentB>();

    // We can blacklist components. All components are allowed, except these ones.
    // Note: You can have either a `whitelist` or a `blacklist`— not both.
    config.blacklist::<ComponentB>();

    // We can set a callback for when a prototype is registered
    config.on_register(Some(Arc::new(|proto| {
        println!("Registered Prototype: {}", proto.name());
        println!("Components:");
        for component in proto.components() {
            println!("  - {}", component.name());
        }
        println!("---");

        // Return `Ok(())` to allow this prototype to be registered
        Ok(())
    })));

    // And we can set a callback for when a component is loaded
    config.on_register_component(Some(Arc::new(|component| {
        println!("Registered Component: {}", component.name());

        // Return `true` to allow this component to be loaded.
        // For fun, we'll prevent `ComponentC` from being loaded.
        if component.type_name() == std::any::type_name::<ComponentC>() {
            println!("Blocked {}", component.type_name());
            false
        } else {
            true
        }
    })));

    App::new()
        .add_plugins(DefaultPlugins)
        // Add our plugin with our config
        .add_plugin(ProtoPlugin::<Prototype>::with_config(config))
        // !!! Make sure you register your types !!! //
        .register_type::<ComponentA>()
        .register_type::<ComponentB>()
        .register_type::<ComponentC>()
        .add_startup_system(load_prototypes)
        .run();
}
