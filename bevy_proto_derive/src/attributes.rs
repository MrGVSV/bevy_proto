use proc_macro2::{Ident, Span};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

use syn::{
    Attribute, Data, DataEnum, DataStruct, Error, ExprPath, Fields, Index, Lit, LitStr, Member,
    Meta, NestedMeta, Path, Result, Token, Type, Variant,
};

const PROTO_COMP: &str = "proto_comp";
const WITH_IDENT: &str = "with";
const INTO_IDENT: &str = "into";
const PRELOAD_IDENT: &str = "preload";

const VALID_ATTRS: &[&str] = &[WITH_IDENT, INTO_IDENT];
const VALID_FIELD_ATTRS: &[&str] = &[PRELOAD_IDENT];

mod keywords {
    syn::custom_keyword!(preload);
}

/// ProtoComponent attributes applied on structs and enums
pub(crate) enum ProtoCompAttr {
    /// Captures the `#[proto_comp(into = "ActualComponent")]` attribute.
    ///
    /// This is used to specify a separate Component that this marked struct will be cloned into.
    ///
    /// Generates the following code:
    /// ```ignore
    /// let component: ActualComponent = self.clone().into();
    /// commands.insert(component);
    /// ```
    /// # Example
    ///
    /// ```ignore
    /// mod external {
    ///   #[derive(Component)]
    ///   struct SomeOtherComponent {
    ///     count: i32
    ///   }
    /// }
    ///
    /// // Don't forget to bring it into scope!
    /// use external::SomeOtherComponent;
    ///
    /// #[derive(ProtoComponent)]
    /// #[into = "SomeOtherComponent"]
    /// struct MyComponent {
    ///   foo: i32
    /// }
    ///
    /// impl From<MyComponent> for SomeOtherComponent {
    ///   fn from(component: MyComponent) -> Self {
    ///     SomeOtherComponent {
    ///       count: component.foo
    ///     }
    ///   }
    /// }
    /// ```
    Into(Type),
    /// Captures the `#[proto_comp(with = "my_function")]` attribute.
    ///
    /// This is used to specify a custom function with which custom Components will be created and/or inserted.
    /// This is essentially identical to just simply implementing `ProtoComponent` yourself, but allows some of
    /// the process to be automated by the derive.
    ///
    /// Generates the following code:
    /// ```ignore
    /// my_function(self, entity);
    /// ```
    ///
    /// # Example
    ///
    /// ```ignore
    /// mod external {
    ///   #[derive(Component)]
    ///   struct SomeOtherComponent {
    ///     count: i32
    ///   }
    /// }
    ///
    /// use external::SomeOtherComponent;
    ///
    /// #[derive(ProtoComponent)]
    /// #[with = "my_function"]
    /// struct MyComponent {
    ///   foo: i32
    /// }
    ///
    /// fn my_function(component: &MyComponent, entity: &mut EntityMut) {
    ///   entity.insert(SomeOtherComponent {
    ///     count: component.foo
    ///   });
    /// }
    /// ```
    With(ExprPath),
}

/// Represents a ProtoComponent field on both structs and enums
pub(crate) struct ProtoCompField {
    attrs: Vec<ProtoCompFieldAttr>,
    member: Member,
    variant: Option<Variant>,
}

/// ProtoComponent field attributes applied on structs and enums
pub(crate) enum ProtoCompFieldAttr {
    /// Captures the `#[proto_comp(preload(type = "Image", dest = "some_field"))]` attribute.
    ///
    /// This is used to mark a field as containing the path to an asset that should be preloaded.
    Preload {
        /// _(Required)_ The asset's type
        asset_type: Type,
        /// _(Optional)_ The name of the field to save the handle to. If `None`, adds the
        /// preloaded asset as a dependency of the entire prototype
        dest: Option<Member>,
    },
}

impl Parse for ProtoCompAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let path: Path = input.parse()?;

        if path.is_ident(WITH_IDENT) {
            let _: Token![=] = input.parse()?;
            let item: LitStr = input.parse()?;
            Ok(Self::With(item.parse()?))
        } else if path.is_ident(INTO_IDENT) {
            let _: Token![=] = input.parse()?;
            let item: LitStr = input.parse()?;
            Ok(Self::Into(item.parse()?))
        } else {
            Err(Error::new(
                Span::call_site(),
                format!("invalid attribute, expected one of: {:?}", VALID_ATTRS),
            ))
        }
    }
}

impl Parse for ProtoCompFieldAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(keywords::preload) {
            parse_preload(input)
        } else {
            Err(syn::Error::new(
                input.span(),
                format!(
                    "invalid attribute, expected one of: {:?}",
                    VALID_FIELD_ATTRS
                ),
            ))
        }
    }
}

impl ProtoCompField {
    /// Method constructing a `TokenStream` that performs an action on a particular field.
    ///
    /// This is needed so that we can support enums (where we can't just do `self.foo`).
    ///
    /// # Examples
    ///
    /// For structs, this might result in something like:
    ///
    /// ```ignore
    /// let field_temp_ident = &self.field_member_ident;
    /// do_something(field_temp_ident);
    /// ```
    ///
    /// And for enums:
    ///
    /// ```ignore
    /// if let Self::A {field_temp_ident, ..} = &self {
    ///   do_something(field_temp_ident);
    /// }
    /// ```
    pub fn on_field(
        &self,
        field_temp_ident: &Ident,
        field_member: &Member,
        func: proc_macro2::TokenStream,
        is_mut: bool,
    ) -> proc_macro2::TokenStream {
        let self_ref = if is_mut {
            quote!(&mut self)
        } else {
            quote!(&self)
        };

        if let Some(variant) = &self.variant {
            // === Handle Enum === //
            let ident = &variant.ident;
            match field_member {
                Member::Named(member) => {
                    quote! {
                        if let Self::#ident {#member, ..} = #self_ref {
                            let #field_temp_ident = #member;
                            #func
                        }
                    }
                }
                Member::Unnamed(Index { index, .. }) => {
                    // This creates the necessary number of underscores to get to the desired index
                    // So to get item at index 3, we generate: `_, _, _,`
                    let underscores = vec![quote!(_); *index as usize];
                    quote! {
                        if let Self::#ident (#(#underscores,)* #field_temp_ident, ..) = #self_ref {
                            #func
                        }
                    }
                }
            }
        } else {
            // === Handle Struct === //
            quote! {
                let #field_temp_ident = #self_ref.#field_member;
                #func
            }
        }
    }

    /// Collects the generated preloader `TokenStream`s for the given field
    pub fn get_preloaders(&self, preloader_ident: &Ident) -> Vec<proc_macro2::TokenStream> {
        let field_ident = format_ident!("_{}", self.member);

        self.attrs.iter().filter_map(|attr| {
            match attr {
                ProtoCompFieldAttr::Preload {asset_type, dest} => {
                    let preloader = if let Some(ref member) = dest {
                        let dest_ident = format_ident!("_{}", member);

                        let store = self.on_field(
                            &dest_ident,
                            &member,
                            quote! {
                        #dest_ident.store_handle(handle);
                    },
                            true
                        );
                        self.on_field(&field_ident, &self.member, quote! {
                    let handle: bevy::prelude::Handle<#asset_type> = #preloader_ident.preload(#field_ident.clone());
                    #store
                }, false)
                    } else {
                        self.on_field(&field_ident, &self.member, quote! {
                    let _: bevy::prelude::Handle<#asset_type> = #preloader_ident.preload_dependency(#field_ident.clone());
                }, false)
                    };
                    Some(preloader)
                },
                // Allowed in case others are added
                #[allow(unreachable_patterns)]
                _ => None
            }
        }).collect()
    }
}

pub(crate) fn parse_attrs(attrs: &[Attribute]) -> Result<Vec<ProtoCompAttr>> {
    let attrs = filter_attributes(attrs);

    let mut proto_attrs = Vec::new();
    for attr in attrs {
        proto_attrs.push(attr.parse_args()?)
    }

    Ok(proto_attrs)
}

pub(crate) fn parse_fields(data: &Data) -> Result<Vec<ProtoCompField>> {
    Ok(match data {
        Data::Struct(DataStruct { fields, .. }) => get_proto_fields(&fields, None)?,
        Data::Enum(DataEnum { variants, .. }) => {
            let mut attrs = Vec::new();
            for variant in variants {
                attrs.extend(get_proto_fields(&variant.fields, Some(variant))?);
            }
            attrs
        }
        _ => {
            return Err(syn::Error::new(
                Span::call_site(),
                "ProtoComponent can only be applied on struct or enum types",
            ));
        }
    })
}

fn get_proto_fields(fields: &Fields, variant: Option<&Variant>) -> Result<Vec<ProtoCompField>> {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            let member = if let Some(ref ident) = field.ident {
                Member::Named(ident.clone())
            } else {
                Member::Unnamed(Index::from(index))
            };
            let attrs = filter_attributes(&field.attrs);

            let mut proto_attrs = Vec::new();
            for attr in attrs {
                proto_attrs.push(attr.parse_args()?);
            }

            Ok(ProtoCompField {
                member,
                attrs: proto_attrs,
                variant: variant.map(|v| v.clone()),
            })
        })
        .collect()
}

fn filter_attributes(attrs: &[Attribute]) -> impl Iterator<Item = &Attribute> {
    attrs.iter().filter(|attr| attr.path.is_ident(PROTO_COMP))
}

fn parse_preload(input: ParseStream) -> Result<ProtoCompFieldAttr> {
    let meta = input.parse::<Meta>()?;
    let mut asset_type: Option<Type> = None;
    let mut dest: Option<Member> = None;
    match &meta {
        Meta::List(list) => {
            for nested_meta in &list.nested {
                let arg_meta = if let NestedMeta::Meta(arg_meta) = nested_meta {
                    arg_meta
                } else {
                    return Err(syn::Error::new(
                        nested_meta.span(),
                        "Expected one or more arguments: `type`, `dest`",
                    ));
                };
                match arg_meta {
                    Meta::NameValue(pair) => {
                        if pair.path.is_ident("type") {
                            match &pair.lit {
                                Lit::Str(path) => {
                                    asset_type = Some(path.parse()?);
                                }
                                etc => {
                                    return Err(syn::Error::new(
                                        etc.span(),
                                        "expected a string referencing an in-scope type",
                                    ));
                                }
                            }
                        } else if pair.path.is_ident("dest") {
                            match &pair.lit {
                                Lit::Str(path) => {
                                    dest = Some(path.parse()?);
                                }
                                etc => {
                                    return Err(syn::Error::new(
                                        etc.span(),
                                        "expected the name of a field",
                                    ));
                                }
                            }
                        }
                    }
                    etc => {
                        return Err(syn::Error::new(
                            etc.span(),
                            "expected one or more arguments: `preload(type=...[, dest=...])`",
                        ));
                    }
                }
            }
        }
        etc => {
            return Err(syn::Error::new(
                etc.span(),
                "invalid syntax for attribute `preload`, expected: `preload(type=...[, dest=...])`",
            ));
        }
    }

    if let Some(asset_type) = asset_type {
        Ok(ProtoCompFieldAttr::Preload { asset_type, dest })
    } else {
        Err(syn::Error::new(meta.span(), "missing argument: `type`"))
    }
}
