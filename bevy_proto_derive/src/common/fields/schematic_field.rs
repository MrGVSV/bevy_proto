use crate::common::fields::{FieldConfig, FieldKind};
use crate::utils::constants::{CONTEXT_IDENT, DEPENDENCIES_IDENT, INPUT_IDENT, TEMP_IDENT};
use crate::utils::exports::{
    AssetServer, EntityAccess, FromReflect, FromSchematicInput, FromSchematicPreloadInput,
    InlinableProtoAsset, ProtoAsset, Reflect,
};
use crate::utils::NextId;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{parse_quote, Attribute, Error, Field, Member, Type};

/// The base field information for fields of a `Schematic` or `AssetSchematic`.
pub(crate) struct SchematicField {
    /// The Bevy `#[reflect]` attributes applied to the field.
    ///
    /// These are used when generating the input type.
    reflect_attrs: Vec<Attribute>,
    /// The field's configuration.
    config: FieldConfig,
    /// The member (ident or index) used to access this field.
    member: Member,
    /// The type of the field, as defined by the user.
    ///
    /// This might not be what is actually generated in the input type.
    defined_ty: Type,
}

impl SchematicField {
    pub fn new(field: &Field, field_index: usize) -> Self {
        Self {
            defined_ty: field.ty.clone(),
            member: field
                .ident
                .clone()
                .map(Member::Named)
                .unwrap_or(Member::Unnamed(field_index.into())),
            reflect_attrs: Vec::new(),
            config: FieldConfig::default(),
        }
    }

    /// Pushes a `#[reflect]` attribute to the field.
    pub fn push_reflect_attr(&mut self, attr: Attribute) {
        self.reflect_attrs.push(attr);
    }

    /// Returns a reference to the field's configuration.
    pub fn config(&self) -> &FieldConfig {
        &self.config
    }

    /// Returns a mutable reference to the field's configuration.
    pub fn config_mut(&mut self) -> &mut FieldConfig {
        &mut self.config
    }

    /// The member (ident or index) used to access this field.
    pub fn member(&self) -> &Member {
        &self.member
    }

    /// Determines whether or not a generated input type should contain this field.
    ///
    /// For fields that cannot be configured by a prototype file (e.g. `path` attributes),
    /// this should return `false`.
    pub fn requires_input_field(self: &&Self) -> bool {
        match self.config.kind() {
            Some(FieldKind::Entity(config)) => config.path().is_none(),
            Some(FieldKind::Asset(config)) => config.path().is_none(),
            _ => true,
        }
    }

    /// The type of the field within a generated input type.
    ///
    /// This may or may not be the same as the field's user-defined type.
    pub fn input_ty(&self) -> Result<Type, Error> {
        let wrap_option = |ty: Type| -> Type {
            if self.config.optional() {
                parse_quote!(::core::option::Option<#ty>)
            } else {
                ty
            }
        };

        Ok(match self.config.kind() {
            None => self.defined_ty.clone(),
            Some(FieldKind::From(ty)) => wrap_option(ty.clone()),
            Some(FieldKind::Entity(_)) => wrap_option(parse_quote!(#EntityAccess)),
            Some(FieldKind::Asset(config)) => {
                let ty = if config.untyped() {
                    None
                } else {
                    Some(config.try_extract_asset_type(&self.defined_ty)?)
                };

                let inline = config.inline();

                match (inline, ty) {
                    (true, Some(ty)) => wrap_option(parse_quote!(#InlinableProtoAsset<#ty>)),
                    (false, Some(ty)) => wrap_option(parse_quote!(#ProtoAsset<#ty>)),
                    _ => {
                        return Err(Error::new(
                            self.member.span(),
                            "untyped assets are not yet supported",
                        ))
                    }
                }
            }
        })
    }

    /// Generate this field's definition within a generated input type.
    pub fn generate_definition(&self) -> TokenStream {
        let reflect_attrs = &self.reflect_attrs;
        let ty = match self.input_ty() {
            Ok(ty) => ty,
            Err(err) => return err.to_compile_error(),
        };
        match &self.member {
            Member::Named(ident) => quote! {
                #(#reflect_attrs)*
                #ident: #ty
            },
            Member::Unnamed(_) => quote! {
                #(#reflect_attrs)*
                #ty
            },
        }
    }

    /// Generate this field's conversion from the generated input type to the user-defined type.
    ///
    /// The generated `TokenStream` will be an expression that accesses the field from the input
    /// type and converts it to the user-defined type.
    pub fn generate_conversion(
        &self,
        custom_accessor: Option<TokenStream>,
    ) -> Result<TokenStream, Error> {
        // Locate span at the field's member so that error messages point to the offending field.
        let span = Span::call_site().located_at(self.member.span());

        let accessor = custom_accessor.unwrap_or_else(|| {
            let member = self.member();
            quote_spanned!(span => #INPUT_IDENT.#member)
        });

        Ok(match self.config.kind() {
            Some(FieldKind::From(_)) => {
                if self.config.optional() {
                    quote_spanned! {span =>
                        #accessor.map(|#TEMP_IDENT| #FromSchematicInput::from_input(
                            #TEMP_IDENT,
                            #NextId,
                            #CONTEXT_IDENT,
                        ))
                    }
                } else {
                    quote_spanned! {span =>
                        #FromSchematicInput::from_input(
                            #accessor,
                            #NextId,
                            #CONTEXT_IDENT,
                        )
                    }
                }
            }
            Some(FieldKind::Entity(config)) => {
                let access = if let Some(path) = config.path() {
                    quote_spanned!(span => #EntityAccess::from(#path))
                } else {
                    quote_spanned!(span => #accessor)
                };

                if self.config.optional() {
                    quote_spanned! {span =>
                        #access.map(|#TEMP_IDENT| #FromSchematicInput::from_input(
                            #TEMP_IDENT,
                            #NextId,
                            #CONTEXT_IDENT,
                        ))
                    }
                } else {
                    quote_spanned! {span =>
                        #FromSchematicInput::from_input(
                            #access,
                            #NextId,
                            #CONTEXT_IDENT,
                        )
                    }
                }
            }
            Some(FieldKind::Asset(config)) => {
                let id = config.asset_id();

                if let Some(path) = config.path() {
                    quote_spanned! {span =>
                        #CONTEXT_IDENT
                            .world()
                            .resource::<#AssetServer>()
                            .load(#path)
                    }
                } else if self.config.optional() {
                    quote_spanned! {span =>
                        #accessor.map(|#TEMP_IDENT| #FromSchematicInput::from_input(
                            #TEMP_IDENT,
                            #id,
                            #CONTEXT_IDENT,
                        ))
                    }
                } else {
                    quote_spanned! {span =>
                        #FromSchematicInput::from_input(
                            #accessor,
                            #id,
                            #CONTEXT_IDENT,
                        )
                    }
                }
            }
            _ => quote_spanned!(span => #accessor),
        })
    }

    /// Generate the preload-specific conversion from the generated input type to the user-defined type.
    ///
    /// The generated `TokenStream` will be an expression that accesses the field from the input
    /// type and converts it to the user-defined type.
    pub fn generate_preload_conversion(
        &self,
        custom_accessor: Option<TokenStream>,
    ) -> Result<TokenStream, Error> {
        // Locate span at the field's member so that error messages point to the offending field.
        let span = Span::call_site().located_at(self.member.span());

        let accessor = custom_accessor.unwrap_or_else(|| {
            let member = self.member();
            quote_spanned!(span => #INPUT_IDENT.#member)
        });

        Ok(match self.config.kind() {
            Some(FieldKind::From(_)) => {
                if self.config.optional() {
                    quote_spanned! {span =>
                        #accessor.map(|#TEMP_IDENT| #FromSchematicPreloadInput::from_preload_input(
                            #TEMP_IDENT,
                            #NextId,
                            #DEPENDENCIES_IDENT,
                        ))
                    }
                } else {
                    quote_spanned! {span =>
                        #FromSchematicPreloadInput::from_preload_input(
                            #accessor,
                            #NextId,
                            #DEPENDENCIES_IDENT,
                        )
                    }
                }
            }
            Some(FieldKind::Entity(_)) => TokenStream::new(),
            Some(FieldKind::Asset(config)) => {
                let id = config.asset_id();

                if let Some(path) = config.path() {
                    if self.config.optional() {
                        quote_spanned! {span =>
                            ::core::option::Option::Some(#DEPENDENCIES_IDENT.add_dependency(#path))
                        }
                    } else {
                        quote_spanned! {span =>
                            #DEPENDENCIES_IDENT.add_dependency(#path)
                        }
                    }
                } else if self.config.optional() {
                    quote_spanned! {span =>
                        #accessor.map(|#TEMP_IDENT| #FromSchematicPreloadInput::from_preload_input(
                            #TEMP_IDENT,
                            #id,
                            #DEPENDENCIES_IDENT,
                        ))
                    }
                } else {
                    quote_spanned! {span =>
                        #FromSchematicPreloadInput::from_preload_input(
                            #accessor,
                            #id,
                            #DEPENDENCIES_IDENT,
                        )
                    }
                }
            }
            _ => quote_spanned!(span => #accessor),
        })
    }

    /// Generates the preload code for the field (assets-only).
    ///
    /// The generated `TokenStream` will be a statement that sets the user-defined field to the asset
    /// loaded using the input type's field.
    pub fn generate_preload(
        &self,
        variant_field_ident: Option<TokenStream>,
    ) -> Result<TokenStream, Error> {
        // Locate span at the field's member so that error messages point to the offending field.
        let span = Span::call_site().located_at(self.member.span());

        Ok(match self.config.kind() {
            Some(FieldKind::Asset(config)) if config.preload() => {
                let accessor = variant_field_ident.unwrap_or_else(|| {
                    let member = self.member();
                    quote_spanned!(span => #INPUT_IDENT.#member)
                });

                let asset_enum = if config.inline() {
                    InlinableProtoAsset.to_token_stream()
                } else {
                    ProtoAsset.to_token_stream()
                };

                if let Some(path) = config.path() {
                    if self.config.optional() {
                        quote_spanned! {span =>
                            #accessor = ::core::option::Option::Some(
                                #asset_enum::Handle(#DEPENDENCIES_IDENT.add_dependency(#path))
                            );
                        }
                    } else {
                        quote_spanned! {span =>
                            #accessor = #asset_enum::Handle(#DEPENDENCIES_IDENT.add_dependency(#path));
                        }
                    }
                } else {
                    let input_ty = self.input_ty()?;
                    let asset_ty = config.try_extract_asset_type(&self.defined_ty)?;
                    let id = config.asset_id();

                    let convert = if self.config.optional() {
                        quote_spanned! {span =>
                            #TEMP_IDENT.map(|#TEMP_IDENT| {
                                #asset_enum::Handle(#FromSchematicPreloadInput::from_preload_input(
                                    #TEMP_IDENT,
                                    #id,
                                    #DEPENDENCIES_IDENT,
                                ))
                            })

                        }
                    } else {
                        quote_spanned! {span =>
                            #asset_enum::Handle(#FromSchematicPreloadInput::from_preload_input(
                                #TEMP_IDENT,
                                #id,
                                #DEPENDENCIES_IDENT,
                            ))
                        }
                    };

                    quote_spanned! {span =>
                        #accessor = {
                            let #TEMP_IDENT = <#input_ty as #FromReflect>::from_reflect(
                                &*#Reflect::clone_value(&#accessor)
                            ).unwrap_or_else(|| {
                                panic!(
                                    "{} should have a functioning `FromReflect` impl",
                                    ::std::any::type_name::<#asset_ty>()
                                )
                            });

                            #convert
                        };
                    }
                }
            }
            _ => TokenStream::new(),
        })
    }
}
