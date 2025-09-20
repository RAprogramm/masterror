use std::{borrow::Cow, collections::HashMap};

use masterror_template::template::{TemplateFormatter, TemplateFormatterKind};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{Error, Index};

use crate::{
    input::{
        DisplaySpec, ErrorData, ErrorInput, Field, Fields, FormatArg, FormatArgProjection,
        FormatArgProjectionMethodCall, FormatArgProjectionSegment, FormatArgShorthand,
        FormatArgValue, FormatArgsSpec, FormatBindingKind, StructData, VariantData,
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
        DisplaySpec::Template(template) => render_template(template, Vec::new(), |placeholder| {
            struct_placeholder_expr(&data.fields, placeholder, None)
        })?,
        DisplaySpec::TemplateWithArgs {
            template,
            args
        } => {
            let mut env = FormatArgumentsEnv::new_struct(args, &data.fields);
            let preludes = env.prelude_tokens();
            render_template(template, preludes, |placeholder| {
                struct_placeholder_expr(&data.fields, placeholder, Some(&mut env))
            })?
        }
        DisplaySpec::FormatterPath {
            path, ..
        } => render_struct_formatter_path(&data.fields, path)
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

fn struct_formatter_arguments(fields: &Fields) -> Vec<TokenStream> {
    match fields {
        Fields::Unit => Vec::new(),
        Fields::Named(fields) | Fields::Unnamed(fields) => fields
            .iter()
            .map(|field| {
                let member = &field.member;
                quote!(&self.#member)
            })
            .collect()
    }
}

fn formatter_path_call(path: &syn::ExprPath, mut args: Vec<TokenStream>) -> TokenStream {
    args.push(quote!(f));
    quote! {
        #path(#(#args),*)
    }
}

fn render_variant(variant: &VariantData) -> Result<TokenStream, Error> {
    match &variant.display {
        DisplaySpec::Transparent {
            ..
        } => render_variant_transparent(variant),
        DisplaySpec::Template(template) => render_variant_template(variant, template, None),
        DisplaySpec::TemplateWithArgs {
            template,
            args
        } => render_variant_template(variant, template, Some(args)),
        DisplaySpec::FormatterPath {
            path, ..
        } => render_variant_formatter_path(variant, path)
    }
}

fn render_struct_formatter_path(fields: &Fields, path: &syn::ExprPath) -> TokenStream {
    let args = struct_formatter_arguments(fields);
    formatter_path_call(path, args)
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

    fn expr_tokens(&self) -> TokenStream {
        self.expr.clone()
    }
}

#[derive(Debug)]
enum RenderedSegment {
    Literal(String),
    Placeholder(PlaceholderRender)
}

#[derive(Debug)]
struct PlaceholderRender {
    identifier: TemplateIdentifierSpec,
    formatter:  TemplateFormatter,
    span:       Span,
    resolved:   ResolvedPlaceholderExpr
}

#[derive(Debug)]
struct FormatArgumentsEnv<'a> {
    context:    FormatArgContext<'a>,
    args:       Vec<EnvFormatArg<'a>>,
    named:      HashMap<String, usize>,
    positional: HashMap<usize, usize>,
    implicit:   Vec<Option<usize>>
}

#[derive(Debug)]
enum FormatArgContext<'a> {
    Struct(&'a Fields),
    Variant {
        fields:   &'a Fields,
        bindings: Vec<Ident>
    }
}

#[derive(Debug)]
struct EnvFormatArg<'a> {
    binding: Option<Ident>,
    arg:     &'a FormatArg
}

impl<'a> FormatArgumentsEnv<'a> {
    fn new_struct(spec: &'a FormatArgsSpec, fields: &'a Fields) -> Self {
        Self::new_with_context(spec, FormatArgContext::Struct(fields))
    }

    fn new_variant(spec: &'a FormatArgsSpec, fields: &'a Fields, bindings: &[Ident]) -> Self {
        Self::new_with_context(
            spec,
            FormatArgContext::Variant {
                fields,
                bindings: bindings.to_vec()
            }
        )
    }

    fn new_with_context(spec: &'a FormatArgsSpec, context: FormatArgContext<'a>) -> Self {
        let mut env = Self {
            context,
            args: Vec::new(),
            named: HashMap::new(),
            positional: HashMap::new(),
            implicit: Vec::new()
        };

        for (index, arg) in spec.args.iter().enumerate() {
            let binding = match &arg.value {
                FormatArgValue::Expr(_) => Some(format_ident!("__masterror_format_arg_{}", index)),
                FormatArgValue::Shorthand(_) => None
            };

            let arg_index = env.args.len();
            env.args.push(EnvFormatArg {
                binding: binding.clone(),
                arg
            });

            match &arg.kind {
                FormatBindingKind::Named(ident) => {
                    env.named.insert(ident.to_string(), arg_index);
                }
                FormatBindingKind::Positional(pos_index) => {
                    env.positional.insert(*pos_index, arg_index);
                    env.implicit.push(Some(arg_index));
                }
                FormatBindingKind::Implicit(implicit_index) => {
                    env.register_implicit(*implicit_index, arg_index);
                }
            }
        }

        env
    }

    fn prelude_tokens(&self) -> Vec<TokenStream> {
        self.args.iter().map(EnvFormatArg::prelude_tokens).collect()
    }

    fn resolve_placeholder(
        &mut self,
        placeholder: &TemplatePlaceholderSpec
    ) -> Result<Option<ResolvedPlaceholderExpr>, Error> {
        let arg_index = match &placeholder.identifier {
            TemplateIdentifierSpec::Named(name) => self.named.get(name).copied(),
            TemplateIdentifierSpec::Positional(index) => self.positional.get(index).copied(),
            TemplateIdentifierSpec::Implicit(index) => {
                self.implicit.get(*index).and_then(|slot| *slot)
            }
        };

        let index = match arg_index {
            Some(index) => index,
            None => return Ok(None)
        };

        let resolved = self.args[index].resolved_expr(self, placeholder)?;
        Ok(Some(resolved))
    }

    fn register_implicit(&mut self, index: usize, arg_index: usize) {
        if self.implicit.len() <= index {
            self.implicit.resize(index + 1, None);
        }
        self.implicit[index] = Some(arg_index);
    }

    fn resolve_shorthand(
        &self,
        shorthand: &FormatArgShorthand,
        placeholder: &TemplatePlaceholderSpec
    ) -> Result<ResolvedPlaceholderExpr, Error> {
        match &self.context {
            FormatArgContext::Struct(fields) => {
                resolve_struct_shorthand(fields, shorthand, placeholder)
            }
            FormatArgContext::Variant {
                fields,
                bindings
            } => resolve_variant_shorthand(fields, bindings, shorthand, placeholder)
        }
    }
}

impl<'a> EnvFormatArg<'a> {
    fn prelude_tokens(&self) -> TokenStream {
        match (&self.binding, &self.arg.value) {
            (Some(binding), FormatArgValue::Expr(expr)) => {
                quote! { let #binding = #expr; }
            }
            _ => TokenStream::new()
        }
    }

    fn resolved_expr(
        &self,
        env: &FormatArgumentsEnv<'_>,
        placeholder: &TemplatePlaceholderSpec
    ) -> Result<ResolvedPlaceholderExpr, Error> {
        match (&self.binding, &self.arg.value) {
            (Some(binding), FormatArgValue::Expr(_)) => {
                if needs_pointer_value(&placeholder.formatter) {
                    Ok(ResolvedPlaceholderExpr::with(quote!(#binding), true))
                } else {
                    Ok(ResolvedPlaceholderExpr::new(quote!(&#binding)))
                }
            }
            (_, FormatArgValue::Shorthand(shorthand)) => {
                env.resolve_shorthand(shorthand, placeholder)
            }
            _ => unreachable!()
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

fn resolve_struct_shorthand(
    fields: &Fields,
    shorthand: &FormatArgShorthand,
    placeholder: &TemplatePlaceholderSpec
) -> Result<ResolvedPlaceholderExpr, Error> {
    let FormatArgShorthand::Projection(projection) = shorthand;

    let (expr, first_field, has_tail) = struct_projection_expr(fields, projection)?;

    if !has_tail && let Some(field) = first_field {
        return Ok(struct_field_expr(field, &placeholder.formatter));
    }

    if needs_pointer_value(&placeholder.formatter) {
        Ok(ResolvedPlaceholderExpr::with(expr, false))
    } else {
        Ok(ResolvedPlaceholderExpr::new(quote!(&(#expr))))
    }
}

fn resolve_variant_shorthand(
    fields: &Fields,
    bindings: &[Ident],
    shorthand: &FormatArgShorthand,
    placeholder: &TemplatePlaceholderSpec
) -> Result<ResolvedPlaceholderExpr, Error> {
    let FormatArgShorthand::Projection(projection) = shorthand;

    let Some(first_segment) = projection.segments.first() else {
        return Err(Error::new(
            projection.span,
            "empty shorthand projection is not supported"
        ));
    };

    match first_segment {
        FormatArgProjectionSegment::Field(ident) => {
            let Fields::Named(named_fields) = fields else {
                return Err(Error::new(
                    ident.span(),
                    format!(
                        "named field `{}` is not available for tuple variants",
                        ident
                    )
                ));
            };

            let position = named_fields.iter().position(|field| {
                field
                    .ident
                    .as_ref()
                    .is_some_and(|field_ident| field_ident == ident)
            });

            let index = position.ok_or_else(|| {
                Error::new(
                    ident.span(),
                    format!("unknown field `{}` in format arguments", ident)
                )
            })?;

            let binding = bindings.get(index).ok_or_else(|| {
                Error::new(
                    ident.span(),
                    format!("field `{}` is not available in format arguments", ident)
                )
            })?;

            let expr = if projection.segments.len() == 1 {
                quote!(#binding)
            } else {
                append_projection_segments(quote!(#binding), &projection.segments[1..])
            };

            if projection.segments.len() == 1 {
                Ok(ResolvedPlaceholderExpr::with(
                    expr,
                    needs_pointer_value(&placeholder.formatter)
                ))
            } else if needs_pointer_value(&placeholder.formatter) {
                Ok(ResolvedPlaceholderExpr::with(expr, false))
            } else {
                Ok(ResolvedPlaceholderExpr::new(quote!(&(#expr))))
            }
        }
        FormatArgProjectionSegment::Index {
            index,
            span
        } => {
            let Fields::Unnamed(_) = fields else {
                return Err(Error::new(
                    *span,
                    "positional fields are not available for struct variants"
                ));
            };

            let binding = bindings.get(*index).ok_or_else(|| {
                Error::new(
                    *span,
                    format!("field `{}` is not available in format arguments", index)
                )
            })?;

            let expr = if projection.segments.len() == 1 {
                quote!(#binding)
            } else {
                append_projection_segments(quote!(#binding), &projection.segments[1..])
            };

            if projection.segments.len() == 1 {
                Ok(ResolvedPlaceholderExpr::with(
                    expr,
                    needs_pointer_value(&placeholder.formatter)
                ))
            } else if needs_pointer_value(&placeholder.formatter) {
                Ok(ResolvedPlaceholderExpr::with(expr, false))
            } else {
                Ok(ResolvedPlaceholderExpr::new(quote!(&(#expr))))
            }
        }
        FormatArgProjectionSegment::MethodCall(call) => Err(Error::new(
            call.span,
            "variant format projections must start with a field or index"
        ))
    }
}

fn struct_projection_expr<'a>(
    fields: &'a Fields,
    projection: &'a FormatArgProjection
) -> Result<(TokenStream, Option<&'a Field>, bool), Error> {
    let Some(first) = projection.segments.first() else {
        return Err(Error::new(
            projection.span,
            "empty shorthand projection is not supported"
        ));
    };

    let mut first_field = None;
    let mut expr = match first {
        FormatArgProjectionSegment::Field(ident) => {
            let field = fields.get_named(&ident.to_string()).ok_or_else(|| {
                Error::new(
                    ident.span(),
                    format!("unknown field `{}` in format arguments", ident)
                )
            })?;
            first_field = Some(field);
            let member = &field.member;
            quote!(self.#member)
        }
        FormatArgProjectionSegment::Index {
            index,
            span
        } => {
            let field = fields.get_positional(*index).ok_or_else(|| {
                Error::new(
                    *span,
                    format!("field `{}` is not available in format arguments", index)
                )
            })?;
            first_field = Some(field);
            let member = &field.member;
            quote!(self.#member)
        }
        FormatArgProjectionSegment::MethodCall(call) => append_method_call(quote!(self), call)
    };

    if projection.segments.len() > 1 {
        expr = append_projection_segments(expr, &projection.segments[1..]);
    }

    Ok((expr, first_field, projection.segments.len() > 1))
}

fn append_projection_segments(
    mut expr: TokenStream,
    segments: &[FormatArgProjectionSegment]
) -> TokenStream {
    for segment in segments {
        expr = append_projection_segment(expr, segment);
    }
    expr
}

fn append_projection_segment(
    expr: TokenStream,
    segment: &FormatArgProjectionSegment
) -> TokenStream {
    match segment {
        FormatArgProjectionSegment::Field(ident) => quote!((#expr).#ident),
        FormatArgProjectionSegment::Index {
            index,
            span
        } => {
            let index_token = Index {
                index: *index as u32,
                span:  *span
            };
            quote!((#expr).#index_token)
        }
        FormatArgProjectionSegment::MethodCall(call) => append_method_call(expr, call)
    }
}

fn append_method_call(expr: TokenStream, call: &FormatArgProjectionMethodCall) -> TokenStream {
    let method = &call.method;
    let args = &call.args;
    if let Some(turbofish) = &call.turbofish {
        let colon2 = turbofish.colon2_token;
        let generics = &turbofish.generics;
        quote!((#expr).#method #colon2 #generics (#args))
    } else {
        quote!((#expr).#method(#args))
    }
}

fn render_variant_formatter_path(
    variant: &VariantData,
    path: &syn::ExprPath
) -> Result<TokenStream, Error> {
    let variant_ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => {
            let call = formatter_path_call(path, Vec::new());
            Ok(quote! {
                Self::#variant_ident => {
                    #call
                }
            })
        }
        Fields::Unnamed(fields) => {
            let bindings: Vec<_> = fields.iter().map(binding_ident).collect();
            let pattern = quote!(Self::#variant_ident(#(#bindings),*));
            let call = formatter_path_call(path, variant_formatter_arguments(&bindings));
            Ok(quote! {
                #pattern => {
                    #call
                }
            })
        }
        Fields::Named(fields) => {
            let bindings: Vec<_> = fields.iter().map(binding_ident).collect();
            let pattern = quote!(Self::#variant_ident { #(#bindings),* });
            let call = formatter_path_call(path, variant_formatter_arguments(&bindings));
            Ok(quote! {
                #pattern => {
                    #call
                }
            })
        }
    }
}

fn variant_formatter_arguments(bindings: &[Ident]) -> Vec<TokenStream> {
    bindings.iter().map(|binding| quote!(#binding)).collect()
}

fn render_variant_template(
    variant: &VariantData,
    template: &DisplayTemplate,
    format_args: Option<&FormatArgsSpec>
) -> Result<TokenStream, Error> {
    let variant_ident = &variant.ident;
    match &variant.fields {
        Fields::Unit => {
            let mut env = format_args
                .map(|args| FormatArgumentsEnv::new_variant(args, &variant.fields, &[]));
            let preludes = env
                .as_ref()
                .map(|env| env.prelude_tokens())
                .unwrap_or_default();
            let span = variant.span;
            let body = render_template(template, preludes, |placeholder| {
                if let Some(env) = env.as_mut()
                    && let Some(resolved) = env.resolve_placeholder(placeholder)?
                {
                    return Ok(resolved);
                }
                Err(Error::new(span, "unit variants cannot reference fields"))
            })?;
            Ok(quote! {
                Self::#variant_ident => {
                    #body
                }
            })
        }
        Fields::Unnamed(fields) => {
            let bindings: Vec<_> = fields.iter().map(binding_ident).collect();
            let mut env = format_args
                .map(|args| FormatArgumentsEnv::new_variant(args, &variant.fields, &bindings));
            let pattern = quote!(Self::#variant_ident(#(#bindings),*));
            let preludes = env
                .as_ref()
                .map(|env| env.prelude_tokens())
                .unwrap_or_default();
            let body = render_template(template, preludes, |placeholder| {
                variant_tuple_placeholder(&bindings, placeholder, env.as_mut())
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
            let mut env = format_args
                .map(|args| FormatArgumentsEnv::new_variant(args, &variant.fields, &bindings));
            let pattern = quote!(Self::#variant_ident { #(#bindings),* });
            let preludes = env
                .as_ref()
                .map(|env| env.prelude_tokens())
                .unwrap_or_default();
            let body = render_template(template, preludes, |placeholder| {
                variant_named_placeholder(fields, &bindings, placeholder, env.as_mut())
            })?;
            Ok(quote! {
                #pattern => {
                    #body
                }
            })
        }
    }
}

fn render_template<F>(
    template: &DisplayTemplate,
    preludes: Vec<TokenStream>,
    mut resolver: F
) -> Result<TokenStream, Error>
where
    F: FnMut(&TemplatePlaceholderSpec) -> Result<ResolvedPlaceholderExpr, Error>
{
    let mut segments = Vec::new();
    let mut literal_buffer = String::new();
    let mut format_buffer = String::new();
    let mut has_placeholder = false;
    let mut has_implicit_placeholders = false;
    let mut requires_format_engine = false;

    for segment in &template.segments {
        match segment {
            TemplateSegmentSpec::Literal(text) => {
                literal_buffer.push_str(text);
                push_literal_fragment(&mut format_buffer, text);
                segments.push(RenderedSegment::Literal(text.clone()));
            }
            TemplateSegmentSpec::Placeholder(placeholder) => {
                has_placeholder = true;
                if matches!(placeholder.identifier, TemplateIdentifierSpec::Implicit(_)) {
                    has_implicit_placeholders = true;
                }
                if placeholder_requires_format_engine(&placeholder.formatter) {
                    requires_format_engine = true;
                }

                let resolved = resolver(placeholder)?;
                format_buffer.push_str(&placeholder_format_fragment(placeholder));
                segments.push(RenderedSegment::Placeholder(PlaceholderRender {
                    identifier: placeholder.identifier.clone(),
                    formatter: placeholder.formatter.clone(),
                    span: placeholder.span,
                    resolved
                }));
            }
        }
    }

    let has_additional_arguments = !preludes.is_empty();

    if !has_placeholder && !has_additional_arguments {
        let literal = Literal::string(&literal_buffer);
        return Ok(quote! {
            #(#preludes)*
            f.write_str(#literal)
        });
    }

    if has_additional_arguments || has_implicit_placeholders || requires_format_engine {
        let format_literal = Literal::string(&format_buffer);
        let args = build_template_arguments(&segments);
        return Ok(quote! {
            #(#preludes)*
            ::core::write!(f, #format_literal #(, #args)*)
        });
    }

    let mut pieces = preludes;
    for segment in segments {
        match segment {
            RenderedSegment::Literal(text) => {
                pieces.push(quote! { f.write_str(#text)?; });
            }
            RenderedSegment::Placeholder(placeholder) => {
                pieces.push(format_placeholder(
                    placeholder.resolved,
                    placeholder.formatter
                ));
            }
        }
    }
    pieces.push(quote! { Ok(()) });

    Ok(quote! {
        #(#pieces)*
    })
}

#[derive(Debug)]
struct NamedArgument {
    name: String,
    span: Span,
    expr: TokenStream
}

#[derive(Debug)]
struct IndexedArgument {
    index: usize,
    expr:  TokenStream
}

fn build_template_arguments(segments: &[RenderedSegment]) -> Vec<TokenStream> {
    let mut named = Vec::new();
    let mut positional = Vec::new();
    let mut implicit = Vec::new();

    for segment in segments {
        let RenderedSegment::Placeholder(placeholder) = segment else {
            continue;
        };

        match &placeholder.identifier {
            TemplateIdentifierSpec::Named(name) => {
                if named
                    .iter()
                    .any(|argument: &NamedArgument| argument.name == *name)
                {
                    continue;
                }

                named.push(NamedArgument {
                    name: name.clone(),
                    span: placeholder.span,
                    expr: placeholder.resolved.expr_tokens()
                });
            }
            TemplateIdentifierSpec::Positional(index) => {
                if positional
                    .iter()
                    .any(|argument: &IndexedArgument| argument.index == *index)
                {
                    continue;
                }

                positional.push(IndexedArgument {
                    index: *index,
                    expr:  placeholder.resolved.expr_tokens()
                });
            }
            TemplateIdentifierSpec::Implicit(index) => {
                if implicit
                    .iter()
                    .any(|argument: &IndexedArgument| argument.index == *index)
                {
                    continue;
                }

                implicit.push(IndexedArgument {
                    index: *index,
                    expr:  placeholder.resolved.expr_tokens()
                });
            }
        }
    }

    positional.sort_by_key(|argument| argument.index);
    implicit.sort_by_key(|argument| argument.index);

    let mut arguments = Vec::with_capacity(named.len() + positional.len() + implicit.len());
    for IndexedArgument {
        expr, ..
    } in positional
    {
        arguments.push(expr);
    }
    for IndexedArgument {
        expr, ..
    } in implicit
    {
        arguments.push(expr);
    }
    for NamedArgument {
        name,
        span,
        expr
    } in named
    {
        let ident = format_ident!("{}", name, span = span);
        arguments.push(quote_spanned!(span => #ident = #expr));
    }

    arguments
}

fn placeholder_requires_format_engine(formatter: &TemplateFormatter) -> bool {
    formatter.kind() != TemplateFormatterKind::Display || formatter.has_display_spec()
}

fn push_literal_fragment(buffer: &mut String, literal: &str) {
    for ch in literal.chars() {
        match ch {
            '{' => buffer.push_str("{{"),
            '}' => buffer.push_str("}}"),
            _ => buffer.push(ch)
        }
    }
}

fn placeholder_format_fragment(placeholder: &TemplatePlaceholderSpec) -> String {
    let mut fragment = String::from("{");

    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) => fragment.push_str(name),
        TemplateIdentifierSpec::Positional(index) => fragment.push_str(&index.to_string()),
        TemplateIdentifierSpec::Implicit(_) => {}
    }

    if let Some(spec) = formatter_format_fragment(&placeholder.formatter) {
        fragment.push(':');
        fragment.push_str(spec.as_ref());
    }

    fragment.push('}');
    fragment
}

fn formatter_format_fragment<'a>(formatter: &'a TemplateFormatter) -> Option<Cow<'a, str>> {
    formatter.format_fragment()
}

fn struct_placeholder_expr(
    fields: &Fields,
    placeholder: &TemplatePlaceholderSpec,
    env: Option<&mut FormatArgumentsEnv<'_>>
) -> Result<ResolvedPlaceholderExpr, Error> {
    if matches!(
        &placeholder.identifier,
        TemplateIdentifierSpec::Named(name) if name == "self"
    ) {
        return Ok(ResolvedPlaceholderExpr::with(
            quote!(self),
            needs_pointer_value(&placeholder.formatter)
        ));
    }

    if let Some(env) = env
        && let Some(resolved) = env.resolve_placeholder(placeholder)?
    {
        return Ok(resolved);
    }

    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) => {
            if let Some(field) = fields.get_named(name) {
                Ok(struct_field_expr(field, &placeholder.formatter))
            } else {
                Err(placeholder_error(placeholder.span, &placeholder.identifier))
            }
        }
        TemplateIdentifierSpec::Positional(index) => fields
            .get_positional(*index)
            .map(|field| struct_field_expr(field, &placeholder.formatter))
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier)),
        TemplateIdentifierSpec::Implicit(index) => fields
            .get_positional(*index)
            .map(|field| struct_field_expr(field, &placeholder.formatter))
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier))
    }
}

fn struct_field_expr(field: &Field, formatter: &TemplateFormatter) -> ResolvedPlaceholderExpr {
    let member = &field.member;

    if needs_pointer_value(formatter) && pointer_prefers_value(&field.ty) {
        ResolvedPlaceholderExpr::pointer(quote!(self.#member))
    } else {
        ResolvedPlaceholderExpr::new(quote!(&self.#member))
    }
}

fn needs_pointer_value(formatter: &TemplateFormatter) -> bool {
    formatter.kind() == TemplateFormatterKind::Pointer
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
    placeholder: &TemplatePlaceholderSpec,
    env: Option<&mut FormatArgumentsEnv<'_>>
) -> Result<ResolvedPlaceholderExpr, Error> {
    if matches!(
        &placeholder.identifier,
        TemplateIdentifierSpec::Named(name) if name == "self"
    ) {
        return Ok(ResolvedPlaceholderExpr::with(
            quote!(self),
            needs_pointer_value(&placeholder.formatter)
        ));
    }

    if let Some(env) = env
        && let Some(resolved) = env.resolve_placeholder(placeholder)?
    {
        return Ok(resolved);
    }

    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(_) => {
            Err(placeholder_error(placeholder.span, &placeholder.identifier))
        }
        TemplateIdentifierSpec::Positional(index) => bindings
            .get(*index)
            .map(|binding| {
                ResolvedPlaceholderExpr::with(
                    quote!(#binding),
                    needs_pointer_value(&placeholder.formatter)
                )
            })
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier)),
        TemplateIdentifierSpec::Implicit(index) => bindings
            .get(*index)
            .map(|binding| {
                ResolvedPlaceholderExpr::with(
                    quote!(#binding),
                    needs_pointer_value(&placeholder.formatter)
                )
            })
            .ok_or_else(|| placeholder_error(placeholder.span, &placeholder.identifier))
    }
}

fn variant_named_placeholder(
    fields: &[Field],
    bindings: &[Ident],
    placeholder: &TemplatePlaceholderSpec,
    env: Option<&mut FormatArgumentsEnv<'_>>
) -> Result<ResolvedPlaceholderExpr, Error> {
    if matches!(
        &placeholder.identifier,
        TemplateIdentifierSpec::Named(name) if name == "self"
    ) {
        return Ok(ResolvedPlaceholderExpr::with(
            quote!(self),
            needs_pointer_value(&placeholder.formatter)
        ));
    }

    if let Some(env) = env
        && let Some(resolved) = env.resolve_placeholder(placeholder)?
    {
        return Ok(resolved);
    }

    match &placeholder.identifier {
        TemplateIdentifierSpec::Named(name) => {
            if let Some(index) = fields
                .iter()
                .position(|field| field.ident.as_ref().is_some_and(|ident| ident == name))
            {
                let binding = &bindings[index];
                Ok(ResolvedPlaceholderExpr::with(
                    quote!(#binding),
                    needs_pointer_value(&placeholder.formatter)
                ))
            } else {
                Err(placeholder_error(placeholder.span, &placeholder.identifier))
            }
        }
        TemplateIdentifierSpec::Positional(index) => Err(placeholder_error(
            placeholder.span,
            &TemplateIdentifierSpec::Positional(*index)
        )),
        TemplateIdentifierSpec::Implicit(index) => Err(placeholder_error(
            placeholder.span,
            &TemplateIdentifierSpec::Implicit(*index)
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
        TemplateFormatter::Display {
            spec: Some(spec)
        } => {
            let format_literal = Literal::string(&format!("{{:{spec}}}"));
            quote! {
                ::core::write!(f, #format_literal, #expr)?;
            }
        }
        TemplateFormatter::Display {
            spec: None
        } => {
            format_with_formatter_kind(expr, pointer_value, TemplateFormatterKind::Display, false)
        }
        TemplateFormatter::Debug {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::Debug,
            alternate
        ),
        TemplateFormatter::LowerHex {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::LowerHex,
            alternate
        ),
        TemplateFormatter::UpperHex {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::UpperHex,
            alternate
        ),
        TemplateFormatter::Pointer {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::Pointer,
            alternate
        ),
        TemplateFormatter::Binary {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::Binary,
            alternate
        ),
        TemplateFormatter::Octal {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::Octal,
            alternate
        ),
        TemplateFormatter::LowerExp {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::LowerExp,
            alternate
        ),
        TemplateFormatter::UpperExp {
            alternate
        } => format_with_formatter_kind(
            expr,
            pointer_value,
            TemplateFormatterKind::UpperExp,
            alternate
        )
    }
}

fn format_with_formatter_kind(
    expr: TokenStream,
    pointer_value: bool,
    kind: TemplateFormatterKind,
    alternate: bool
) -> TokenStream {
    let trait_name = formatter_trait_name(kind);
    match kind {
        TemplateFormatterKind::Display => format_with_trait(expr, trait_name),
        TemplateFormatterKind::Pointer => {
            format_pointer(expr, pointer_value, alternate, trait_name)
        }
        _ => {
            if let Some(specifier) = formatter_specifier(kind) {
                format_with_optional_alternate(expr, trait_name, specifier, alternate)
            } else {
                format_with_trait(expr, trait_name)
            }
        }
    }
}

fn formatter_trait_name(kind: TemplateFormatterKind) -> &'static str {
    match kind {
        TemplateFormatterKind::Display => "Display",
        TemplateFormatterKind::Debug => "Debug",
        TemplateFormatterKind::LowerHex => "LowerHex",
        TemplateFormatterKind::UpperHex => "UpperHex",
        TemplateFormatterKind::Pointer => "Pointer",
        TemplateFormatterKind::Binary => "Binary",
        TemplateFormatterKind::Octal => "Octal",
        TemplateFormatterKind::LowerExp => "LowerExp",
        TemplateFormatterKind::UpperExp => "UpperExp"
    }
}

fn formatter_specifier(kind: TemplateFormatterKind) -> Option<char> {
    match kind {
        TemplateFormatterKind::Display | TemplateFormatterKind::Pointer => None,
        TemplateFormatterKind::Debug => Some('?'),
        TemplateFormatterKind::LowerHex => Some('x'),
        TemplateFormatterKind::UpperHex => Some('X'),
        TemplateFormatterKind::Binary => Some('b'),
        TemplateFormatterKind::Octal => Some('o'),
        TemplateFormatterKind::LowerExp => Some('e'),
        TemplateFormatterKind::UpperExp => Some('E')
    }
}

fn format_with_trait(expr: TokenStream, trait_name: &str) -> TokenStream {
    let trait_ident = format_ident!("{}", trait_name);
    quote! {
        ::core::fmt::#trait_ident::fmt(#expr, f)?;
    }
}

fn format_with_optional_alternate(
    expr: TokenStream,
    trait_name: &str,
    specifier: char,
    alternate: bool
) -> TokenStream {
    if alternate {
        format_with_alternate(expr, specifier)
    } else {
        format_with_trait(expr, trait_name)
    }
}

fn format_with_alternate(expr: TokenStream, specifier: char) -> TokenStream {
    let format_string = format!("{{:#{}}}", specifier);
    quote! {
        ::core::write!(f, #format_string, #expr)?;
    }
}

fn format_pointer(
    expr: TokenStream,
    pointer_value: bool,
    alternate: bool,
    trait_name: &str
) -> TokenStream {
    if alternate {
        format_with_alternate(expr, 'p')
    } else if pointer_value {
        let trait_ident = format_ident!("{}", trait_name);
        quote! {{
            let value = #expr;
            ::core::fmt::#trait_ident::fmt(&value, f)?;
        }}
    } else {
        format_with_trait(expr, trait_name)
    }
}

fn binding_ident(field: &Field) -> Ident {
    field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("__field{}", field.index, span = field.span))
}
