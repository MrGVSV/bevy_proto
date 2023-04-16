use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::Member;

use crate::schematic::field_attributes::{EntityConfig, ReplacementType};
use crate::schematic::fields::SchematicField;
use crate::schematic::idents::{DEPENDENCIES_IDENT, ENTITY_IDENT, TREE_IDENT};
use crate::schematic::input::InputType;
use crate::schematic::structs::SchematicStruct;

pub(crate) struct SchematicVariant {
    pub ident: Ident,
    pub data: SchematicStruct,
}

impl SchematicVariant {
    pub fn generate_constructor_arm(&self, input_ty: &Ident) -> TokenStream {
        let ident = &self.ident;
        match &self.data {
            SchematicStruct::Unit => quote! {
                #input_ty::#ident => Self::#ident
            },
            SchematicStruct::Unnamed(fields) => {
                let pat = fields
                    .iter()
                    .filter(SchematicField::should_generate)
                    .map(|field| format_ident!("field_{}", field.member()));

                let field = fields.iter().map(|field| {
                    let ident = format_ident!("field_{}", field.member());
                    let member = Member::Named(ident);
                    self.convert_field(field, Some(&member))
                });

                quote! {
                    #input_ty::#ident( #(#pat,)* .. ) => Self::#ident( #(#field),* )
                }
            }
            SchematicStruct::Named(fields) => {
                let pat = fields
                    .iter()
                    .filter(SchematicField::should_generate)
                    .map(|field| field.member());
                let member = fields.iter().map(|field| field.member());
                let conversions = fields.iter().map(|field| self.convert_field(field, None));

                quote! {
                    #input_ty::#ident{ #(#pat,)* .. } => Self::#ident{ #(#member: #conversions),* }
                }
            }
        }
    }

    pub fn generate_preload_arm(&self, input_ty: &InputType) -> TokenStream {
        let ident = &self.ident;
        match &self.data {
            SchematicStruct::Unit => quote! {
                #input_ty::#ident => {}
            },
            SchematicStruct::Unnamed(fields) => {
                let (pat, stmt): (Vec<_>, Vec<_>) = fields
                    .iter()
                    .filter_map(|field| {
                        match field.attrs().replacement_ty() {
                            ReplacementType::Asset(config) if config.is_preload() => {
                                let ty = field.defined_ty();
                                let member = field.member();
                                let name = format_ident!("field_{}", member);

                                Some(if let Some(path) = config.path() {
                                    (
                                        None,
                                        quote! {
                                            let _: #ty = #DEPENDENCIES_IDENT.add_dependency(#path);
                                        },
                                    )
                                } else {
                                    (
                                        Some(quote!(#member: #name,)),
                                        quote! {
                                            let _: #ty = #DEPENDENCIES_IDENT.add_dependency(
                                                #name.to_asset_path().expect("ProtoAsset should contain an asset path")
                                            );
                                        },
                                    )
                                })
                            },
                            _ => None
                        }
                    })
                    .unzip();

                quote! {
                    #input_ty::#ident{ #(#pat)* .. } => { #(#stmt)* }
                }
            }
            SchematicStruct::Named(fields) => {
                let (pat, stmt): (Vec<_>, Vec<_>) = fields
                    .iter()
                    .filter_map(|field| match field.attrs().replacement_ty() {
                        ReplacementType::Asset(config) if config.is_preload() => {
                            let ty = field.defined_ty();
                            let member = field.member();

                            Some(if let Some(path) = config.path() {
                                (
                                    None,
                                    quote! {
                                        let _: #ty = #DEPENDENCIES_IDENT.add_dependency(#path);
                                    },
                                )
                            } else {
                                (
                                    Some(quote!(#member,)),
                                    quote! {
                                        let _: #ty = #DEPENDENCIES_IDENT.add_dependency(
                                            #member
                                                .to_asset_path()
                                                .expect("ProtoAsset should contain an asset path")
                                                .to_owned()
                                        );
                                    },
                                )
                            })
                        }
                        _ => None,
                    })
                    .unzip();

                quote! {
                    #input_ty::#ident{ #(#pat)* .. } => { #(#stmt)* }
                }
            }
        }
    }

    fn convert_field(&self, field: &SchematicField, field_name: Option<&Member>) -> TokenStream {
        let proto_crate = field.proto_crate();
        let bevy_crate = field.bevy_crate();
        let member = field_name.unwrap_or_else(|| field.member());
        let ty = field.defined_ty();

        match field.attrs().replacement_ty() {
            ReplacementType::None => quote!(#member),
            ReplacementType::Asset(config) => {
                if let Some(path) = config.path() {
                    quote!(
                        #ENTITY_IDENT
                            .world()
                            .resource::<#bevy_crate::asset::AssetServer>()
                            .load(#path)
                    )
                } else {
                    quote!(
                        #ENTITY_IDENT
                            .world()
                            .resource::<#bevy_crate::asset::AssetServer>()
                            .load(
                                #member
                                    .to_asset_path()
                                    .expect("ProtoAsset should contain an asset path")
                            )
                    )
                }
            }
            ReplacementType::Entity(EntityConfig::Undefined) => quote! {
                #TREE_IDENT
                    .find_entity(&#member)
                    .unwrap_or_else(|| panic!("entity should exist at path {:?}", &#member))
            },
            ReplacementType::Entity(EntityConfig::Path(path)) => quote! {
                #TREE_IDENT
                    .find_entity(&#proto_crate::tree::EntityAccess::from(#path))
                    .unwrap_or_else(|| panic!("entity should exist at path {:?}", #path))
            },
            ReplacementType::From(replacement_ty) => quote! {
                <#ty as #proto_crate::schematics::FromSchematicInput<#replacement_ty>>::from_input(
                    #member,
                    #ENTITY_IDENT,
                    #TREE_IDENT
                )
            },
        }
    }

    /// Compile-time assertions, if any.
    ///
    /// These are generated within an anonymous context and should either:
    /// 1. Enforce invariants at runtime
    /// 2. Provide clearer error outputs for users
    pub fn assertions(&self) -> Option<TokenStream> {
        let assertions = self.data.assertions()?;
        let mod_ident = format_ident!("{}VariantAssertions", self.ident);

        Some(quote! {
            mod #mod_ident {
                #assertions
            }
        })
    }
}

impl ToTokens for SchematicVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        match &self.data {
            SchematicStruct::Unit => ident.to_tokens(tokens),
            SchematicStruct::Unnamed(fields) => {
                let filtered = SchematicField::iter_definitions(fields);

                tokens.extend(quote! {
                    #ident( #(#filtered),* )
                })
            }
            SchematicStruct::Named(fields) => {
                let filtered = SchematicField::iter_definitions(fields);

                tokens.extend(quote! {
                    #ident{ #(#filtered),* }
                })
            }
        }
    }
}
