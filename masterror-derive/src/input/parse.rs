// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Main parsing logic for error derive macro input.
//!
//! Handles top-level parsing of struct and enum error definitions,
//! coordinating attribute extraction and validation.

use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Error, Ident, spanned::Spanned};

use super::{
    parse_attr::{extract_app_error_spec, extract_display_spec, extract_masterror_spec},
    types::{ErrorData, ErrorInput, Fields, FormatArgsSpec, StructData, VariantData},
    utils::{
        collect_errors, path_is, validate_backtrace_usage, validate_from_usage,
        validate_transparent
    }
};

/// Parses derive macro input into ErrorInput structure.
///
/// Main entry point for parsing error definitions from syn AST.
pub fn parse_input(input: DeriveInput) -> Result<ErrorInput, Error> {
    let mut errors = Vec::new();

    let ident = input.ident;
    let generics = input.generics;

    let data = match input.data {
        Data::Struct(data) => parse_struct(&ident, &input.attrs, data, &mut errors),
        Data::Enum(data) => parse_enum(&input.attrs, data, &mut errors),
        Data::Union(union) => {
            errors.push(Error::new(
                union.union_token.span(),
                "Error cannot be derived for unions"
            ));
            Err(())
        }
    };

    let data = match data {
        Ok(value) => value,
        Err(()) => {
            return Err(collect_errors(errors));
        }
    };

    if errors.is_empty() {
        Ok(ErrorInput {
            ident,
            generics,
            data
        })
    } else {
        Err(collect_errors(errors))
    }
}

/// Parses struct error definition.
fn parse_struct(
    ident: &Ident,
    attrs: &[Attribute],
    data: DataStruct,
    errors: &mut Vec<Error>
) -> Result<ErrorData, ()> {
    let display = extract_display_spec(attrs, ident.span(), errors)?;
    let app_error = extract_app_error_spec(attrs, errors)?;
    let masterror = extract_masterror_spec(attrs, errors)?;
    let fields = Fields::from_syn(&data.fields, errors);

    validate_from_usage(&fields, &display, errors);
    validate_backtrace_usage(&fields, errors);
    validate_transparent(&fields, &display, errors, None);

    Ok(ErrorData::Struct(Box::new(StructData {
        fields,
        display,
        format_args: FormatArgsSpec::default(),
        app_error,
        masterror
    })))
}

/// Parses enum error definition.
fn parse_enum(
    attrs: &[Attribute],
    data: DataEnum,
    errors: &mut Vec<Error>
) -> Result<ErrorData, ()> {
    for attr in attrs {
        if path_is(attr, "error") {
            errors.push(Error::new_spanned(
                attr,
                "type-level #[error] attributes are not supported"
            ));
        }
    }

    let mut variants = Vec::new();
    for variant in data.variants {
        variants.push(parse_variant(variant, errors)?);
    }

    Ok(ErrorData::Enum(variants))
}

/// Parses single enum variant.
fn parse_variant(variant: syn::Variant, errors: &mut Vec<Error>) -> Result<VariantData, ()> {
    let span = variant.span();
    for attr in &variant.attrs {
        if path_is(attr, "from") {
            errors.push(Error::new_spanned(
                attr,
                "not expected here; the #[from] attribute belongs on a specific field"
            ));
        }
    }

    let display = extract_display_spec(&variant.attrs, span, errors)?;
    let app_error = extract_app_error_spec(&variant.attrs, errors)?;
    let masterror = extract_masterror_spec(&variant.attrs, errors)?;
    let fields = Fields::from_syn(&variant.fields, errors);

    validate_from_usage(&fields, &display, errors);
    validate_backtrace_usage(&fields, errors);
    validate_transparent(&fields, &display, errors, Some(&variant));

    Ok(VariantData {
        ident: variant.ident,
        fields,
        display,
        format_args: FormatArgsSpec::default(),
        app_error,
        masterror,
        span
    })
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn parse_input_struct() {
        let input: DeriveInput = parse_quote! {
            #[error("test error")]
            struct TestError {
                msg: String
            }
        };
        let result = parse_input(input);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_input_enum() {
        let input: DeriveInput = parse_quote! {
            enum TestError {
                #[error("variant a")]
                A,
                #[error("variant b")]
                B
            }
        };
        let result = parse_input(input);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_input_union_error() {
        let input: DeriveInput = parse_quote! {
            union TestError {
                x: i32
            }
        };
        let result = parse_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn parse_enum_type_level_error_attr() {
        let input: DeriveInput = parse_quote! {
            #[error("not allowed")]
            enum TestError {
                #[error("variant")]
                A
            }
        };
        let result = parse_input(input);
        assert!(result.is_err());
    }

    #[test]
    fn parse_variant_from_attr() {
        let input: DeriveInput = parse_quote! {
            enum TestError {
                #[from]
                #[error("variant")]
                A(io::Error)
            }
        };
        let result = parse_input(input);
        assert!(result.is_err());
    }
}
