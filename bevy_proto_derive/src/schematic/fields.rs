use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Member, Type};

use crate::schematic::field_attributes::{EntityConfig, FieldAttributes, ReplacementType};
use crate::schematic::idents::{ENTITY_IDENT, INPUT_IDENT, TREE_IDENT};

/// Field data for a `Schematic`.
///
/// The [`ToTokens`] impl for this type will generate the field definitions for a
/// generated `Schematic::Input` type, and will automatically use the field's
/// [`ReplacementType`], if any.
pub(crate) struct SchematicField {
    attrs: FieldAttributes,
    member: Member,
    ty: Type,
    replacement_ty: Option<Type>,
    bevy_crate: TokenStream,
    proto_crate: TokenStream,
}

impl SchematicField {
    pub fn new<M: Into<Member>>(
        attrs: FieldAttributes,
        member: M,
        ty: Type,
        proto_crate: &TokenStream,
        bevy_crate: &TokenStream,
    ) -> Self {
        let replacement_ty = attrs.replacement_ty().generate_type(proto_crate);

        Self {
            attrs,
            member: member.into(),
            ty,
            replacement_ty,
            bevy_crate: bevy_crate.clone(),
            proto_crate: proto_crate.clone(),
        }
    }

    /// The field's `#[schematic]` attributes.
    pub fn attrs(&self) -> &FieldAttributes {
        &self.attrs
    }

    /// The member (ident or index) used to access this field.
    pub fn member(&self) -> &Member {
        &self.member
    }

    /// The type of the field, as defined by the user.
    pub fn defined_ty(&self) -> &Type {
        &self.ty
    }

    /// The type of the field, as determined by its [`ReplacementType`].
    ///
    /// If there is no replacement type, returns `None`.
    pub fn replacement_ty(&self) -> Option<&Type> {
        self.replacement_ty.as_ref()
    }

    pub fn bevy_crate(&self) -> &TokenStream {
        &self.bevy_crate
    }

    pub fn proto_crate(&self) -> &TokenStream {
        &self.proto_crate
    }

    /// Generate the conversion from the field to its [`ReplacementType`].
    pub fn conversion_def(&self) -> TokenStream {
        let proto_crate = &self.proto_crate;
        let bevy_crate = &self.bevy_crate;
        let member = &self.member;
        let ty = &self.ty;

        match self.attrs().replacement_ty() {
            ReplacementType::None => quote!(#INPUT_IDENT.#member),
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
                                #INPUT_IDENT
                                    .#member
                                    .to_asset_path()
                                    .expect("ProtoAsset should contain an asset path")
                            )
                    )
                }
            }
            replacement_ty @ ReplacementType::Entity(EntityConfig::Undefined) => {
                let entity_ty = replacement_ty.generate_type(proto_crate).unwrap();
                quote! {
                     <#ty as #proto_crate::schematics::FromSchematicInput<#entity_ty>>::from_input(
                        #INPUT_IDENT.#member,
                        #ENTITY_IDENT,
                        #TREE_IDENT,
                    )
                }
            }
            replacement_ty @ ReplacementType::Entity(EntityConfig::Path(path)) => {
                let entity_ty = replacement_ty.generate_type(proto_crate).unwrap();
                quote! {
                    <#ty as #proto_crate::schematics::FromSchematicInput<#entity_ty>>::from_input(
                        #proto_crate::tree::EntityAccess::from(#path),
                        #ENTITY_IDENT,
                        #TREE_IDENT,
                    )
                }
            }
            ReplacementType::From(replacement_ty) => quote! {
                <#ty as #proto_crate::schematics::FromSchematicInput<#replacement_ty>>::from_input(
                    #INPUT_IDENT.#member,
                    #ENTITY_IDENT,
                    #TREE_IDENT,
                )
            },
        }
    }

    /// Returns true if this field should be included in a generated input's type definition.
    pub fn should_generate(self: &&Self) -> bool {
        match self.attrs().replacement_ty() {
            ReplacementType::Asset(config) if config.path().is_some() => false,
            ReplacementType::Entity(EntityConfig::Path(_)) => false,
            _ => true,
        }
    }

    /// Compile-time assertions, if any.
    ///
    /// These are generated within an anonymous context and should either:
    /// 1. Enforce invariants at runtime
    /// 2. Provide clearer error outputs for users
    pub fn assertions(&self) -> Option<TokenStream> {
        self.attrs().assertions()
    }

    /// Takes a list of fields and returns an iterator over their struct/enum definition stream.
    ///
    /// This will automatically filter out any field that [should not be generated].
    ///
    /// [should not be generated]: Self::should_generate
    pub fn iter_definitions(fields: &[Self]) -> impl Iterator<Item = TokenStream> + '_ {
        fields.iter().filter(Self::should_generate).map(|field| {
            let attrs = field.attrs().reflect_attrs();
            quote! {
                #(#attrs)*
                #field
            }
        })
    }
}

impl ToTokens for SchematicField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ty = self.replacement_ty().unwrap_or(&self.ty);
        tokens.extend(match &self.member {
            Member::Named(ident) => quote!(#ident: #ty),
            Member::Unnamed(_) => quote!(#ty),
        })
    }
}
