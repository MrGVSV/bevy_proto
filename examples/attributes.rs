use bevy::prelude::*;
use bevy_proto::prelude::*;
use serde::{Deserialize, Serialize};

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
/// our `ProtoComponent` had turns it into our "ActualComponent".
///
/// Note: We still do need to derive/impl `Clone`, `Serialize`, and `Deserialize` traits.
#[derive(Clone, Serialize, Deserialize, ProtoComponent)]
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
fn create_emoji<T: AsEmoji + ProtoComponent>(
    component: &T,
    commands: &mut ProtoCommands,
    _asset_server: &AssetServer,
) {
    commands.insert(component.as_emoji());
}

/// Then we can use the `#[proto_comp(with = "my_function")]` attribute. This works exactly
/// like [`ProtoComponent::insert_self`], but allows you to use an extracted version of that function.
#[derive(Clone, Serialize, Deserialize, ProtoComponent)]
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
#[derive(Clone, Serialize, Deserialize, ProtoComponent)]
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

fn spawn_emojis(mut commands: Commands, data: Res<ProtoData>, asset_server: Res<AssetServer>) {
    let proto = data.get_prototype("Happy").expect("Should exist!");
    proto.spawn(&mut commands, &data, &asset_server);
    let proto = data.get_prototype("Sad").expect("Should exist!");
    proto.spawn(&mut commands, &data, &asset_server);
    let proto = data.get_prototype("Silly").expect("Should exist!");
    proto.spawn(&mut commands, &data, &asset_server);
    let proto = data.get_prototype("Angry").expect("Should exist!");
    proto.spawn(&mut commands, &data, &asset_server);
}

fn print_emojies(query: Query<&Emoji, Added<Emoji>>) {
    for emoji in query.iter() {
        println!("{}", emoji.0);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ProtoPlugin::default())
        .add_startup_system(spawn_emojis)
        .add_system(print_emojies)
        .run();
}
