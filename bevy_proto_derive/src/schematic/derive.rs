use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{DeriveInput, Generics, Visibility};

use crate::schematic::container_attributes::{ContainerAttributes, SchematicKind};
use crate::schematic::data::SchematicData;
use crate::schematic::field_attributes::ReplacementType;
use crate::schematic::idents::{DEPENDENCIES_IDENT, ENTITY_IDENT, INPUT_IDENT, TREE_IDENT};
use crate::schematic::input::{Input, InputType};
use crate::schematic::self_type::SelfType;
use crate::schematic::structs::SchematicStruct;
use crate::utils::{get_bevy_crate, get_proto_crate};

pub(crate) struct DeriveSchematic {
    attrs: ContainerAttributes,
    ident: Ident,
    generics: Generics,
    data: SchematicData,
    self_ty: SelfType,
    input_ty: InputType,
    assertions: TokenStream,
    proto_crate: TokenStream,
    bevy_crate: TokenStream,
}

impl DeriveSchematic {
    pub fn attrs(&self) -> &ContainerAttributes {
        &self.attrs
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn generics(&self) -> &Generics {
        &self.generics
    }

    pub fn proto_crate(&self) -> &TokenStream {
        &self.proto_crate
    }

    pub fn bevy_crate(&self) -> &TokenStream {
        &self.bevy_crate
    }

    pub fn self_ty(&self) -> &SelfType {
        &self.self_ty
    }

    pub fn input_ty(&self) -> &InputType {
        &self.input_ty
    }

    pub fn data(&self) -> &SchematicData {
        &self.data
    }

    /// Generate the logic for `Schematic::apply`.
    pub fn apply_def(&self) -> TokenStream {
        let conversion = self
            .input_ty
            .generate_conversion(self.self_ty(), &self.proto_crate);
        let construct_input = self.generate_from_reflect_input();

        let insert = if matches!(self.attrs.kind(), SchematicKind::Resource) {
            quote!(#ENTITY_IDENT.world_scope(|world| world.insert_resource(#INPUT_IDENT));)
        } else {
            quote!(#ENTITY_IDENT.insert(#INPUT_IDENT);)
        };

        quote! {
            let #INPUT_IDENT = #construct_input;

            #conversion

            #insert

        }
    }

    /// Generate the logic for `Schematic::remove`.
    pub fn remove_def(&self) -> TokenStream {
        let self_ty = self.self_ty();

        if matches!(self.attrs.kind(), SchematicKind::Resource) {
            quote!(#ENTITY_IDENT.world_scope(|world| world.remove_resource::<#self_ty>());)
        } else {
            quote!(#ENTITY_IDENT.remove::<#self_ty>();)
        }
    }

    /// Generates the logic for `Schematic::preload`.
    pub fn preload_def(&self) -> Option<TokenStream> {
        match &self.data {
            SchematicData::Struct(SchematicStruct::Unit) => None,
            SchematicData::Struct(
                SchematicStruct::Named(fields) | SchematicStruct::Unnamed(fields),
            ) => Some(
                fields
                    .iter()
                    .filter_map(|field| match field.attrs().replacement_ty() {
                        ReplacementType::Asset(config) if config.is_preload() => {
                            let ty = field.defined_ty();
                            let member = field.member();

                            Some(if let Some(path) = config.path() {
                                quote!(
                                    let _: #ty = #DEPENDENCIES_IDENT.add_dependency(#path);
                                )
                            } else {
                                quote!(
                                    let _: #ty = #DEPENDENCIES_IDENT.add_dependency(
                                        #INPUT_IDENT.#member
                                            .to_asset_path()
                                            .expect("ProtoAsset should contain an asset path")
                                            .to_owned()
                                    );
                                )
                            })
                        }
                        _ => None,
                    })
                    .collect(),
            ),
            SchematicData::Enum(variants) => {
                let input_ty = self.input_ty();
                let arms = variants
                    .iter()
                    .map(|variant| variant.generate_preload_arm(input_ty));

                Some(quote! {
                    match #INPUT_IDENT {
                        #(#arms,)*
                        _ => unreachable!(),
                    }
                })
            }
        }
    }

    /// Generates an expression that clones the `Schematic`'s input argument to
    /// a `Box<dyn Reflect>` using its `FromReflect`.
    fn generate_from_reflect_input(&self) -> TokenStream {
        let bevy_crate = &self.bevy_crate;
        quote! {
            <Self::Input as #bevy_crate::reflect::FromReflect>::from_reflect(
                &*#bevy_crate::reflect::Reflect::clone_value(#INPUT_IDENT)
            ).unwrap_or_else(|| {
                panic!(
                    "{} should have a functioning `FromReflect` impl",
                    std::any::type_name::<Self::Input>()
                )
            })
        }
    }
}

impl Parse for DeriveSchematic {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input = input.parse::<DeriveInput>()?;

        let mut assertions = TokenStream::new();

        let mut self_ty = SelfType::default();
        #[cfg(feature = "assertions")]
        assertions.extend(self_ty.assertions());

        let mut input_ty = InputType::default();
        #[cfg(feature = "assertions")]
        assertions.extend(input_ty.assertions());

        let attrs = ContainerAttributes::new(&input.attrs, &mut self_ty, &mut input_ty)?;
        #[cfg(feature = "assertions")]
        assertions.extend(attrs.assertions());

        let proto_crate = get_proto_crate();
        let bevy_crate = get_bevy_crate();

        let data = SchematicData::from_data(
            input.data,
            &input.ident,
            &mut input_ty,
            &proto_crate,
            &bevy_crate,
        )?;
        #[cfg(feature = "assertions")]
        assertions.extend(data.assertions());

        Ok(Self {
            attrs,
            ident: input.ident,
            generics: input.generics,
            data,
            self_ty,
            input_ty,
            assertions,
            proto_crate,
            bevy_crate,
        })
    }
}

impl ToTokens for DeriveSchematic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.ident();
        let (impl_generics, ty_generics, where_clause) = self.generics().split_for_impl();

        let proto_crate = self.proto_crate();
        let bevy_crate = self.bevy_crate();

        let attrs = self.attrs();

        let input = Input::get_input(self);
        let apply_def = self.apply_def();
        let remove_def = self.remove_def();
        let preload_def = self.preload_def();

        let input_ty = match self.input_ty() {
            InputType::Generated(ident) => {
                quote!(#ident #ty_generics)
            }
            input_ty => input_ty.to_token_stream(),
        };

        let assertions = if cfg!(feature = "assertions") {
            let assertions = &self.assertions;
            Some(quote! {
                const _: () = {
                    mod Assertions {
                        #assertions
                    }
                };
            })
        } else {
            None
        };

        let output = quote! {
            #input

            impl #impl_generics #proto_crate::schematics::Schematic for #ident #ty_generics #where_clause {
                type Input = #input_ty;

                fn apply(#INPUT_IDENT: &Self::Input, #ENTITY_IDENT: &mut #bevy_crate::ecs::world::EntityMut, #TREE_IDENT: &#proto_crate::tree::EntityTree) {
                    #apply_def
                }

                fn remove(#INPUT_IDENT: &Self::Input, #ENTITY_IDENT: &mut #bevy_crate::ecs::world::EntityMut, #TREE_IDENT: &#proto_crate::tree::EntityTree) {
                    #remove_def
                }

                fn preload_dependencies(#INPUT_IDENT: &mut Self::Input, #DEPENDENCIES_IDENT: &mut #proto_crate::deps::DependenciesBuilder)  {
                    #preload_def
                }
            }

            #assertions
        };

        if matches!(attrs.input_vis(), None | Some(Visibility::Inherited)) {
            tokens.extend(quote! {
                const _: () = {
                    #output
                };
            })
        } else {
            tokens.extend(output);
        }
    }
}
