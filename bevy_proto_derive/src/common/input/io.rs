use crate::common::input::{InputType, InputVisArg, OutputType};
use crate::utils::AttrArgValue;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{DeriveInput, Error, Visibility};

/// The input and output types of a schematic.
///
/// An input type is the type that the schematic is deserialized from,
/// while an output type is the type that the schematic is applied as.
pub(crate) struct SchematicIo {
    /// The name of the schematic.
    ident: Ident,
    /// The visibility of the schematic.
    vis: Visibility,
    /// The visibility of the input type.
    input_vis: InputVisArg,
    /// The input type.
    input_ty: InputType,
    /// The output type.
    output_ty: OutputType,
}

impl SchematicIo {
    pub fn new(input: &DeriveInput) -> Self {
        Self {
            ident: input.ident.clone(),
            vis: input.vis.clone(),
            input_vis: InputVisArg::default(),
            input_ty: InputType::Reflexive,
            output_ty: OutputType::Reflexive,
        }
    }

    /// The user-defined name of the schematic.
    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    /// The user-defined visibility of the schematic.
    pub fn vis(&self) -> &Visibility {
        &self.vis
    }

    pub fn input_ty(&self) -> &InputType {
        &self.input_ty
    }

    pub fn try_set_input_ty(&mut self, value: InputType, span: Option<Span>) -> Result<(), Error> {
        match &self.input_ty {
            InputType::Reflexive => {
                self.input_ty = value;
                Ok(())
            }
            InputType::Existing(ty) => {
                let message = if matches!(value, InputType::Generated(_)) {
                    format!(
                        "an input type needs to be generated, but it's already specified as {}",
                        ty.to_token_stream()
                    )
                } else {
                    format!("input type already specified as {}", ty.to_token_stream())
                };
                Err(Error::new(span.unwrap_or_else(|| value.span()), message))
            }
            InputType::Generated(ident) => Err(Error::new(
                span.unwrap_or_else(|| value.span()),
                format_args!("input type already specified as {}", ident),
            )),
        }
    }

    pub fn output_ty(&self) -> &OutputType {
        &self.output_ty
    }

    pub fn try_set_output_ty(
        &mut self,
        value: OutputType,
        span: Option<Span>,
    ) -> Result<(), Error> {
        match &self.output_ty {
            OutputType::Reflexive => {
                self.output_ty = value;
                Ok(())
            }
            OutputType::Custom(ty) => Err(Error::new(
                span.unwrap_or_else(|| value.span()),
                format_args!("output type already specified as {}", ty.to_token_stream()),
            )),
        }
    }

    pub fn input_vis(&self) -> Option<&Visibility> {
        self.input_vis.get()
    }

    pub fn try_set_input_vis(
        &mut self,
        value: Visibility,
        span: Option<Span>,
    ) -> Result<(), Error> {
        let span = span.unwrap_or_else(|| value.span());
        self.input_vis.try_set(Some(value), span)
    }
}
