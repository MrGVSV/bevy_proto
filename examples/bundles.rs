use bevy_proto::HandlePath;

#[derive(Serialize, Deserialize)]
struct SpriteBundleDef {
	pub texture_path: HandlePath,
	pub other_path: HandlePath,
}

fn main() {}
