use proc_macro2::Span;
use quote::ToTokens;
use std::fmt::{Debug, Formatter};
use syn::Error;

/// Defines the basic information for an attribute argument.
pub(crate) trait AttrArg {
    /// The name of the attribute argument.
    const NAME: &'static str;
    /// The target of the attribute argument.
    ///
    /// This is used for error messages.
    const TARGET: AttrTarget = AttrTarget::Field;
}

/// Defines common value-handling methods for an attribute argument.
pub(crate) trait AttrArgValue: AttrArg + Debug {
    /// The type of the value this argument expects.
    type Value: ToTokens;

    fn new(value: Option<Self::Value>) -> Self;

    fn get(&self) -> Option<&Self::Value>;

    fn set(&mut self, value: Option<Self::Value>);

    /// Tries to set the value of this argument.
    ///
    /// Returns an error if the argument is already configured.
    fn try_set(&mut self, value: Option<Self::Value>, span: Span) -> Result<(), Error> {
        if self.get().is_some() {
            Err(self.already_configured_error(value.as_ref(), span))
        } else {
            self.set(value);
            Ok(())
        }
    }

    /// Generates an error for when this argument is already configured.
    #[allow(unused_variables)]
    fn already_configured_error(&self, new: Option<&Self::Value>, span: Span) -> Error {
        Error::new(
            span,
            format_args!("{:?} already configured as `{:?}`", Self::TARGET, self),
        )
    }

    /// Generates an error for when this argument is used with another argument
    /// that it cannot be used with.
    fn invariant_error<T: AttrArg>(&self, span: Span) -> Error {
        Error::new(
            span,
            format_args!("cannot use `{}` argument with `{}`", Self::NAME, T::NAME,),
        )
    }
}

/// The target of the attribute argument.
///
/// This is used for error messages.
#[derive(Default)]
pub(crate) enum AttrTarget {
    #[default]
    Field,
    Asset,
    Input,
    InputVisibility,
}

impl Debug for AttrTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AttrTarget::Field => write!(f, "field"),
            AttrTarget::Asset => write!(f, "asset"),
            AttrTarget::Input => write!(f, "input"),
            AttrTarget::InputVisibility => write!(f, "input visibility"),
        }
    }
}

/// Helper macro for defining an attribute argument.
///
/// Generates the implementations for [`AttrArg`], [`AttrArgValue`], and (optionally) [`Debug`].
macro_rules! define_attribute {
    // Generates the implementations for `AttrArg` and `AttrArgValue`, but not `Debug`.
    ($name: literal => $ident: ident($ty: ty) for $target: expr, no_debug) => {
        #[derive(Default)]
        pub(crate) struct $ident(Option<$ty>);

        impl $crate::utils::AttrArg for $ident {
            const NAME: &'static str = $name;
            const TARGET: $crate::utils::AttrTarget = $target;
        }

        impl $crate::utils::AttrArgValue for $ident {
            type Value = $ty;

            fn new(value: Option<Self::Value>) -> Self {
                Self(value)
            }

            fn get(&self) -> Option<&Self::Value> {
                self.0.as_ref()
            }

            fn set(&mut self, value: Option<Self::Value>) {
                self.0 = value;
            }
        }
    };
    // Generates the implementations for `AttrArg`, `AttrArgValue`, and `Debug`.
    ($name: literal => $ident: ident($ty: ty) for $target: expr $(,)?) => {
        $crate::utils::define_attribute!($name => $ident($ty) for $target, no_debug);

        impl std::fmt::Debug for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match $crate::utils::AttrArgValue::get(self) {
                    None => Ok(()),
                    Some(value) => write!(
                        f,
                        "{} = {}",
                        <Self as $crate::utils::AttrArg>::NAME,
                        ::quote::ToTokens::to_token_stream(value)
                    ),
                }
            }
        }
    };
}

pub(crate) use define_attribute;
