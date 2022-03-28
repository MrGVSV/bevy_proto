use crate::errors::ProtoSpawnError;
use crate::Prototypical;
use bevy::asset::{Asset, AssetPath, Assets, HandleId};
use bevy::log::warn;
use indexmap::IndexSet;
use std::hash::{Hash, Hasher};
use std::iter::Rev;
use std::ops::{Add, Deref};
use std::path::PathBuf;
use std::slice::Iter;

/// A list of templates a [prototypical] struct will inherit from.
///
/// A _template_ is just the asset path of another prototype. This template will be applied
/// along with the actual prototype. In this way, prototypes can "inherit" from
/// other prototypes, reducing duplication and improving configuration.
///
/// Templates are listed in reverse order of inheritance, where templates that are
/// listed last may be overridden by templates listed first. For example, a list
/// of templates like `["IFoo", "IBar", "IBaz"]` means that [prototypical] information
/// generated under `"IBaz"` may be overridden by `"IBar"` or `"IFoo"`. The reason for this
/// is to position more specific or relevant templates first so that it's more visible at a glance.
///
/// By default, template paths take on the extension of the prototype itself unless given a dedicated
/// extension. This means that a prototype with the filepath `prototypes/Foo.prototype.yaml` will add
/// `.prototype.yaml` to its templates. So the templates `["Bar", "../Baz.json"]` are expanded to
/// `["./Bar.prototype.yaml", "./../Baz.json"]`.
///
/// [prototypical]: crate::prelude::Prototypical
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TemplateList {
    asset_paths: Vec<PathBuf>,
}

impl TemplateList {
    /// Create a new [`TemplateList`].
    pub fn new<P: Into<PathBuf>, I: IntoIterator<Item = P>>(paths: I) -> Self {
        Self {
            asset_paths: paths.into_iter().map(|p| p.into()).collect(),
        }
    }

    /// Gets an iterator over the templates in their defined order.
    ///
    /// This is not recursive.
    pub fn iter_defined_order(&self) -> Iter<'_, PathBuf> {
        self.asset_paths.iter()
    }

    /// Gets an iterator over the templates in order of inheritance.
    ///
    /// This is not recursive.
    pub fn iter_inheritance_order(&self) -> Rev<Iter<'_, PathBuf>> {
        self.asset_paths.iter().rev()
    }

    /// Returns true if this list is empty.
    pub fn is_empty(&self) -> bool {
        self.asset_paths.is_empty()
    }

    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        self.asset_paths.len()
    }
}

/// A node used for template traversal
struct TemplateNode<'a, T: Prototypical + Asset>(&'a T);

impl<'a, T: Prototypical + Asset> Copy for TemplateNode<'a, T> {}
impl<'a, T: Prototypical + Asset> Clone for TemplateNode<'a, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<'a, T: Prototypical + Asset> Deref for TemplateNode<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
impl<'a, T: Prototypical + Asset> Eq for TemplateNode<'a, T> {}
impl<'a, T: Prototypical + Asset> PartialEq for TemplateNode<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.name() == other.0.name()
    }
}
impl<'a, T: Prototypical + Asset> Hash for TemplateNode<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name().hash(state)
    }
}

/// Utility function for iterating over a prototype's templates (if any) and performing some action.
///
/// The function is applied to each template in their _inherited_ order, bottom-up. This means that for
/// a template tree of:
///
/// ```text
/// A: B, C
/// B: D, E
/// D: C
/// ```
///
/// We will apply the given function in the following order:
///
/// 1. `C` (from `A: C`)
/// 2. `E` (from `A: B` -> `B: E`)
/// 3. `C` (from `A: B` -> `B: D` -> `D: C`)
/// 4. `D` (from `A: B` -> `B: D`)
/// 5. `B` (from `A: B`)
/// 6. `A`
///
/// Wait, by doing this we applied `C` twice! This is actually somewhat intended behavior. We can't
/// just prune all usages of `C` after the first one because higher priority templates need to override
/// lower priority ones. If we skipped the second `C`, we'd be breaking that rule.
///
/// If a cycle is found it will be skipped (e.g., `A -> B -> A` becomes `A -> B`). If multiple circular
/// dependencies are found, they will be combined into a single error.
///
/// # Panics
///
/// This should panic if a circular dependency is found and the `no_cycles` feature is enabled.
pub(crate) fn apply_templates<'a, T: Prototypical + Asset>(
    proto: &'a T,
    assets: &'a Assets<T>,
    func: &mut dyn FnMut(&'a T),
) -> Result<(), ProtoSpawnError> {
    fn recurse_templates<'a, T: Prototypical + Asset>(
        assets: &'a Assets<T>,
        node: TemplateNode<'a, T>,
        stack: &mut IndexSet<TemplateNode<'a, T>>,
        func: &mut dyn FnMut(&'a T),
    ) -> Result<(), ProtoSpawnError> {
        if stack.contains(&node) {
            let tree = make_cycle_tree(node.name(), &stack);
            return Err(ProtoSpawnError::CircularDependency {
                tree,
                msg: Default::default(),
            });
        }

        stack.insert(node);

        let mut retval = Ok(());
        if let Some(templates) = node.templates() {
            for template_path in templates.iter_inheritance_order() {
                let asset_path = AssetPath::from(template_path.as_path());
                let template_handle: HandleId = asset_path.into();
                let template = assets.get(template_handle).unwrap();
                let result = recurse_templates(assets, TemplateNode(template), stack, func);
                match result {
                    Err(err) if matches!(err, ProtoSpawnError::CircularDependency { .. }) => {
                        match err {
                            ProtoSpawnError::CircularDependency {
                                tree: ref new_tree, ..
                            } => {
                                if let Err(ProtoSpawnError::CircularDependency {
                                    ref mut tree,
                                    ..
                                }) = retval
                                {
                                    tree.push_str(" | ");
                                    tree.push_str(new_tree);
                                } else {
                                    retval = Err(err);
                                }

                                #[cfg(feature = "no_cycles")]
                                panic!("{}", retval.err().unwrap());
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        // Any other errors should terminate recursion
                        return Err(err);
                    }
                    _ => {}
                }
            }
        }

        let node = stack.pop().unwrap();
        func(node.0);

        retval
    }

    let root = TemplateNode(proto);
    let mut stack = IndexSet::new();
    let result = recurse_templates(assets, root, &mut stack, func);

    match result {
        Err(err) if matches!(err, ProtoSpawnError::CircularDependency { .. }) => {
            #[cfg(feature = "no_cycles")]
            panic!("{}", err);
            #[cfg(feature = "analysis")]
            warn!("{}", err);

            Err(err)
        }
        res => res,
    }
}

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
/// let mut traversed = IndexSet::new();
/// traversed.insert("A");
/// traversed.insert("B");
/// traversed.insert("C");
///
/// let tree = make_cycle_tree("B", &traversed);
/// assert_eq!("'A' -> 'B' -> 'C' -> 'B'", tree);
/// ```
fn make_cycle_tree<T: Prototypical + Asset>(
    template_name: &str,
    traversed: &IndexSet<TemplateNode<T>>,
) -> String {
    traversed
        .iter()
        .map(|proto| format!("'{}' -> ", proto.name()))
        .collect::<String>()
        .add(&format!("'{}'", template_name))
}
