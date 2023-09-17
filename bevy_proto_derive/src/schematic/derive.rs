use crate::common::data::{DeriveType, SchematicData};
use crate::common::fields::SchematicFields;
use crate::common::input::{
    generate_from_reflect_conversion, generate_input, generate_input_conversion, InputType,
    OutputType, SchematicIo,
};
use crate::utils::constants::{CONTEXT_IDENT, DEPENDENCIES_IDENT, ID_IDENT, INPUT_IDENT};
use crate::utils::exports::{DependenciesBuilder, Schematic, SchematicContext, SchematicId};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{DeriveInput, Error, Generics, Visibility};

use crate::schematic::container_attributes::{ContainerAttributes, SchematicKind};

pub(crate) struct DeriveSchematic {
    attrs: ContainerAttributes,
    generics: Generics,
    data: SchematicData,
    io: SchematicIo,
}

impl DeriveSchematic {
    fn generics(&self) -> &Generics {
        &self.generics
    }

    fn io(&self) -> &SchematicIo {
        &self.io
    }

    fn output_ty(&self) -> &OutputType {
        self.io.output_ty()
    }

    fn input_ty(&self) -> &InputType {
        self.io.input_ty()
    }

    fn data(&self) -> &SchematicData {
        &self.data
    }

    /// Generate the logic for `Schematic::apply`.
    fn apply_def(&self) -> TokenStream {
        let from_reflect = generate_from_reflect_conversion();
        let conversion = generate_input_conversion(self.io());

        let insert = if matches!(self.attrs.kind(), SchematicKind::Resource) {
            quote!(#CONTEXT_IDENT.world_mut().insert_resource(#INPUT_IDENT);)
        } else {
            quote! {
                #CONTEXT_IDENT
                    .entity_mut()
                    .unwrap_or_else(|| panic!("schematic `{}` expected entity", std::any::type_name::<Self>()))
                    .insert(#INPUT_IDENT);
            }
        };

        quote! {
            #from_reflect

            #conversion

            #insert

        }
    }

    /// Generate the logic for `Schematic::remove`.
    fn remove_def(&self) -> TokenStream {
        let output_ty = self.output_ty();

        if matches!(self.attrs.kind(), SchematicKind::Resource) {
            quote!(#CONTEXT_IDENT.world_mut().remove_resource::<#output_ty>();)
        } else {
            quote!(
                #CONTEXT_IDENT
                    .entity_mut()
                    .unwrap_or_else(|| panic!("schematic `{}` expected entity", std::any::type_name::<Self>()))
                    .remove::<#output_ty>();
            )
        }
    }

    /// Generates the logic for `Schematic::preload`.
    fn preload_def(&self) -> Result<TokenStream, Error> {
        Ok(match &self.data {
            SchematicData::Struct(SchematicFields::Unit) => TokenStream::new(),
            SchematicData::Struct(
                SchematicFields::Named(fields) | SchematicFields::Unnamed(fields),
            ) => fields
                .iter()
                .map(|field| field.generate_preload(None).map(|preload| quote!(#preload)))
                .collect::<Result<TokenStream, Error>>()?,
            SchematicData::Enum(variants) => {
                let arms = variants
                    .iter()
                    .map(|variant| variant.generate_preload_arm(self.io()))
                    .collect::<Result<Vec<_>, Error>>()?;

                quote! {
                    match #INPUT_IDENT {
                        #(#arms)*
                        _ => unreachable!(),
                    }
                }
            }
        })
    }

    fn generate(&self) -> Result<TokenStream, Error> {
        let ident: &Ident = self.io.ident();
        let (impl_generics, ty_generics, where_clause) = self.generics().split_for_impl();

        let input = generate_input(self.io(), self.data(), self.generics(), true)?;
        let apply_def = self.apply_def();
        let remove_def = self.remove_def();
        let preload_def = self.preload_def()?;

        let input_vis = self.io.input_vis();
        let input_ty = match self.input_ty() {
            InputType::Generated(ident) => {
                quote!(#ident #ty_generics)
            }
            input_ty => input_ty.to_token_stream(),
        };

        let output = quote! {
            #input

            impl #impl_generics #Schematic for #ident #ty_generics #where_clause {
                type Input = #input_ty;

                fn apply(#INPUT_IDENT: &Self::Input, #ID_IDENT: #SchematicId, #CONTEXT_IDENT: &mut #SchematicContext) {
                    #apply_def
                }

                fn remove(#INPUT_IDENT: &Self::Input, #ID_IDENT: #SchematicId, #CONTEXT_IDENT: &mut #SchematicContext) {
                    #remove_def
                }

                fn preload_dependencies(#INPUT_IDENT: &mut Self::Input, #ID_IDENT: #SchematicId, #DEPENDENCIES_IDENT: &mut #DependenciesBuilder)  {
                    #preload_def
                }
            }
        };

        Ok(match input_vis {
            None | Some(Visibility::Inherited) => quote! {
                const _: () = {
                    #output
                };
            },
            _ => output,
        })
    }
}

impl Parse for DeriveSchematic {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input = input.parse::<DeriveInput>()?;

        let mut io = SchematicIo::new(&input);

        let attrs = ContainerAttributes::new(&input.attrs, &mut io)?;

        let data = SchematicData::new(input.data, &mut io, DeriveType::Schematic)?;

        Ok(Self {
            attrs,
            generics: input.generics,
            data,
            io,
        })
    }
}

impl ToTokens for DeriveSchematic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.generate() {
            Ok(output) => tokens.extend(output),
            Err(err) => err.to_compile_error().to_tokens(tokens),
        }
    }
}
