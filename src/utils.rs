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
/// ```
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
