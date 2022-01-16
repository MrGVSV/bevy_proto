/// Taken from [`serde`](https://github.com/serde-rs/serde/blob/c3c1641c06f4baaa06ce9627071a4f2169b770ec/serde_derive/src/internals/symbol.rs#L5)
#[derive(Copy, Clone)]
pub(crate) struct Symbol(&'static str);

impl<'a> PartialEq<Symbol> for &'a syn::Path {
	fn eq(&self, ident: &Symbol) -> bool {
		self.is_ident(ident.0)
	}
}

impl PartialEq<Symbol> for syn::Path {
	fn eq(&self, ident: &Symbol) -> bool {
		self.is_ident(ident.0)
	}
}

pub(crate) const WITH_IDENT: Symbol = Symbol("with");
pub(crate) const INTO_IDENT: Symbol = Symbol("into");
