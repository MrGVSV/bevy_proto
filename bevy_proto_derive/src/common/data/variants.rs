use crate::common::fields::{FieldKind, SchematicField, SchematicFields};
use crate::common::input::SchematicIo;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Error, Member};

pub(crate) struct SchematicVariant {
    pub ident: Ident,
    pub fields: SchematicFields,
}

impl SchematicVariant {
    pub fn generate_conversion_arm(&self, io: &SchematicIo) -> Result<TokenStream, Error> {
        let input_ty = io.input_ty();
        let output_ty = io.output_ty();
        let variant = &self.ident;

        Ok(match &self.fields {
            SchematicFields::Unit => quote! {
                #input_ty::#variant => #output_ty::#variant
            },
            SchematicFields::Named(fields) | SchematicFields::Unnamed(fields) => {
                let mut patterns = Vec::new();
                let mut conversions = Vec::new();
                for field in fields.iter() {
                    match field.member() {
                        Member::Named(ident) => {
                            patterns.push(quote!(#ident));

                            let conversion = field.generate_conversion(Some(quote!(#ident)))?;
                            conversions.push(quote!(#ident: #conversion))
                        }
                        Member::Unnamed(index) => {
                            let ident = format_ident!("field_{}", index);
                            patterns.push(quote!(#index: #ident));

                            let conversion = field.generate_conversion(Some(quote!(#ident)))?;
                            conversions.push(quote!(#index: #conversion))
                        }
                    }
                }

                quote! {
                    #input_ty::#variant{ #(#patterns,)* .. } => Self::#variant{ #(#conversions),* }
                }
            }
        })
    }

    pub fn generate_preload_conversion_arm(&self, io: &SchematicIo) -> Result<TokenStream, Error> {
        let input_ty = io.input_ty();
        let output_ty = io.output_ty();
        let variant = &self.ident;

        Ok(match &self.fields {
            SchematicFields::Unit => quote! {
                #input_ty::#variant => #output_ty::#variant
            },
            SchematicFields::Named(fields) | SchematicFields::Unnamed(fields) => {
                let mut patterns = Vec::new();
                let mut conversions = Vec::new();
                for field in fields.iter() {
                    match field.member() {
                        Member::Named(ident) => {
                            patterns.push(quote!(#ident));

                            let conversion =
                                field.generate_preload_conversion(Some(quote!(#ident)))?;
                            conversions.push(quote!(#ident: #conversion))
                        }
                        Member::Unnamed(index) => {
                            let ident = format_ident!("field_{}", index);
                            patterns.push(quote!(#index: #ident));

                            let conversion =
                                field.generate_preload_conversion(Some(quote!(#ident)))?;
                            conversions.push(quote!(#index: #conversion))
                        }
                    }
                }

                quote! {
                    #input_ty::#variant{ #(#patterns,)* .. } => Self::#variant{ #(#conversions),* }
                }
            }
        })
    }

    pub fn generate_preload_arm(&self, io: &SchematicIo) -> Result<TokenStream, Error> {
        let input_ty = io.input_ty();
        let variant = &self.ident;

        Ok(match &self.fields {
            SchematicFields::Unit => quote! {
                #input_ty::#variant => {/* Do nothing */},
            },
            SchematicFields::Named(fields) | SchematicFields::Unnamed(fields) => {
                let mut patterns = Vec::new();
                let mut preloads = Vec::new();
                let preload_fields = fields.iter().filter(|field| match field.config().kind() {
                    Some(FieldKind::Asset(config)) => config.preload(),
                    _ => false,
                });
                for field in preload_fields {
                    match field.member() {
                        Member::Named(ident) => {
                            patterns.push(quote!(#ident));
                            preloads.push(field.generate_preload(Some(quote!(*#ident)))?)
                        }
                        Member::Unnamed(index) => {
                            let ident = format_ident!("field_{}", index);
                            patterns.push(quote!(#index: #ident));
                            preloads.push(field.generate_preload(Some(quote!(*#ident)))?)
                        }
                    }
                }

                quote! {
                    #input_ty::#variant{ #(#patterns,)* .. } => {
                        #(#preloads)*
                    }
                }
            }
        })
    }

    pub fn generate_definition(&self) -> TokenStream {
        let variant = &self.ident;
        match &self.fields {
            SchematicFields::Unit => quote! {
                #variant
            },
            SchematicFields::Named(fields) => {
                let fields = fields
                    .iter()
                    .filter(SchematicField::requires_input_field)
                    .map(SchematicField::generate_definition);
                quote! {
                    #variant { #(#fields),* }
                }
            }
            SchematicFields::Unnamed(fields) => {
                let fields = fields
                    .iter()
                    .filter(SchematicField::requires_input_field)
                    .map(SchematicField::generate_definition);
                quote! {
                    #variant ( #(#fields),* )
                }
            }
        }
    }
}
