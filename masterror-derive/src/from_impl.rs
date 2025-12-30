// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

use crate::input::{
    ErrorData, ErrorInput, Field, Fields, StructData, VariantData, is_option_type
};

pub fn expand(input: &ErrorInput) -> Result<Vec<TokenStream>, Error> {
    let mut impls = Vec::new();
    match &input.data {
        ErrorData::Struct(data) => {
            if let Some(field) = data.fields.first_from_field() {
                impls.push(struct_from_impl(input, data, field)?);
            }
        }
        ErrorData::Enum(variants) => {
            for variant in variants {
                if let Some(field) = variant.fields.first_from_field() {
                    impls.push(enum_from_impl(input, variant, field)?);
                }
            }
        }
    }
    Ok(impls)
}

fn struct_from_impl(
    input: &ErrorInput,
    data: &StructData,
    field: &Field
) -> Result<TokenStream, Error> {
    let ident = &input.ident;
    let ty = &field.ty;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let constructor = struct_constructor(&data.fields, field)?;
    Ok(quote! {
        impl #impl_generics core::convert::From<#ty> for #ident #ty_generics #where_clause {
            fn from(value: #ty) -> Self {
                #constructor
            }
        }
    })
}

fn enum_from_impl(
    input: &ErrorInput,
    variant: &VariantData,
    field: &Field
) -> Result<TokenStream, Error> {
    let ident = &input.ident;
    let ty = &field.ty;
    let variant_ident = &variant.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let constructor = variant_constructor(variant_ident, &variant.fields, field)?;
    Ok(quote! {
        impl #impl_generics core::convert::From<#ty> for #ident #ty_generics #where_clause {
            fn from(value: #ty) -> Self {
                #constructor
            }
        }
    })
}

fn struct_constructor(fields: &Fields, from_field: &Field) -> Result<TokenStream, Error> {
    match fields {
        Fields::Named(named) => {
            let mut initializers = Vec::new();
            for field in named {
                let field_ident = field.ident.clone().expect("named field");
                let value = field_value_expr(field, from_field)?;
                initializers.push(quote! { #field_ident: #value });
            }
            Ok(quote! { Self { #(#initializers),* } })
        }
        Fields::Unnamed(unnamed) => {
            let mut values = Vec::new();
            for field in unnamed {
                values.push(field_value_expr(field, from_field)?);
            }
            Ok(quote! { Self(#(#values),*) })
        }
        Fields::Unit => Err(Error::new(
            from_field.span,
            "#[from] is not supported on unit structs"
        ))
    }
}

fn variant_constructor(
    variant_ident: &syn::Ident,
    fields: &Fields,
    from_field: &Field
) -> Result<TokenStream, Error> {
    match fields {
        Fields::Named(named) => {
            let mut initializers = Vec::new();
            for field in named {
                let field_ident = field.ident.clone().expect("named field");
                let value = field_value_expr(field, from_field)?;
                initializers.push(quote! { #field_ident: #value });
            }
            Ok(quote! { Self::#variant_ident { #(#initializers),* } })
        }
        Fields::Unnamed(unnamed) => {
            let mut values = Vec::new();
            for field in unnamed {
                values.push(field_value_expr(field, from_field)?);
            }
            Ok(quote! { Self::#variant_ident(#(#values),*) })
        }
        Fields::Unit => Err(Error::new(
            from_field.span,
            "#[from] is not supported on unit variants"
        ))
    }
}

fn field_value_expr(field: &Field, from_field: &Field) -> Result<TokenStream, Error> {
    if field.index == from_field.index {
        return Ok(quote! { value });
    }
    if field.attrs.has_backtrace() {
        return Ok(backtrace_initializer(field));
    }
    if field.attrs.has_source() && field.attrs.from.is_none() {
        return source_initializer(field);
    }
    Err(Error::new(
        field.span,
        "deriving From requires no fields other than source and backtrace"
    ))
}

fn source_initializer(field: &Field) -> Result<TokenStream, Error> {
    if is_option_type(&field.ty) {
        Ok(quote! { ::core::option::Option::None })
    } else {
        Err(Error::new(
            field.span,
            "additional #[source] fields used with #[from] must be Option<_>"
        ))
    }
}

fn backtrace_initializer(field: &Field) -> TokenStream {
    let capture = quote! { ::std::backtrace::Backtrace::capture() };
    if is_option_type(&field.ty) {
        quote! {
            ::core::option::Option::Some(::core::convert::From::from(#capture))
        }
    } else {
        quote! {
            ::core::convert::From::from(#capture)
        }
    }
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::parse_quote;

    use super::*;
    use crate::{
        input::{DisplaySpec, FieldAttrs, FormatArgsSpec, StructData, VariantData},
        template_support::{DisplayTemplate, TemplateSegmentSpec}
    };

    fn make_field(index: usize, ident: Option<&str>, ty: syn::Type, attrs: FieldAttrs) -> Field {
        let ident = ident.map(|s| syn::Ident::new(s, Span::call_site()));
        let member = match &ident {
            Some(name) => syn::Member::Named(name.clone()),
            None => syn::Member::Unnamed(syn::Index::from(index))
        };
        Field {
            ident,
            member,
            ty,
            attrs,
            span: Span::call_site(),
            index
        }
    }

    fn make_field_attrs_with_from() -> FieldAttrs {
        let attr: syn::Attribute = parse_quote!(#[from]);
        let mut attrs = FieldAttrs::default();
        attrs.from = Some(attr.clone());
        attrs.source = Some(attr);
        attrs
    }

    fn make_field_attrs_with_backtrace() -> FieldAttrs {
        let attr: syn::Attribute = parse_quote!(#[backtrace]);
        let mut attrs = FieldAttrs::default();
        attrs.backtrace = Some(attr);
        attrs
    }

    fn make_field_attrs_with_source() -> FieldAttrs {
        let attr: syn::Attribute = parse_quote!(#[source]);
        let mut attrs = FieldAttrs::default();
        attrs.source = Some(attr);
        attrs
    }

    fn make_field_attrs_plain() -> FieldAttrs {
        FieldAttrs::default()
    }

    fn make_error_input(ident: &str, data: ErrorData) -> ErrorInput {
        ErrorInput {
            ident: syn::Ident::new(ident, Span::call_site()),
            generics: syn::Generics::default(),
            data
        }
    }

    fn make_display_spec() -> DisplaySpec {
        DisplaySpec::Template(DisplayTemplate {
            segments: vec![TemplateSegmentSpec::Literal("error".to_string())]
        })
    }

    #[test]
    fn test_expand_struct_with_from_field() {
        let from_field = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let fields = Fields::Named(vec![from_field]);
        let data = ErrorData::Struct(Box::new(StructData {
            fields,
            display: make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error: None,
            masterror: None
        }));
        let input = make_error_input("MyError", data);
        let result = expand(&input);
        assert!(result.is_ok());
        let impls = result.unwrap();
        assert_eq!(impls.len(), 1);
        let impl_str = impls[0].to_string();
        assert!(impl_str.contains("impl"));
        assert!(impl_str.contains("From"));
        assert!(impl_str.contains("std :: io :: Error"));
    }

    #[test]
    fn test_expand_struct_without_from_field() {
        let field = make_field(
            0,
            Some("message"),
            parse_quote!(String),
            make_field_attrs_plain()
        );
        let fields = Fields::Named(vec![field]);
        let data = ErrorData::Struct(Box::new(StructData {
            fields,
            display: make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error: None,
            masterror: None
        }));
        let input = make_error_input("MyError", data);
        let result = expand(&input);
        assert!(result.is_ok());
        let impls = result.unwrap();
        assert_eq!(impls.len(), 0);
    }

    #[test]
    fn test_expand_enum_with_from_fields() {
        let from_field1 = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let variant1 = VariantData {
            ident:       syn::Ident::new("Io", Span::call_site()),
            fields:      Fields::Named(vec![from_field1]),
            display:     make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };
        let from_field2 = make_field(0, None, parse_quote!(String), make_field_attrs_with_from());
        let variant2 = VariantData {
            ident:       syn::Ident::new("Parse", Span::call_site()),
            fields:      Fields::Unnamed(vec![from_field2]),
            display:     make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };
        let data = ErrorData::Enum(vec![variant1, variant2]);
        let input = make_error_input("MyError", data);
        let result = expand(&input);
        assert!(result.is_ok());
        let impls = result.unwrap();
        assert_eq!(impls.len(), 2);
    }

    #[test]
    fn test_struct_from_impl_named_fields() {
        let from_field = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let from_field_ref = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let fields = Fields::Named(vec![from_field]);
        let struct_data = StructData {
            fields,
            display: make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error: None,
            masterror: None
        };
        let input = make_error_input(
            "MyError",
            ErrorData::Struct(Box::new(StructData {
                fields:      Fields::Named(vec![from_field_ref]),
                display:     make_display_spec(),
                format_args: FormatArgsSpec::default(),
                app_error:   None,
                masterror:   None
            }))
        );
        let result = struct_from_impl(
            &input,
            &struct_data,
            struct_data.fields.iter().next().unwrap()
        );
        assert!(result.is_ok());
        let impl_tokens = result.unwrap().to_string();
        assert!(impl_tokens.contains("From"));
        assert!(impl_tokens.contains("std :: io :: Error"));
        assert!(impl_tokens.contains("source"));
    }

    #[test]
    fn test_struct_from_impl_unnamed_fields() {
        let from_field = make_field(
            0,
            None,
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let fields = Fields::Unnamed(vec![from_field]);
        let struct_data = StructData {
            fields,
            display: make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error: None,
            masterror: None
        };
        let from_field_input = make_field(
            0,
            None,
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let input = make_error_input(
            "MyError",
            ErrorData::Struct(Box::new(StructData {
                fields:      Fields::Unnamed(vec![from_field_input]),
                display:     make_display_spec(),
                format_args: FormatArgsSpec::default(),
                app_error:   None,
                masterror:   None
            }))
        );
        let result = struct_from_impl(
            &input,
            &struct_data,
            struct_data.fields.iter().next().unwrap()
        );
        assert!(result.is_ok());
        let impl_tokens = result.unwrap().to_string();
        assert!(impl_tokens.contains("From"));
        assert!(impl_tokens.contains("Self"));
    }

    #[test]
    fn test_enum_from_impl_named_fields() {
        let from_field = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let variant = VariantData {
            ident:       syn::Ident::new("Io", Span::call_site()),
            fields:      Fields::Named(vec![from_field]),
            display:     make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };
        let from_field_input = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let variant_input = VariantData {
            ident:       syn::Ident::new("Io", Span::call_site()),
            fields:      Fields::Named(vec![from_field_input]),
            display:     make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };
        let input = make_error_input("MyError", ErrorData::Enum(vec![variant_input]));
        let result = enum_from_impl(&input, &variant, variant.fields.iter().next().unwrap());
        assert!(result.is_ok());
        let impl_tokens = result.unwrap().to_string();
        assert!(impl_tokens.contains("From"));
        assert!(impl_tokens.contains("Io"));
    }

    #[test]
    fn test_enum_from_impl_unnamed_fields() {
        let from_field = make_field(
            0,
            None,
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let variant = VariantData {
            ident:       syn::Ident::new("Io", Span::call_site()),
            fields:      Fields::Unnamed(vec![from_field]),
            display:     make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };
        let from_field_input = make_field(
            0,
            None,
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let variant_input = VariantData {
            ident:       syn::Ident::new("Io", Span::call_site()),
            fields:      Fields::Unnamed(vec![from_field_input]),
            display:     make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };
        let input = make_error_input("MyError", ErrorData::Enum(vec![variant_input]));
        let result = enum_from_impl(&input, &variant, variant.fields.iter().next().unwrap());
        assert!(result.is_ok());
        let impl_tokens = result.unwrap().to_string();
        assert!(impl_tokens.contains("From"));
        assert!(impl_tokens.contains("Self :: Io"));
    }

    #[test]
    fn test_struct_constructor_named_fields() {
        let from_field = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let backtrace_field = make_field(
            1,
            Some("backtrace"),
            parse_quote!(Option<std::backtrace::Backtrace>),
            make_field_attrs_with_backtrace()
        );
        let fields = Fields::Named(vec![from_field, backtrace_field]);
        let result = struct_constructor(&fields, fields.iter().next().unwrap());
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        assert!(tokens.contains("Self"));
        assert!(tokens.contains("source"));
        assert!(tokens.contains("backtrace"));
    }

    #[test]
    fn test_struct_constructor_unnamed_fields() {
        let from_field = make_field(
            0,
            None,
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let fields = Fields::Unnamed(vec![from_field]);
        let result = struct_constructor(&fields, fields.iter().next().unwrap());
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        assert!(tokens.contains("Self"));
    }

    #[test]
    fn test_struct_constructor_unit_fails() {
        let from_field = make_field(
            0,
            None,
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let fields = Fields::Unit;
        let result = struct_constructor(&fields, &from_field);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("not supported on unit structs"));
    }

    #[test]
    fn test_variant_constructor_named_fields() {
        let from_field = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let fields = Fields::Named(vec![from_field]);
        let variant_ident = syn::Ident::new("Io", Span::call_site());
        let result = variant_constructor(&variant_ident, &fields, fields.iter().next().unwrap());
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        assert!(tokens.contains("Self :: Io"));
        assert!(tokens.contains("source"));
    }

    #[test]
    fn test_variant_constructor_unnamed_fields() {
        let from_field = make_field(
            0,
            None,
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let backtrace_field = make_field(
            1,
            None,
            parse_quote!(std::backtrace::Backtrace),
            make_field_attrs_with_backtrace()
        );
        let fields = Fields::Unnamed(vec![from_field, backtrace_field]);
        let variant_ident = syn::Ident::new("Io", Span::call_site());
        let result = variant_constructor(&variant_ident, &fields, fields.iter().next().unwrap());
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        assert!(tokens.contains("Self :: Io"));
    }

    #[test]
    fn test_variant_constructor_unit_fails() {
        let from_field = make_field(
            0,
            None,
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let fields = Fields::Unit;
        let variant_ident = syn::Ident::new("Io", Span::call_site());
        let result = variant_constructor(&variant_ident, &fields, &from_field);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("not supported on unit variants"));
    }

    #[test]
    fn test_field_value_expr_from_field() {
        let from_field = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let result = field_value_expr(&from_field, &from_field);
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        assert_eq!(tokens, "value");
    }

    #[test]
    fn test_field_value_expr_backtrace_field() {
        let from_field = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let backtrace_field = make_field(
            1,
            Some("backtrace"),
            parse_quote!(std::backtrace::Backtrace),
            make_field_attrs_with_backtrace()
        );
        let result = field_value_expr(&backtrace_field, &from_field);
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        assert!(tokens.contains("Backtrace :: capture"));
    }

    #[test]
    fn test_field_value_expr_source_field_option() {
        let from_field = make_field(
            0,
            Some("cause"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let source_field = make_field(
            1,
            Some("source"),
            parse_quote!(Option<String>),
            make_field_attrs_with_source()
        );
        let result = field_value_expr(&source_field, &from_field);
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        assert!(tokens.contains("Option :: None"));
    }

    #[test]
    fn test_field_value_expr_source_field_non_option_fails() {
        let from_field = make_field(
            0,
            Some("cause"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let source_field = make_field(
            1,
            Some("source"),
            parse_quote!(String),
            make_field_attrs_with_source()
        );
        let result = field_value_expr(&source_field, &from_field);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("must be Option"));
    }

    #[test]
    fn test_field_value_expr_plain_field_fails() {
        let from_field = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let plain_field = make_field(
            1,
            Some("message"),
            parse_quote!(String),
            make_field_attrs_plain()
        );
        let result = field_value_expr(&plain_field, &from_field);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.to_string()
                .contains("no fields other than source and backtrace")
        );
    }

    #[test]
    fn test_source_initializer_option_type() {
        let source_field = make_field(
            0,
            Some("source"),
            parse_quote!(Option<String>),
            make_field_attrs_with_source()
        );
        let result = source_initializer(&source_field);
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        assert!(tokens.contains("Option :: None"));
    }

    #[test]
    fn test_source_initializer_non_option_type_fails() {
        let source_field = make_field(
            0,
            Some("source"),
            parse_quote!(String),
            make_field_attrs_with_source()
        );
        let result = source_initializer(&source_field);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("must be Option"));
    }

    #[test]
    fn test_backtrace_initializer_option_type() {
        let backtrace_field = make_field(
            0,
            Some("backtrace"),
            parse_quote!(Option<std::backtrace::Backtrace>),
            make_field_attrs_with_backtrace()
        );
        let tokens = backtrace_initializer(&backtrace_field);
        let result = tokens.to_string();
        assert!(result.contains("Option :: Some"));
        assert!(result.contains("Backtrace :: capture"));
    }

    #[test]
    fn test_backtrace_initializer_non_option_type() {
        let backtrace_field = make_field(
            0,
            Some("backtrace"),
            parse_quote!(std::backtrace::Backtrace),
            make_field_attrs_with_backtrace()
        );
        let tokens = backtrace_initializer(&backtrace_field);
        let result = tokens.to_string();
        assert!(result.contains("Backtrace :: capture"));
        assert!(!result.contains("Option :: Some"));
    }

    #[test]
    fn test_struct_constructor_multiple_fields() {
        let from_field = make_field(
            0,
            Some("source"),
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let backtrace_field = make_field(
            1,
            Some("backtrace"),
            parse_quote!(Option<std::backtrace::Backtrace>),
            make_field_attrs_with_backtrace()
        );
        let source_field = make_field(
            2,
            Some("inner_source"),
            parse_quote!(Option<Box<dyn std::error::Error>>),
            make_field_attrs_with_source()
        );
        let fields = Fields::Named(vec![from_field, backtrace_field, source_field]);
        let result = struct_constructor(&fields, fields.iter().next().unwrap());
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        assert!(tokens.contains("source : value"));
        assert!(tokens.contains("backtrace :"));
        assert!(tokens.contains("inner_source :"));
    }

    #[test]
    fn test_variant_constructor_multiple_unnamed_fields() {
        let from_field = make_field(
            0,
            None,
            parse_quote!(std::io::Error),
            make_field_attrs_with_from()
        );
        let backtrace_field = make_field(
            1,
            None,
            parse_quote!(std::backtrace::Backtrace),
            make_field_attrs_with_backtrace()
        );
        let fields = Fields::Unnamed(vec![from_field, backtrace_field]);
        let variant_ident = syn::Ident::new("Io", Span::call_site());
        let result = variant_constructor(&variant_ident, &fields, fields.iter().next().unwrap());
        assert!(result.is_ok());
        let tokens = result.unwrap().to_string();
        assert!(tokens.contains("Self :: Io"));
        assert!(tokens.contains("value"));
        assert!(tokens.contains("Backtrace :: capture"));
    }

    #[test]
    fn test_expand_enum_without_from_fields() {
        let plain_field = make_field(
            0,
            Some("message"),
            parse_quote!(String),
            make_field_attrs_plain()
        );
        let variant = VariantData {
            ident:       syn::Ident::new("Custom", Span::call_site()),
            fields:      Fields::Named(vec![plain_field]),
            display:     make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error:   None,
            masterror:   None,
            span:        Span::call_site()
        };
        let data = ErrorData::Enum(vec![variant]);
        let input = make_error_input("MyError", data);
        let result = expand(&input);
        assert!(result.is_ok());
        let impls = result.unwrap();
        assert_eq!(impls.len(), 0);
    }

    #[test]
    fn test_struct_from_impl_with_generics() {
        let from_field = make_field(
            0,
            Some("source"),
            parse_quote!(T),
            make_field_attrs_with_from()
        );
        let fields = Fields::Named(vec![from_field]);
        let struct_data = StructData {
            fields,
            display: make_display_spec(),
            format_args: FormatArgsSpec::default(),
            app_error: None,
            masterror: None
        };
        let from_field_input = make_field(
            0,
            Some("source"),
            parse_quote!(T),
            make_field_attrs_with_from()
        );
        let mut input = make_error_input(
            "MyError",
            ErrorData::Struct(Box::new(StructData {
                fields:      Fields::Named(vec![from_field_input]),
                display:     make_display_spec(),
                format_args: FormatArgsSpec::default(),
                app_error:   None,
                masterror:   None
            }))
        );
        input.generics = parse_quote!(<T>);
        let result = struct_from_impl(
            &input,
            &struct_data,
            struct_data.fields.iter().next().unwrap()
        );
        assert!(result.is_ok());
        let impl_tokens = result.unwrap().to_string();
        assert!(impl_tokens.contains("impl"));
        assert!(impl_tokens.contains("From"));
    }
}
