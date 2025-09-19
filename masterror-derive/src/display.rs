use masterror_template::template::TemplateFormatter;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::Error;

use crate::{
    input::{
        DisplaySpec, ErrorData, ErrorInput, Field, Fields, StructData, VariantData,
        placeholder_error
    },
    template_support::{
        DisplayTemplate, TemplateIdentifierSpec, TemplatePlaceholderSpec, TemplateSegmentSpec
    }
};

pub fn expand(input: &ErrorInput) -> Result<TokenStream, Error> {
    match &input.data {
        ErrorData::Struct(data) => expand_struct(input, data),
        ErrorData::Enum(variants) => expand_enum(input, variants)
    }
}

fn expand_struct(input: &ErrorInput, data: &StructData) -> Result<TokenStream, Error> {
    let body = match &data.display {
        DisplaySpec::Transparent {
            ..
        } => render_struct_transparent(&data.fields),
        DisplaySpec::Template(template) => render_template(template, |placeholder| {
            struct_placeholder_expr(&data.fields, placeholder)
        })?
    };

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics core::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                #body
            }
        }
    })
}

fn expand_enum(input: &ErrorInput, variants: &[VariantData]) -> Result<TokenStream, Error> {
    let mut arms = Vec::new();

    for variant in variants {
        arms.push(render_variant(variant)?);
    }

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics core::fmt::Display for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    #(#arms),*
                }
            }
        }
    })
}

fn render_struct_transparent(fields: &Fields) -> TokenStream {
    if let Some(field) = fields.iter().next() {
        let member = &field.member;
        quote! {
            core::fmt::Display::fmt(&self.#member, f)
        }
    } else {
        quote! {
            Ok(())
        }
    }
}

fn render_variant(variant: &VariantData) -> Result<TokenStream, Error> {
    match &variant.display {
        DisplaySpec::Transparent {
            ..
        } => render_variant_transparent(variant),
        DisplaySpec::Template(template) => render_variant_template(variant, template)
    }
}

#[derive(Debug)]
struct ResolvedPlaceholderExpr {
    expr:          TokenStream,
    pointer_value: bool
}

impl ResolvedPlaceholderExpr {
    fn new(expr: TokenStream) -> Self {
        Self::with(expr, false)
    }

    fn pointer(expr: TokenStream) -> Self {
        Self::with(expr, true)
    }

    fn with(expr: TokenStream, pointer_value: bool) -> Self {
        Self {
            expr,
            pointer_value
        }
    }
}

fn render_variant_transparent(variant: &VariantData) -> Result<TokenStream, Error> {
    let variant_ident = &variant.ident;

    match &variant.fields {
        Fields::Unit => Err(Error::new(
            variant.span,
            "#[error(transparent)] requires exactly one field"
        )),
        Fields::Named(fields) | Fields::Unnamed(fields) => {
            if fields.len() != 1 {
                return Err(Error::new(
                    variant.span,
                    "#[error(transparent)] requires exactly one field"
                ));
            }

            let binding = binding_ident(&fields[0]);
            let pattern = match &variant.fields {
                Fields::Named(_) => {
                    let field_ident = fields[0].ident.clone().expect("named field");
                    quote!(Self::#variant_ident { #field_ident: #binding })
                }
                Fields::Unnamed(_) => {
                    quote!(Self::#variant_ident(#binding))
                }
                Fields::Unit => unreachable!()
            };

            Ok(quote! {
                #pattern => core::fmt::Display::fmt(#binding, f)
            })
        }
    }
}

fn render_variant_template(
    variant: &VariantData,
    template: &DisplayTemplate
) -> Result<TokenStream, Error> {
    let variant_ident = &variant.ident;

    match &variant.fields {
        Fields::Unit => {
            let body = render_template(template, |_placeholder| {
                Err(Error::new(
                    variant.span,
                    "unit variants cannot reference fields"
                ))
            })?;
            Ok(quote! {
                Self::#variant_ident => {
                    #body
                }
            })
        }
        Fields::Unnamed(fields) => {
            let bindings: Vec<_> = fields.iter().map(binding_ident).collect();
            let pattern = quote!(Self::#variant_ident(#(#bindings),*));
            let body = render_template(template, |placeholder| {
                variant_tuple_placeholder(&bindings, placeholder)
            })?;
            Ok(quote! {
                #pattern => {
                    #body
                }
            })
        }
        Fields::Named(fields) => {
            let bindings: Vec<_> = fields
                .iter()
                .map(|field| field.ident.clone().expect("named field"))
                .collect();
            let pattern = quote!(Self::#variant_ident { #(#bindings),* });
            let body = render_template(template, |placeholder| {
                variant_named_placeholder(fields, &bindings, placeholder)
            })?;
            Ok(quote! {
                #pattern => {
                    #body
                }
            })
        }
    }
}

fn render_template<F>(template: &DisplayTemplate, resolver: F) -> Result<TokenStream, Error>
where
    F: Fn(&TemplatePlaceholderSpec) -> Result<ResolvedPlaceholderExpr, Error>
{
    let mut pieces = Vec::new();
    for segment in &template.segments {
        match segment {
            TemplateSegmentSpec::Literal(text) => {
                pieces.push(quote! { f.write_str(#text)?; });
            }
            TemplateSegmentSpec::Placeholder(placeholder) => {
                let resolved = resolver(placeholder)?;
                pieces.push(format_placeholder(resolved, placeholder.formatter));
            }
        }
    }
    pieces.push(quote! { Ok(()) });

    Ok(quote! {
        #(#pieces)*
    })
}

fn struct_placeholder_expr(
    fields: &Fields,
    placeholder: &TemplatePlaceholderSpec
) -> Result<ResolvedPlaceholderExpr, Error> {
    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) if name == "self" => {
            Ok(ResolvedPlaceholderExpr::with(
                quote!(self),
                needs_pointer_value(placeholder.formatter)
            ))
        }
        TemplateIdentifierSpec::Named(name) => {
            if let Some(field) = fields.get_named(name) {
                Ok(struct_field_expr(field, placeholder.formatter))
            } else {
                Err(placeholder_error(placeholder.span, &placeholder.identifier))
            }
        }
        TemplateIdentifierSpec::Positional(index) => fields
            .get_positional(*index)
            .map(|field| struct_field_expr(field, placeholder.formatter))
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier))
    }
}

fn struct_field_expr(field: &Field, formatter: TemplateFormatter) -> ResolvedPlaceholderExpr {
    let member = &field.member;

    if needs_pointer_value(formatter) && pointer_prefers_value(&field.ty) {
        ResolvedPlaceholderExpr::pointer(quote!(self.#member))
    } else {
        ResolvedPlaceholderExpr::new(quote!(&self.#member))
    }
}

fn needs_pointer_value(formatter: TemplateFormatter) -> bool {
    matches!(formatter, TemplateFormatter::Pointer { .. })
}

fn pointer_prefers_value(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Ptr(_) => true,
        syn::Type::Reference(reference) => reference.mutability.is_none(),
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident == "NonNull")
            .unwrap_or(false),
        _ => false
    }
}

fn variant_tuple_placeholder(
    bindings: &[Ident],
    placeholder: &TemplatePlaceholderSpec
) -> Result<ResolvedPlaceholderExpr, Error> {
    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) if name == "self" => {
            Ok(ResolvedPlaceholderExpr::with(
                quote!(self),
                needs_pointer_value(placeholder.formatter)
            ))
        }
        TemplateIdentifierSpec::Named(_) => {
            Err(placeholder_error(placeholder.span, &placeholder.identifier))
        }
        TemplateIdentifierSpec::Positional(index) => bindings
            .get(*index)
            .map(|binding| {
                ResolvedPlaceholderExpr::with(
                    quote!(#binding),
                    needs_pointer_value(placeholder.formatter)
                )
            })
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier))
    }
}

fn variant_named_placeholder(
    fields: &[Field],
    bindings: &[Ident],
    placeholder: &TemplatePlaceholderSpec
) -> Result<ResolvedPlaceholderExpr, Error> {
    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) if name == "self" => {
            Ok(ResolvedPlaceholderExpr::with(
                quote!(self),
                needs_pointer_value(placeholder.formatter)
            ))
        }
        TemplateIdentifierSpec::Named(name) => {
            if let Some(index) = fields
                .iter()
                .position(|field| field.ident.as_ref().is_some_and(|ident| ident == name))
            {
                let binding = &bindings[index];
                Ok(ResolvedPlaceholderExpr::with(
                    quote!(#binding),
                    needs_pointer_value(placeholder.formatter)
                ))
            } else {
                Err(placeholder_error(placeholder.span, &placeholder.identifier))
            }
        }
        TemplateIdentifierSpec::Positional(index) => Err(placeholder_error(
            placeholder.span,
            &TemplateIdentifierSpec::Positional(*index)
        ))
    }
}

fn format_placeholder(
    resolved: ResolvedPlaceholderExpr,
    formatter: TemplateFormatter
) -> TokenStream {
    let ResolvedPlaceholderExpr {
        expr,
        pointer_value
    } = resolved;

    match formatter {
        TemplateFormatter::Display => quote! {
            core::fmt::Display::fmt(#expr, f)?;
        },
        TemplateFormatter::Debug {
            alternate: false
        } => quote! {
            core::fmt::Debug::fmt(#expr, f)?;
        },
        TemplateFormatter::Debug {
            alternate: true
        } => quote! {
            write!(f, "{:#?}", #expr)?;
        },
        TemplateFormatter::LowerHex {
            alternate
        } => {
            if alternate {
                quote! { write!(f, "{:#x}", #expr)?; }
            } else {
                quote! { core::fmt::LowerHex::fmt(#expr, f)?; }
            }
        }
        TemplateFormatter::UpperHex {
            alternate
        } => {
            if alternate {
                quote! { write!(f, "{:#X}", #expr)?; }
            } else {
                quote! { core::fmt::UpperHex::fmt(#expr, f)?; }
            }
        }
        TemplateFormatter::Pointer {
            alternate
        } => {
            if alternate {
                quote! { write!(f, "{:#p}", #expr)?; }
            } else if pointer_value {
                quote! {{
                    let value = #expr;
                    core::fmt::Pointer::fmt(&value, f)?;
                }}
            } else {
                quote! { core::fmt::Pointer::fmt(#expr, f)?; }
            }
        }
        TemplateFormatter::Binary {
            alternate
        } => {
            if alternate {
                quote! { write!(f, "{:#b}", #expr)?; }
            } else {
                quote! { core::fmt::Binary::fmt(#expr, f)?; }
            }
        }
        TemplateFormatter::Octal {
            alternate
        } => {
            if alternate {
                quote! { write!(f, "{:#o}", #expr)?; }
            } else {
                quote! { core::fmt::Octal::fmt(#expr, f)?; }
            }
        }
        TemplateFormatter::LowerExp {
            alternate
        } => {
            if alternate {
                quote! { write!(f, "{:#e}", #expr)?; }
            } else {
                quote! { core::fmt::LowerExp::fmt(#expr, f)?; }
            }
        }
        TemplateFormatter::UpperExp {
            alternate
        } => {
            if alternate {
                quote! { write!(f, "{:#E}", #expr)?; }
            } else {
                quote! { core::fmt::UpperExp::fmt(#expr, f)?; }
            }
        }
    }
}

fn binding_ident(field: &Field) -> Ident {
    field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("__field{}", field.index, span = field.span))
}
