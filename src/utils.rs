use crate::templates::apply_templates;
use crate::Prototypical;
use bevy::asset::{Asset, Assets};

/// Performs some analysis on the current set of loaded prototypical assets.
pub(crate) fn analyze_deps<'a, T: Prototypical + Asset>(proto: &'a T, assets: &'a Assets<T>) {
    apply_templates(proto, assets, &mut |_| {}).ok();
}
