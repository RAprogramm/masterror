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
    F: Fn(&TemplatePlaceholderSpec) -> Result<TokenStream, Error>
{
    let mut pieces = Vec::new();
    for segment in &template.segments {
        match segment {
            TemplateSegmentSpec::Literal(text) => {
                pieces.push(quote! { f.write_str(#text)?; });
            }
            TemplateSegmentSpec::Placeholder(placeholder) => {
                let expr = resolver(placeholder)?;
                pieces.push(format_placeholder(expr, placeholder.formatter));
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
) -> Result<TokenStream, Error> {
    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) if name == "self" => Ok(quote!(self)),
        TemplateIdentifierSpec::Named(name) => {
            if let Some(field) = fields.get_named(name) {
                let member = &field.member;
                Ok(quote!(&self.#member))
            } else {
                Err(placeholder_error(placeholder.span, &placeholder.identifier))
            }
        }
        TemplateIdentifierSpec::Positional(index) => {
            if let Some(field) = fields.get_positional(*index) {
                let member = &field.member;
                Ok(quote!(&self.#member))
            } else {
                Err(placeholder_error(placeholder.span, &placeholder.identifier))
            }
        }
    }
}

fn variant_tuple_placeholder(
    bindings: &[Ident],
    placeholder: &TemplatePlaceholderSpec
) -> Result<TokenStream, Error> {
    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) if name == "self" => Ok(quote!(self)),
        TemplateIdentifierSpec::Named(_) => {
            Err(placeholder_error(placeholder.span, &placeholder.identifier))
        }
        TemplateIdentifierSpec::Positional(index) => bindings
            .get(*index)
            .map(|binding| quote!(#binding))
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier))
    }
}

fn variant_named_placeholder(
    fields: &[Field],
    bindings: &[Ident],
    placeholder: &TemplatePlaceholderSpec
) -> Result<TokenStream, Error> {
    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) if name == "self" => Ok(quote!(self)),
        TemplateIdentifierSpec::Named(name) => {
            if let Some(index) = fields
                .iter()
                .position(|field| field.ident.as_ref().is_some_and(|ident| ident == name))
            {
                let binding = &bindings[index];
                Ok(quote!(#binding))
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

fn format_placeholder(expr: TokenStream, formatter: TemplateFormatter) -> TokenStream {
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
        }
    }
}

fn binding_ident(field: &Field) -> Ident {
    field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("__field{}", field.index, span = field.span))
}
