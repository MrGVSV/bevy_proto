use syn::Attribute;

use crate::schematic::ATTRIBUTE;

/// Filters the given attributes down to only the attributes used by [`Schematic`].
///
/// [`Schematic`]: crate::Schematic
pub(crate) fn filter_attributes<'a>(
    attrs: impl IntoIterator<Item = &'a Attribute>,
) -> impl Iterator<Item = &'a Attribute> {
    attrs
        .into_iter()
        .filter(|attr| attr.path().is_ident(ATTRIBUTE))
}
