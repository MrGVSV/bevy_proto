use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::token::Pub;
use syn::{Generics, Token, TypePath, Visibility};
use to_phantom::ToPhantom;

use crate::schematic::data::SchematicData;
use crate::schematic::fields::SchematicField;
use crate::schematic::idents::{ENTITY_IDENT, INPUT_IDENT, TREE_IDENT};
use crate::schematic::self_type::SelfType;
use crate::schematic::structs::SchematicStruct;
use crate::schematic::DeriveSchematic;
use crate::utils::{get_bevy_crate, get_proto_crate};

/// The type of the `Schematic::Input`.
#[derive(Default)]
pub(crate) enum InputType {
    /// Corresponds to using `Self`.
    #[default]
    Reflexive,
    /// Specifies an existing type.
    ///
    /// Most often this will be a user-generated type made to be used in `#[schematic(from = path::to::Type)]`.
    Existing(TypePath),
    Generated(Ident),
}

impl InputType {
    pub fn new_generated(ident: &Ident) -> Self {
        Self::Generated(format_ident!("{}Input", ident))
    }

    /// Generates a conversion statement from the [`InputType`] to the `Schematic`'s type.
    ///
    /// If no conversion is necessary, returns `None`.
    pub fn generate_conversion(
        &self,
        self_ty: &SelfType,
        proto_crate: &TokenStream,
    ) -> Option<TokenStream> {
        match (self, self_ty) {
            (InputType::Reflexive, SelfType::Reflexive) => None,
            (_, SelfType::Into(self_ty)) => Some(quote! {
                let #INPUT_IDENT = <Self as #proto_crate::schematics::FromSchematicInput<Self::Input>>::from_input(
                    #INPUT_IDENT, #ENTITY_IDENT, #TREE_IDENT
                );
                let #INPUT_IDENT = <#self_ty as #proto_crate::schematics::FromSchematicInput<Self>>::from_input(
                    #INPUT_IDENT, #ENTITY_IDENT, #TREE_IDENT
                );
            }),
            _ => Some(quote! {
                let #INPUT_IDENT = <Self as #proto_crate::schematics::FromSchematicInput<Self::Input>>::from_input(#INPUT_IDENT, #ENTITY_IDENT, #TREE_IDENT);
            }),
        }
    }

    /// Compile-time assertions, if any.
    ///
    /// These are generated within an anonymous context and should either:
    /// 1. Enforce invariants at runtime
    /// 2. Provide clearer error outputs for users
    pub fn assertions(&self) -> Option<TokenStream> {
        None
    }
}

impl ToTokens for InputType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            InputType::Reflexive => Token![Self](Span::call_site()).to_tokens(tokens),
            InputType::Existing(path) => path.to_tokens(tokens),
            InputType::Generated(ident) => ident.to_tokens(tokens),
        }
    }
}

/// A temp struct used to generate a `Schematic::Input` type.
pub(crate) struct Input<'a> {
    vis: Visibility,
    base_ident: &'a Ident,
    input_ident: &'a Ident,
    generics: &'a Generics,
    data: &'a SchematicData,
}

impl<'a> Input<'a> {
    /// Returns the [`Input`] definition used to generate a `Schematic::Input` type.
    ///
    /// This will return `None` if no type needs to be generated.
    pub fn get_input(derive_data: &'a DeriveSchematic) -> Option<Input<'a>> {
        let vis = derive_data
            .attrs()
            .input_vis()
            .cloned()
            .unwrap_or(Visibility::Public(Pub::default()));

        match derive_data.input_ty() {
            InputType::Generated(ident) => Some(Self {
                vis,
                base_ident: derive_data.ident(),
                input_ident: ident,
                generics: derive_data.generics(),
                data: derive_data.data(),
            }),
            _ => None,
        }
    }
}

impl<'a> ToTokens for Input<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let proto_crate = get_proto_crate();
        let bevy_crate = get_bevy_crate();

        let vis = &self.vis;
        let base_ty = self.base_ident;
        let input_ident = self.input_ident;
        let (impl_generics, impl_ty_generics, where_clause) = self.generics.split_for_impl();

        // Ex: Foo<T>
        let input_ty = quote!(#input_ident #impl_ty_generics);
        // Ex: Foo<T: Bar>
        let input_ty_def = quote!(#input_ident #impl_generics);
        let phantom_ty = if self.generics.params.is_empty() {
            None
        } else {
            Some(self.generics.to_phantom())
        };

        let make_from_impl = |body: TokenStream| {
            quote! {
                impl #impl_generics #proto_crate::schematics::FromSchematicInput<#input_ty> for #base_ty #impl_ty_generics #where_clause {
                    fn from_input(
                        #INPUT_IDENT: #input_ty,
                        #ENTITY_IDENT: &mut #bevy_crate::ecs::world::EntityMut,
                        #TREE_IDENT: &#proto_crate::tree::EntityTree
                    ) -> Self {
                        #body
                    }
                }
            }
        };

        match self.data {
            SchematicData::Struct(SchematicStruct::Unit) => {
                let from_impl = make_from_impl(quote!(Self));

                tokens.extend(quote! {
                    #[derive(#bevy_crate::prelude::Reflect, #bevy_crate::prelude::FromReflect)]
                    #vis struct #input_ty_def #where_clause;

                    #from_impl
                })
            }
            SchematicData::Struct(SchematicStruct::Unnamed(fields)) => {
                let conversions = fields.iter().map(SchematicField::conversion_def);
                let filtered = SchematicField::iter_definitions(fields);
                let from_impl = make_from_impl(quote! {
                    Self(
                        #(#conversions),*
                    )
                });

                let phantom_ty = phantom_ty.map(|phantom_ty| {
                    quote! {
                        #[reflect(ignore, default)]
                        #phantom_ty
                    }
                });

                tokens.extend(quote! {
                    #[derive(#bevy_crate::prelude::Reflect, #bevy_crate::prelude::FromReflect)]
                    #vis struct #input_ty_def (
                        #(#filtered,)*
                        #phantom_ty
                    ) #where_clause;

                    #from_impl
                })
            }
            SchematicData::Struct(SchematicStruct::Named(fields)) => {
                let conversions = fields.iter().map(SchematicField::conversion_def);
                let members = fields.iter().map(SchematicField::member);
                let filtered = SchematicField::iter_definitions(fields);

                let from_impl = make_from_impl(quote! {
                    Self {
                        #(#members: #conversions),*
                    }
                });

                let phantom_ty = phantom_ty.map(|phantom_ty| {
                    quote! {
                        #[reflect(ignore, default)]
                        __phantom_ty__: #phantom_ty
                    }
                });

                tokens.extend(quote! {
                    #[derive(#bevy_crate::prelude::Reflect, #bevy_crate::prelude::FromReflect)]
                    #vis struct #input_ty_def #where_clause {
                        #(#filtered,)*
                        #phantom_ty
                    }

                    #from_impl
                })
            }
            SchematicData::Enum(variants) => {
                let constructors = variants
                    .iter()
                    .map(|variant| variant.generate_constructor_arm(input_ident));

                let from_impl = make_from_impl(quote! {
                    match #INPUT_IDENT {
                        #(#constructors,)*
                        _ => unreachable!(),
                    }
                });

                let phantom_ty = phantom_ty.map(|phantom_ty| {
                    quote! {
                        _Phantom(#[reflect(ignore, default)] #phantom_ty)
                    }
                });

                tokens.extend(quote! {
                    #[derive(#bevy_crate::prelude::Reflect, #bevy_crate::prelude::FromReflect)]
                    #vis enum #input_ty_def #where_clause {
                        #(#variants,)*
                        #phantom_ty
                    }

                    #from_impl
                })
            }
        }
    }
}
