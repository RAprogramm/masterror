use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Error, Expr, ExprPath, Index};

use crate::input::{
    ErrorData, ErrorInput, Field, Fields, MasterrorSpec, StructData, VariantData, is_option_type
};

pub fn expand(input: &ErrorInput) -> Result<TokenStream, Error> {
    match &input.data {
        ErrorData::Struct(data) => expand_struct(input, data),
        ErrorData::Enum(variants) => expand_enum(input, variants)
    }
}

fn expand_struct(input: &ErrorInput, data: &StructData) -> Result<TokenStream, Error> {
    let spec = data.masterror.as_ref().ok_or_else(|| {
        Error::new(
            input.ident.span(),
            "#[derive(Masterror)] requires #[masterror(...)] on structs"
        )
    })?;

    let conversion = struct_conversion_impl(input, data, spec);
    let mappings = struct_mapping_impl(input, spec);

    Ok(quote! {
        #conversion
        #mappings
    })
}

fn expand_enum(input: &ErrorInput, variants: &[VariantData]) -> Result<TokenStream, Error> {
    ensure_all_variants_have_masterror(variants)?;

    let conversion = enum_conversion_impl(input, variants);
    let mappings = enum_mapping_impl(input, variants);

    Ok(quote! {
        #conversion
        #mappings
    })
}

fn ensure_all_variants_have_masterror(variants: &[VariantData]) -> Result<(), Error> {
    for variant in variants {
        if variant.masterror.is_none() {
            return Err(Error::new(
                variant.span,
                "all variants must use #[masterror(...)] to derive masterror::Error conversion"
            ));
        }
    }
    Ok(())
}

struct BoundField<'a> {
    field:   &'a Field,
    binding: Ident
}

fn struct_conversion_impl(
    input: &ErrorInput,
    data: &StructData,
    spec: &MasterrorSpec
) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let code = &spec.code;
    let category = &spec.category;

    let message_init = message_initialization(spec.expose_message, quote!(&value));
    let (destructure, bound_fields) = bind_struct_fields(ident, &data.fields);
    let field_usage = field_usage_tokens(&bound_fields);
    let telemetry_init = telemetry_initialization(&spec.telemetry);
    let metadata_attach = metadata_attach_tokens();
    let redact_tokens = redact_tokens(spec.redact_message);
    let source_tokens = source_attachment_tokens(&bound_fields);
    let backtrace_tokens = backtrace_attachment_tokens(&data.fields, &bound_fields);

    quote! {
        impl #impl_generics core::convert::From<#ident #ty_generics> for masterror::Error #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                #message_init
                #destructure
                #field_usage
                #telemetry_init
                let mut __masterror_error = match __masterror_message {
                    Some(message) => masterror::Error::with((#category), message),
                    None => masterror::Error::bare((#category))
                };
                __masterror_error = __masterror_error.with_code((#code));
                #metadata_attach
                #redact_tokens
                #source_tokens
                #backtrace_tokens
                __masterror_error
            }
        }
    }
}

fn enum_conversion_impl(input: &ErrorInput, variants: &[VariantData]) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut arms = Vec::new();

    let mut message_arms = Vec::new();

    for variant in variants {
        let spec = variant.masterror.as_ref().expect("presence checked");
        let code = &spec.code;
        let category = &spec.category;
        let (pattern, bound_fields) = bind_variant_fields(ident, variant);
        let field_usage = field_usage_tokens(&bound_fields);
        let telemetry_init = telemetry_initialization(&spec.telemetry);
        let metadata_attach = metadata_attach_tokens();
        let redact_tokens = redact_tokens(spec.redact_message);
        let source_tokens = source_attachment_tokens(&bound_fields);
        let backtrace_tokens = backtrace_attachment_tokens(&variant.fields, &bound_fields);
        message_arms.push(enum_message_arm(ident, variant, spec.expose_message));

        arms.push(quote! {
            #pattern => {
                #field_usage
                #telemetry_init
                let mut __masterror_error = match __masterror_message {
                    Some(message) => masterror::Error::with((#category), message),
                    None => masterror::Error::bare((#category))
                };
                __masterror_error = __masterror_error.with_code((#code));
                #metadata_attach
                #redact_tokens
                #source_tokens
                #backtrace_tokens
                __masterror_error
            }
        });
    }

    let message_match = quote! {
        let __masterror_message: Option<String> = match &value {
            #(#message_arms)*
        };
    };

    quote! {
        impl #impl_generics core::convert::From<#ident #ty_generics> for masterror::Error #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                #message_match
                match value {
                    #(#arms),*
                }
            }
        }
    }
}

fn enum_message_arm(
    enum_ident: &Ident,
    variant: &VariantData,
    expose_message: bool
) -> TokenStream {
    if expose_message {
        let binding = format_ident!("__masterror_variant_ref");
        let pattern = enum_message_pattern(enum_ident, variant, Some(&binding));
        quote! {
            #pattern => Some(std::string::ToString::to_string(#binding)),
        }
    } else {
        let pattern = enum_message_pattern(enum_ident, variant, None);
        quote! {
            #pattern => None,
        }
    }
}

fn enum_message_pattern(
    enum_ident: &Ident,
    variant: &VariantData,
    binding: Option<&Ident>
) -> TokenStream {
    let variant_ident = &variant.ident;
    match (&variant.fields, binding) {
        (Fields::Unit, Some(binding)) => quote!(#binding @ #enum_ident::#variant_ident),
        (Fields::Unit, None) => quote!(#enum_ident::#variant_ident),
        (Fields::Named(_), Some(binding)) => quote!(#binding @ #enum_ident::#variant_ident { .. }),
        (Fields::Named(_), None) => quote!(#enum_ident::#variant_ident { .. }),
        (Fields::Unnamed(_), Some(binding)) => quote!(#binding @ #enum_ident::#variant_ident(..)),
        (Fields::Unnamed(_), None) => quote!(#enum_ident::#variant_ident(..))
    }
}

fn field_usage_tokens(bound_fields: &[BoundField<'_>]) -> TokenStream {
    if bound_fields.is_empty() {
        return TokenStream::new();
    }

    let names = bound_fields.iter().map(|field| &field.binding);
    quote! {
        let _ = (#(&#names),*);
    }
}

fn struct_mapping_impl(input: &ErrorInput, spec: &MasterrorSpec) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let code = &spec.code;
    let category = &spec.category;
    let grpc_mapping =
        mapping_option_tokens(spec.map_grpc.as_ref(), code, category, MappingKind::Grpc);
    let problem_mapping = mapping_option_tokens(
        spec.map_problem.as_ref(),
        code,
        category,
        MappingKind::Problem
    );

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            /// HTTP mapping for this error type.
            pub const HTTP_MAPPING: masterror::mapping::HttpMapping =
                masterror::mapping::HttpMapping::new((#code), (#category));

            /// gRPC mapping for this error type.
            pub const GRPC_MAPPING: Option<masterror::mapping::GrpcMapping> = #grpc_mapping;

            /// Problem JSON mapping for this error type.
            pub const PROBLEM_MAPPING: Option<masterror::mapping::ProblemMapping> = #problem_mapping;
        }
    }
}

fn enum_mapping_impl(input: &ErrorInput, variants: &[VariantData]) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let http_entries: Vec<_> = variants
        .iter()
        .map(|variant| {
            let spec = variant.masterror.as_ref().expect("presence checked");
            let code = &spec.code;
            let category = &spec.category;
            quote!(masterror::mapping::HttpMapping::new((#code), (#category)))
        })
        .collect();

    let grpc_entries: Vec<_> = variants
        .iter()
        .filter_map(|variant| {
            let spec = variant.masterror.as_ref().expect("presence checked");
            let code = &spec.code;
            let category = &spec.category;
            spec.map_grpc.as_ref().map(
                |expr| quote!(masterror::mapping::GrpcMapping::new((#code), (#category), (#expr)))
            )
        })
        .collect();

    let problem_entries: Vec<_> = variants
        .iter()
        .filter_map(|variant| {
            let spec = variant.masterror.as_ref().expect("presence checked");
            let code = &spec.code;
            let category = &spec.category;
            spec.map_problem.as_ref().map(|expr| {
                quote!(masterror::mapping::ProblemMapping::new((#code), (#category), (#expr)))
            })
        })
        .collect();

    let http_len = Index::from(http_entries.len());

    let grpc_slice = if grpc_entries.is_empty() {
        quote!(&[] as &[masterror::mapping::GrpcMapping])
    } else {
        quote!(&[#(#grpc_entries),*])
    };

    let problem_slice = if problem_entries.is_empty() {
        quote!(&[] as &[masterror::mapping::ProblemMapping])
    } else {
        quote!(&[#(#problem_entries),*])
    };

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            /// HTTP mappings for enum variants.
            pub const HTTP_MAPPINGS: [masterror::mapping::HttpMapping; #http_len] = [#(#http_entries),*];

            /// gRPC mappings for enum variants.
            pub const GRPC_MAPPINGS: &'static [masterror::mapping::GrpcMapping] = #grpc_slice;

            /// Problem JSON mappings for enum variants.
            pub const PROBLEM_MAPPINGS: &'static [masterror::mapping::ProblemMapping] = #problem_slice;
        }
    }
}

fn message_initialization(enabled: bool, value: TokenStream) -> TokenStream {
    if enabled {
        quote! {
            let __masterror_message = Some(std::string::ToString::to_string(#value));
        }
    } else {
        quote! {
            let __masterror_message: Option<String> = None;
        }
    }
}

fn bind_struct_fields<'a>(
    ident: &Ident,
    fields: &'a Fields
) -> (TokenStream, Vec<BoundField<'a>>) {
    match fields {
        Fields::Unit => (quote!(let _ = value;), Vec::new()),
        Fields::Named(list) => {
            let mut pattern = Vec::new();
            let mut bound = Vec::new();
            for field in list {
                let binding = binding_ident(field);
                let pattern_binding = binding.clone();
                pattern.push(quote!(#pattern_binding));
                bound.push(BoundField {
                    field,
                    binding
                });
            }
            let pattern_tokens = quote!(let #ident { #(#pattern),* } = value;);
            (pattern_tokens, bound)
        }
        Fields::Unnamed(list) => {
            let mut pattern = Vec::new();
            let mut bound = Vec::new();
            for field in list {
                let binding = binding_ident(field);
                let pattern_binding = binding.clone();
                pattern.push(quote!(#pattern_binding));
                bound.push(BoundField {
                    field,
                    binding
                });
            }
            let pattern_tokens = quote!(let #ident(#(#pattern),*) = value;);
            (pattern_tokens, bound)
        }
    }
}

fn bind_variant_fields<'a>(
    enum_ident: &Ident,
    variant: &'a VariantData
) -> (TokenStream, Vec<BoundField<'a>>) {
    let variant_ident = &variant.ident;

    match &variant.fields {
        Fields::Unit => (quote!(#enum_ident::#variant_ident), Vec::new()),
        Fields::Named(list) => {
            let mut pattern = Vec::new();
            let mut bound = Vec::new();
            for field in list {
                let binding = binding_ident(field);
                let pattern_binding = binding.clone();
                pattern.push(quote!(#pattern_binding));
                bound.push(BoundField {
                    field,
                    binding
                });
            }
            (quote!(#enum_ident::#variant_ident { #(#pattern),* }), bound)
        }
        Fields::Unnamed(list) => {
            let mut pattern = Vec::new();
            let mut bound = Vec::new();
            for field in list {
                let binding = binding_ident(field);
                let pattern_binding = binding.clone();
                pattern.push(quote!(#pattern_binding));
                bound.push(BoundField {
                    field,
                    binding
                });
            }
            (quote!(#enum_ident::#variant_ident(#(#pattern),*)), bound)
        }
    }
}

fn telemetry_initialization(entries: &[Expr]) -> TokenStream {
    if entries.is_empty() {
        quote!(let __masterror_metadata: Option<masterror::Metadata> = None;)
    } else {
        let inserts = entries.iter().map(|expr| {
            quote! {
                if let Some(field) = (#expr) {
                    __masterror_metadata_inner.insert(field);
                }
            }
        });
        quote! {
            let mut __masterror_metadata_inner = masterror::Metadata::new();
            #(#inserts)*
            let __masterror_metadata = if __masterror_metadata_inner.is_empty() {
                None
            } else {
                Some(__masterror_metadata_inner)
            };
        }
    }
}

fn metadata_attach_tokens() -> TokenStream {
    quote! {
        if let Some(metadata) = __masterror_metadata {
            __masterror_error = __masterror_error.with_metadata(metadata);
        }
    }
}

fn redact_tokens(enabled: bool) -> TokenStream {
    if enabled {
        quote!(
            __masterror_error = __masterror_error.redactable();
        )
    } else {
        TokenStream::new()
    }
}

fn source_attachment_tokens(bound_fields: &[BoundField<'_>]) -> TokenStream {
    for bound in bound_fields {
        if bound.field.attrs.has_source() {
            let binding = &bound.binding;
            if is_option_type(&bound.field.ty) {
                return quote! {
                    if let Some(source) = #binding {
                        __masterror_error = __masterror_error.with_source(source);
                    }
                };
            } else {
                return quote! {
                    __masterror_error = __masterror_error.with_source(#binding);
                };
            }
        }
    }
    TokenStream::new()
}

fn backtrace_attachment_tokens(fields: &Fields, bound_fields: &[BoundField<'_>]) -> TokenStream {
    let Some(backtrace_field) = fields.backtrace_field() else {
        return TokenStream::new();
    };
    let index = backtrace_field.index();
    let Some(binding) = bound_fields
        .iter()
        .find(|bound| bound.field.index == index)
        .map(|bound| &bound.binding)
    else {
        return TokenStream::new();
    };

    if is_option_type(&backtrace_field.field().ty) {
        quote! {
            if let Some(trace) = #binding {
                __masterror_error = __masterror_error.with_backtrace(trace);
            }
        }
    } else {
        quote! {
            __masterror_error = __masterror_error.with_backtrace(#binding);
        }
    }
}

#[derive(Clone, Copy)]
enum MappingKind {
    Grpc,
    Problem
}

fn mapping_option_tokens(
    expr: Option<&Expr>,
    code: &Expr,
    category: &ExprPath,
    kind: MappingKind
) -> TokenStream {
    match expr {
        Some(value) => match kind {
            MappingKind::Grpc => {
                quote!(Some(masterror::mapping::GrpcMapping::new((#code), (#category), (#value))))
            }
            MappingKind::Problem => {
                quote!(Some(masterror::mapping::ProblemMapping::new((#code), (#category), (#value))))
            }
        },
        None => quote!(None)
    }
}

fn binding_ident(field: &Field) -> Ident {
    field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("__field{}", field.index, span = field.span))
}
