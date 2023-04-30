use std::borrow::Cow;

use bevy::asset::{Assets, Handle};

use crate::children::{Children, MergeKey, PrototypicalChild};
use crate::cycles::{Cycle, CycleChecker, CycleNode, CycleResponse};
use crate::proto::{Config, ProtoError, Prototypical};
use crate::registration::ProtoRegistry;
use crate::templates::Templates;
use crate::tree::ProtoTree;

/// Cache object used to create [`ProtoTree`] objects.
pub(crate) struct ProtoTreeBuilder<'a, T: Prototypical> {
    registry: &'a mut ProtoRegistry<T>,
    prototypes: &'a Assets<T>,
    config: &'a T::Config,
}

impl<'a, T: Prototypical> ProtoTreeBuilder<'a, T> {
    pub fn new(
        registry: &'a mut ProtoRegistry<T>,
        prototypes: &'a Assets<T>,
        config: &'a T::Config,
    ) -> Self {
        Self {
            registry,
            prototypes,
            config,
        }
    }

    /// Create a [`ProtoTree`] for the [prototype] with the given handle.
    ///
    /// [prototype]: Prototypical
    pub fn build(&mut self, handle: &Handle<T>) -> Result<(), ProtoError> {
        let prototype = self.get_prototype(handle)?;

        let mut checker = CycleChecker::<T>::new(Cow::Borrowed(prototype.id()));
        self.recursive_build(prototype, handle.clone_weak(), None, &mut checker)?;

        Ok(())
    }

    /// Recursively build the [tree] for the given [prototype] and handle.
    ///
    /// [tree]: ProtoTree
    /// [prototype]: Prototypical
    fn recursive_build(
        &mut self,
        prototype: &'a T,
        handle: Handle<T>,
        merge_key: Option<MergeKey<T>>,
        checker: &mut CycleChecker<'a, T>,
    ) -> Result<Option<ProtoTree<T>>, ProtoError> {
        let handle_id = handle.id();
        if let Some(tree) = self.registry.get_tree(&handle).cloned() {
            // Tree already cached -> return that value
            return Ok(Some(tree));
        }

        let mut tree = ProtoTree::new(handle, merge_key, prototype);

        if let Some(children) = prototype.children() {
            self.recurse_children(children, &mut tree, checker)?;
        }
        if let Some(templates) = prototype.templates() {
            self.recurse_templates(templates, &mut tree, checker)?;
        }

        if !tree.requires_entity() && !tree.children().is_empty() {
            return Err(ProtoError::RequiresEntity {
                id: prototype.id().to_string(),
            });
        }

        self.registry.insert_tree(handle_id, tree);
        Ok(self.registry.get_tree(handle_id).cloned())
    }

    fn recurse_templates(
        &mut self,
        templates: &'a Templates,
        tree: &mut ProtoTree<T>,
        checker: &mut CycleChecker<'a, T>,
    ) -> Result<(), ProtoError> {
        for (_, template_handle) in templates.iter() {
            let template_handle = template_handle.typed_weak();
            let template_prototype = self.get_prototype(&template_handle)?;

            self.registry
                .add_dependent(template_handle.id(), tree.handle());

            let node = CycleNode::Template {
                id: Cow::Borrowed(template_prototype.id()),
            };
            if let Err(cycle) = checker.try_push(node) {
                if self.handle_cycle(cycle)? {
                    checker.pop();
                    continue;
                } else {
                    checker.pop();
                    return Ok(());
                }
            }

            if let Some(template_tree) =
                self.recursive_build(template_prototype, template_handle, None, checker)?
            {
                tree.inherit(template_tree.clone());
            }

            checker.pop();
        }
        Ok(())
    }

    fn recurse_children(
        &mut self,
        children: &'a Children<T>,
        tree: &mut ProtoTree<T>,
        checker: &mut CycleChecker<'a, T>,
    ) -> Result<(), ProtoError> {
        for child in children.iter() {
            let child_handle = child.handle();
            let child_prototype = self.get_prototype(child_handle)?;

            self.registry
                .add_dependent(child_handle.id(), tree.handle());

            let node = CycleNode::Child {
                id: Cow::Borrowed(child_prototype.id()),
            };
            if let Err(cycle) = checker.try_push(node) {
                if self.handle_cycle(cycle)? {
                    checker.pop();
                    continue;
                } else {
                    checker.pop();
                    return Ok(());
                }
            }

            let child_handle = child_handle.clone_weak();
            let merge_key = child.merge_key().cloned();
            if let Some(child_tree) =
                self.recursive_build(child_prototype, child_handle, merge_key, checker)?
            {
                tree.append_child(child_tree.clone());
            }

            checker.pop();
        }
        Ok(())
    }

    fn get_prototype(&self, handle: &Handle<T>) -> Result<&'a T, ProtoError> {
        self.prototypes
            .get(handle)
            .ok_or_else(|| ProtoError::DoesNotExist(handle.clone_weak_untyped()))
    }

    /// Handle a cycle using the configured [`CycleResponse`].
    ///
    /// Returns `Ok(true)` if the recursion should be skipped,
    /// otherwise the operation should be canceled.
    fn handle_cycle(&self, cycle: Cycle<T>) -> Result<bool, ProtoError> {
        match self.config.on_cycle(&cycle) {
            CycleResponse::Ignore => Ok(true),
            CycleResponse::Cancel => Err(ProtoError::ContainsCycle {
                cycle: cycle.to_string(),
            }),
            CycleResponse::Panic => panic!(
                "{}",
                ProtoError::ContainsCycle {
                    cycle: cycle.to_string(),
                }
            ),
        }
    }
}
