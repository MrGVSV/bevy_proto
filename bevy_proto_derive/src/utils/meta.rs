use std::fmt::{Arguments, Formatter};
use syn::meta::ParseNestedMeta;
use syn::{Error, LitBool, Token};

/// Parses an attribute as a `bool` value.
///
/// This will return `true` if the attribute is empty (i.e. `#[foo]`),
/// otherwise it will parse the value as a `bool` (i.e. `#[foo = false]`).
pub(crate) fn parse_bool(meta: &ParseNestedMeta) -> Result<bool, Error> {
    Ok(!meta.input.peek(Token![=]) || meta.value()?.parse::<LitBool>()?.value())
}

/// Helper function for implementing [`Debug`] on attributes.
///
/// The callback will write out whatever it's given while automatically
/// adding commas between each entry.
///
/// This does not automatically add opening and closing parentheses.
pub(crate) fn debug_attribute(
    f: &mut Formatter,
    func: impl FnOnce(&mut dyn FnMut(Arguments) -> std::fmt::Result) -> std::fmt::Result,
) -> std::fmt::Result {
    let mut needs_comma = false;
    let mut write = |args: Arguments| -> std::fmt::Result {
        if needs_comma {
            write!(f, ", ")?;
        }

        write!(f, "{}", args)?;
        needs_comma = true;

        Ok(())
    };

    func(&mut write)
}

/// Returns an error for an unsupported attribute argument.
pub(crate) fn unsupported_arg(meta: &ParseNestedMeta, expected: Option<&[&str]>) -> Error {
    if let Some(expected) = expected {
        meta.error(format_args!(
            "unsupported argument, expected one of: {:?}",
            expected
        ))
    } else {
        meta.error("unsupported argument")
    }
}

/// Helper macro for quickly parsing nested attributes and returning an error
/// if the attribute is not supported.
macro_rules! parse_nested_meta {
    ($attr: ident, |$meta: ident| { $($arg: expr => $expr: expr $(,)?)+ }) => {
        $attr.parse_nested_meta(|$meta| {
            parse_nested_meta!(|$meta| { $($arg => $expr)+ })
        })
    };
    (|$meta: ident| { $($arg: expr => $expr: expr $(,)?)+ }) => {{
        $(
            if $meta.path.is_ident($arg) {
                return $expr;
            }
        )+

        Err($crate::utils::unsupported_arg(&$meta, Some(&[ $($arg),+ ])))
    }};
}

pub(crate) use parse_nested_meta;
