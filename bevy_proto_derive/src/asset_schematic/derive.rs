use crate::asset_schematic::container_attributes::ContainerAttributes;
use crate::common::data::{DeriveType, SchematicData};
use crate::common::input::{
    generate_from_reflect_conversion, generate_input, generate_input_conversion,
    generate_preload_input_conversion, InputType, OutputType, SchematicIo,
};
use crate::utils::constants::{CONTEXT_IDENT, DEPENDENCIES_IDENT, ID_IDENT, INPUT_IDENT};
use crate::utils::exports::{
    AssetSchematic, DependenciesBuilder, PreloadAssetSchematic, SchematicContext, SchematicId,
};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{DeriveInput, Error, Generics, Visibility};

pub(crate) struct DeriveAssetSchematic {
    attrs: ContainerAttributes,
    generics: Generics,
    data: SchematicData,
    io: SchematicIo,
}

impl DeriveAssetSchematic {
    fn attrs(&self) -> &ContainerAttributes {
        &self.attrs
    }

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

    fn load_def(&self) -> TokenStream {
        let from_reflect = generate_from_reflect_conversion();
        let conversion = generate_input_conversion(self.io());

        quote! {
            #from_reflect

            #conversion

            #INPUT_IDENT
        }
    }

    /// Generates the logic for `PreloadAssetSchematic::preload`.
    fn preload_def(&self) -> Option<TokenStream> {
        if self.attrs.no_preload() {
            return None;
        }

        let from_reflect = generate_from_reflect_conversion();
        let conversion = generate_preload_input_conversion(self.io());

        Some(quote! {
            let #INPUT_IDENT = &#INPUT_IDENT;

            #from_reflect

            #conversion

            #INPUT_IDENT
        })
    }

    fn try_to_tokens(&self) -> Result<TokenStream, Error> {
        let ident: &Ident = self.io.ident();
        let (impl_generics, ty_generics, where_clause) = self.generics().split_for_impl();

        let input = generate_input(
            self.io(),
            self.data(),
            self.generics(),
            self.attrs().no_preload(),
        )?;
        let load_def = self.load_def();

        let input_vis = self.io.input_vis();
        let input_ty = match self.input_ty() {
            InputType::Generated(ident) => {
                quote!(#ident #ty_generics)
            }
            input_ty => input_ty.to_token_stream(),
        };
        let output_ty = self.output_ty();

        let preload_impl = self.preload_def().map( |preload_def| {
            quote! {
                impl #impl_generics #PreloadAssetSchematic for #ident #ty_generics #where_clause {
                    fn preload(#INPUT_IDENT: Self::Input, #ID_IDENT: #SchematicId, #DEPENDENCIES_IDENT: &mut #DependenciesBuilder) -> Self::Output {
                        #preload_def
                    }
                }
            }
        });

        let output = quote! {
            #input

            impl #impl_generics #AssetSchematic for #ident #ty_generics #where_clause {
                type Input = #input_ty;
                type Output = #output_ty;

                fn load(#INPUT_IDENT: &Self::Input, #ID_IDENT: #SchematicId, #CONTEXT_IDENT: &mut #SchematicContext) -> Self::Output {
                    #load_def
                }
            }

            #preload_impl
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

impl Parse for DeriveAssetSchematic {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input = input.parse::<DeriveInput>()?;

        let mut io = SchematicIo::new(&input);

        let attrs = ContainerAttributes::new(&input.attrs, &mut io)?;

        let data = SchematicData::new(input.data, &mut io, DeriveType::AssetSchematic)?;

        Ok(Self {
            attrs,
            generics: input.generics,
            data,
            io,
        })
    }
}

impl ToTokens for DeriveAssetSchematic {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.try_to_tokens() {
            Ok(output) => tokens.extend(output),
            Err(err) => tokens.extend(err.to_compile_error()),
        }
    }
}
