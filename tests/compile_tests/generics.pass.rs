use bevy::asset::Asset;
use bevy::prelude::*;
use bevy_proto::prelude::*;
use std::borrow::Cow;

#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
struct TupleStructWithGenerics<'a: 'static, T: Asset, const N: usize>(
    #[schematic(asset)] Handle<T>,
    Cow<'a, str>,
    [i32; N],
);

#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
struct StructWithGenerics<'a: 'static, T: Asset, const N: usize> {
    #[schematic(asset)]
    asset: Handle<T>,
    string: Cow<'a, str>,
    array: [i32; N],
}

#[derive(Component, Schematic, Reflect)]
#[reflect(Schematic)]
enum EnumWithGenerics<'a: 'static, T: Asset, const N: usize> {
    Unit,
    Tuple(#[schematic(asset)] Handle<T>, Cow<'a, str>, [i32; N]),
    Struct {
        #[schematic(asset)]
        asset: Handle<T>,
        string: Cow<'a, str>,
        array: [i32; N],
    },
}

fn main() {}
