pub const YAML_EXT: &str = "prototype.yaml";
pub const JSON_EXT: &str = "prototype.json";
pub const RON_EXT: &str = "prototype.ron";

pub const ALL_EXT: &[&str] = &[YAML_EXT, JSON_EXT, RON_EXT];

/// Get the extension of the given path.
///
/// This only checks amongst the default extensions.
pub fn get_default_extension(path: &str) -> Option<&str> {
    ALL_EXT
        .iter()
        .find(|ext| path.ends_with(**ext))
        .map(|ext| *ext)
}
