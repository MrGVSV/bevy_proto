use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy::reflect::FromReflect;
use bevy_proto::prelude::*;
use serde::Deserialize;

/// Not every `ProtoComponent` needs to itself be a `Component` (or vice-versa)
///
/// Let's pretend we don't have access to this struct. Maybe it's from another library or
/// its a Bevy internal component. Maybe it's has a generic type that can't be deserialized.
///
/// Whatever, the case, how can we include this component in our templates? We _could_ manually
/// implement `ProtoComponent` like in the `bundles` example. However, there are other means of
/// achieving this that might be a little easier or utilize other code you might already have.
#[derive(Component)]
struct Emoji(String);

/// The simplest method to spawn a different component is to utilize the `From<T>` trait.
///
/// If our `ProtoComponent` implements `From<T>` for the "ActualComponent", we can use the
/// `#[proto_comp(into = "ActualComponent")]` attribute. This attribute essentially just clones
/// our `ProtoComponent` and turns it into our "ActualComponent".
///
/// Note: We still do need to derive/impl the `Reflect`, `FromReflect`, and `Clone` traits.
#[derive(Reflect, FromReflect, ProtoComponent, Clone)]
#[reflect(ProtoComponent)]
#[proto_comp(into = "Emoji")]
struct EmojiDef {
    emoji: String,
}

/// Make sure you impl `From<T>`!
impl From<EmojiDef> for Emoji {
    fn from(def: EmojiDef) -> Self {
        Self(def.emoji)
    }
}

/// Alternatively, you might want or have a function that performs the spawn logic that should
/// be shared so that it may be used in other places or for other components.
///
/// Say we have a trait that allows its implementors to return an `Emoji` struct.
trait AsEmoji {
    fn as_emoji(&self) -> Emoji;
}

/// We can create a function that takes any `ProtoComponent` that implements `AsEmoji` and inserts
/// an `Emoji` component.
fn create_emoji<T: AsEmoji + ProtoComponent>(component: &T, entity: &mut EntityMut) {
    entity.insert(component.as_emoji());
}

/// Then we can use the `#[proto_comp(with = "my_function")]` attribute. This works exactly
/// like [`ProtoComponent::insert_self`], but allows you to use an extracted version of that function.
///
/// Note: Enums are still not fully supported and require that you reflect them as a value type. This
/// is why we use `#[reflect_value(ProtoComponent, Deserialize)]`.
#[derive(Reflect, FromReflect, ProtoComponent, Deserialize, Copy, Clone)]
#[reflect_value(ProtoComponent, Deserialize)]
#[proto_comp(with = "create_emoji")]
enum Mood {
    Normal,
    Silly,
}
impl AsEmoji for Mood {
    fn as_emoji(&self) -> Emoji {
        match self {
            Self::Normal => Emoji(String::from("😶")),
            Self::Silly => Emoji(String::from("🤪")),
        }
    }
}

/// Notice that we only had to define the function once even though we're using it across multiple
/// `ProtoComponent` structs.
#[derive(Reflect, FromReflect, ProtoComponent, Deserialize, Copy, Clone)]
#[reflect_value(ProtoComponent, Deserialize)]
#[proto_comp(with = "create_emoji")]
enum Face {
    Normal,
    Frowning,
}
impl AsEmoji for Face {
    fn as_emoji(&self) -> Emoji {
        match self {
            Self::Normal => Emoji(String::from("😶")),
            Self::Frowning => Emoji(String::from("😠")),
        }
    }
}

fn load_prototypes(asset_server: Res<AssetServer>, mut manager: ProtoManager) {
    let handles = asset_server.load_folder("prototypes/attributes").unwrap();
    manager.add_multiple_untyped(handles);
}

fn spawn_emojis(mut manager: ProtoManager, mut has_ran: Local<bool>) {
    if *has_ran {
        return;
    }

    if !manager.all_loaded() {
        return;
    }

    manager.spawn("Happy");
    manager.spawn("Sad");
    manager.spawn("Silly");
    manager.spawn("Angry");

    *has_ran = true;
}

fn print_emojis(query: Query<&Emoji, Added<Emoji>>) {
    for emoji in query.iter() {
        println!("{}", emoji.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ProtoPlugin::<Prototype>::default())
        // !!! Make sure you register your types !!! //
        .register_type::<EmojiDef>()
        .register_type::<Mood>()
        .register_type::<Face>()
        .add_startup_system(load_prototypes)
        .add_system(spawn_emojis)
        .add_system(print_emojis)
        .run();
}
