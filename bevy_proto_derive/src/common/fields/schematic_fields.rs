use crate::common::data::DeriveType;
use crate::common::fields::{
    AssetInlineArg, AssetPathArg, AssetPreloadArg, AssetTypeArg, AssetUniqueArg, EntityPathArg,
    OptionalArg, SchematicField,
};
use crate::common::input::{InputType, SchematicIo};
use crate::utils::constants::{ASSET_ATTR, ENTITY_ATTR, FROM_ATTR};
use crate::utils::{parse_bool, parse_nested_meta, AttrArg};
use proc_macro2::Span;
use syn::meta::ParseNestedMeta;
use syn::spanned::Spanned;
use syn::{Error, Field, Fields, Type};

/// The collection of fields for a struct or enum.
pub(crate) enum SchematicFields {
    Unit,
    Named(Vec<SchematicField>),
    Unnamed(Vec<SchematicField>),
}

impl SchematicFields {
    /// Creates a [`SchematicFields`] from a [`Fields`].
    ///
    /// The `container_ident` is the name of the user-defined struct or enum that contains the fields
    /// and will be used to generate the input type if necessary.
    pub fn new(
        fields: &Fields,
        io: &mut SchematicIo,
        derive_type: DeriveType,
    ) -> Result<Self, Error> {
        Ok(match fields {
            Fields::Unit => Self::Unit,
            fields => {
                let is_named = matches!(fields, Fields::Named(_));

                let fields = fields
                    .iter()
                    .enumerate()
                    .map(|(field_index, field)| {
                        let proto_field = SchematicField::new(field, field_index);

                        let builder = ProtoFieldBuilder {
                            field,
                            proto_field,
                            io,
                            derive_type,
                        };

                        builder.build()
                    })
                    .collect::<Result<_, _>>()?;

                if is_named {
                    Self::Named(fields)
                } else {
                    Self::Unnamed(fields)
                }
            }
        })
    }
}

/// Builds a [`SchematicField`] from a [`Field`].
struct ProtoFieldBuilder<'a> {
    field: &'a Field,
    proto_field: SchematicField,
    io: &'a mut SchematicIo,
    derive_type: DeriveType,
}

impl<'a> ProtoFieldBuilder<'a> {
    fn build(mut self) -> Result<SchematicField, Error> {
        for attr in &self.field.attrs {
            if attr.path().is_ident("reflect") {
                self.proto_field.push_reflect_attr(attr.clone());
            }

            if !attr.path().is_ident(self.derive_type.attr_name()) {
                continue;
            }

            match self.derive_type {
                DeriveType::Schematic => {
                    parse_nested_meta!(attr, |meta| {
                        FROM_ATTR => self.parse_from_meta(meta),
                        ASSET_ATTR => self.parse_asset_meta(meta),
                        ENTITY_ATTR => self.parse_entity_meta(meta),
                        OptionalArg::NAME => self.parse_optional_meta(meta),
                    })?;
                }
                DeriveType::AssetSchematic => {
                    parse_nested_meta!(attr, |meta| {
                        FROM_ATTR => self.parse_from_meta(meta),
                        ASSET_ATTR => self.parse_asset_meta(meta),
                        OptionalArg::NAME => self.parse_optional_meta(meta),
                    })?;
                }
            }
        }

        // Automatically detect `Option` types
        if self.detect_optional() {
            self.proto_field
                .config_mut()
                .try_set_optional(true, self.field.ty.span())
                .ok();
        }

        Ok(self.proto_field)
    }

    /// Detects if the field is an `Option` type.
    ///
    /// Returns `true` if the field is an `Option` type or if it was manually marked as optional.
    fn detect_optional(&mut self) -> bool {
        if self.proto_field.config().optional() {
            return true;
        }

        match &self.field.ty {
            Type::Path(ty_path) => ty_path
                .path
                .segments
                .last()
                .map(|segment| segment.ident == "Option" && !segment.arguments.is_empty())
                .unwrap_or_default(),
            _ => false,
        }
    }

    /// Parse a `#[schematic(asset)]` attribute.
    ///
    /// This takes in the meta starting at `asset`.
    fn parse_asset_meta(&mut self, meta: ParseNestedMeta) -> Result<(), Error> {
        self.require_input(meta.path.span())?;

        let config = self
            .proto_field
            .config_mut()
            .try_init_asset_kind(meta.path.span())?;

        if meta.input.is_empty() {
            return Ok(());
        }

        match self.derive_type {
            DeriveType::Schematic => {
                parse_nested_meta!(meta, |meta| {
                    AssetPreloadArg::NAME => config.try_set_preload(parse_bool(&meta)?, meta.input.span()),
                    AssetInlineArg::NAME => config.try_set_inline(parse_bool(&meta)?, meta.input.span()),
                    AssetUniqueArg::NAME => config.try_set_unique(parse_bool(&meta)?, meta.input.span()),
                    AssetPathArg::NAME => config.try_set_path(meta.value()?.parse()?, meta.input.span()),
                    AssetTypeArg::NAME => config.try_set_custom_type(meta.value()?.parse()?, meta.input.span()),
                })
            }
            DeriveType::AssetSchematic => {
                parse_nested_meta!(meta, |meta| {
                    AssetInlineArg::NAME => config.try_set_inline(parse_bool(&meta)?, meta.input.span()),
                    AssetPathArg::NAME => config.try_set_path(meta.value()?.parse()?, meta.input.span()),
                    AssetTypeArg::NAME => config.try_set_custom_type(meta.value()?.parse()?, meta.input.span()),
                })
            }
        }
    }

    /// Parse a `#[schematic(entity)]` attribute.
    ///
    /// This takes in the meta starting at `entity`.
    fn parse_entity_meta(&mut self, meta: ParseNestedMeta) -> Result<(), Error> {
        self.require_input(meta.path.span())?;

        let config = self
            .proto_field
            .config_mut()
            .try_init_entity_kind(meta.path.span())?;

        if meta.input.is_empty() {
            return Ok(());
        }

        parse_nested_meta!(meta, |meta| {
            EntityPathArg::NAME => config.try_set_path(meta.value()?.parse()?, meta.input.span()),
        })
    }

    /// Parse a `#[schematic(from)]` attribute.
    ///
    /// This takes in the meta starting at `from`.
    fn parse_from_meta(&mut self, meta: ParseNestedMeta) -> Result<(), Error> {
        self.require_input(meta.path.span())?;

        self.proto_field
            .config_mut()
            .try_init_from_kind(meta.value()?.parse()?, meta.input.span())
    }

    /// Parse a `#[schematic(optional)]` attribute.
    ///
    /// This takes in the meta starting at `optional`.
    fn parse_optional_meta(&mut self, meta: ParseNestedMeta) -> Result<(), Error> {
        self.proto_field
            .config_mut()
            .try_set_optional(parse_bool(&meta)?, meta.input.span())
    }

    /// Method used to require that a `Schematic::Input` type be generated for the attribute
    /// (identified by the given [`Span`]).
    fn require_input(&mut self, span: Span) -> Result<(), Error> {
        match &self.io.input_ty() {
            InputType::Generated(_) => Ok(()),
            _ => self
                .io
                .try_set_input_ty(InputType::new_generated(self.io.ident()), Some(span)),
        }
    }
}
