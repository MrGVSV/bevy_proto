use crate::prelude::Prototypical;
use bevy::asset::{Asset, Assets};
use bevy::log::warn;
use indexmap::IndexSet;
use std::ops::Add;

/// Generates a string displaying a dependency cycle
///
/// # Arguments
///
/// * `template_name`: The name of the offending template
/// * `traversed`: The set of traversed templates
///
/// returns: String
///
/// # Examples
///
/// ```ignore
/// use indexmap::IndexSet;
///
/// let mut traversed = IndexSet::<_>::default();
/// traversed.insert("A");
/// traversed.insert("B");
/// traversed.insert("C");
///
/// let tree = make_cycle_tree("B", &traversed);
/// println!("{}", tree);
/// // Output: 'A' -> 'B' -> 'C' -> 'B'
/// ```
pub(crate) fn make_cycle_tree(template_name: &str, traversed: &IndexSet<&str>) -> String {
    traversed
        .iter()
        .map(|n| format!("'{}' -> ", n))
        .collect::<String>()
        .add(&format!("'{}'", template_name))
}

/// Handles a dependency cycle by panicking
///
/// # Arguments
///
/// * `template_name`: The name of the offending template
/// * `traversed`: The set of traversed templates
///
#[cfg(feature = "no_cycles")]
macro_rules! handle_cycle {
	($template_name: ident, $traversed: ident) => {{
		handle_cycle!($template_name, $traversed, "");
	}};
	($template_name: ident, $traversed: ident, $($arg:tt)*) => {{
		let tree = crate::utils::make_cycle_tree($template_name, $traversed);
		panic!(
			"{} {}\n\t{} {}",
			"Found a circular dependency in the following prototypes:",
			tree,
			"Make sure you remove any template that might call itself from the dependency tree!",
			format!("{}", $($arg)*)
		);
	}};
}

/// Handles a dependency cycle by logging a warning
///
/// # Arguments
///
/// * `template_name`: The name of the offending template
/// * `traversed`: The set of traversed templates
///
#[cfg(not(feature = "no_cycles"))]
macro_rules! handle_cycle {
	($template_name: ident, $traversed: ident) => {{
		handle_cycle!($template_name, $traversed, "");
	}};
	($template_name: ident, $traversed: ident, $($arg:tt)*) => {{
		let tree = crate::utils::make_cycle_tree($template_name, $traversed);
		bevy::log::warn!(
			"{} {}\n\t{} {}",
			"Found a circular dependency in the following prototypes:",
			tree,
			"Make sure you remove any template that might call itself from the dependency tree!",
			format!("{}", $($arg)*)
		);
	}};
}

pub(crate) use handle_cycle;

/// Performs some analysis on the current set of loaded prototypical assets.
pub(crate) fn analyze_deps<'a, T: Prototypical + Asset>(proto: &'a T, assets: &'a Assets<T>) {
    for_each_template(proto, assets, &mut |_| {});
}

/// Utility function for iterating over a prototype's templates (if any) and performing some action.
///
/// The templates are iterated over in their _inherited_ order.
pub(crate) fn for_each_template<'a, T: Prototypical + Asset>(
    proto: &'a T,
    assets: &'a Assets<T>,
    func: &mut dyn FnMut(&'a T),
) {
    fn next<'a, T: Prototypical + Asset>(
        proto: &'a T,
        assets: &'a Assets<T>,
        traversed: &mut IndexSet<&'a str>,
        func: &mut dyn FnMut(&'a T),
    ) {
        traversed.insert(proto.name());

        if let Some(templates) = proto.templates() {
            for template in templates.iter_inheritance_order() {
                if let Some(template) = proto.dependencies().get_template(template) {
                    if let Some(template) = assets.get(template.id) {
                        let name = template.name();
                        if traversed.contains(name) {
                            // ! --- Found Circular Dependency --- ! //
                            handle_cycle!(name, traversed);

                            continue;
                        }

                        func(template);
                        next(template, assets, traversed, func);
                    }
                } else {
                    warn!("Could not find template with path: {}", template.display());
                }
            }
        }
    }

    next(proto, assets, &mut IndexSet::new(), func);
}
