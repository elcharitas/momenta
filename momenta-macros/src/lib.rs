use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Colon;
use syn::{
    Block, Expr, ExprLit, Ident, ItemFn, Lit, LitStr, Result, Token,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    token::Brace,
};
use syn::{ExprLet, FnArg, PatType, Signature, Type, TypeReference};
use syn::{Stmt, braced};

/// Parse either an identifier or a keyword token as an identifier
/// Keywords get converted to identifiers with _ suffix
/// Also handles on:eventname patterns, converting them to on_eventname
///
/// Examples:
/// - `type` -> `type_`
/// - `loop` -> `loop_`
/// - `on:click` -> `on_click`
/// - `on:keydown` -> `on_keydown`
fn parse_attribute_name(input: ParseStream) -> Result<(Ident, Span)> {
    let start_span = input.span();

    // Try to parse as regular identifier first
    if let Ok(ident) = input.parse::<Ident>() {
        // Check for prefixed attributes (on:, data:)
        if (ident == "on" || ident == "data") && input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            let suffix_ident = input.parse::<Ident>()?;
            let suffix_name = suffix_ident.to_string();
            let end_span = suffix_ident.span();
            let combined_span = start_span.join(end_span).unwrap_or(end_span);
            return Ok((
                Ident::new(&format!("{}_{}", ident, suffix_name), combined_span),
                combined_span,
            ));
        }
        let span = ident.span();
        return Ok((ident, span));
    }

    // Handle specific keyword tokens
    let lookahead = input.lookahead1();
    if lookahead.peek(Token![type]) {
        let token = input.parse::<Token![type]>()?;
        Ok((Ident::new("type_", token.span), token.span))
    } else if lookahead.peek(Token![loop]) {
        let token = input.parse::<Token![loop]>()?;
        Ok((Ident::new("loop_", token.span), token.span))
    } else if lookahead.peek(Token![for]) {
        let token = input.parse::<Token![for]>()?;
        Ok((Ident::new("for_", token.span), token.span))
    } else if lookahead.peek(Token![if]) {
        let token = input.parse::<Token![if]>()?;
        Ok((Ident::new("if_", token.span), token.span))
    } else if lookahead.peek(Token![else]) {
        let token = input.parse::<Token![else]>()?;
        Ok((Ident::new("else_", token.span), token.span))
    } else if lookahead.peek(Token![while]) {
        let token = input.parse::<Token![while]>()?;
        Ok((Ident::new("while_", token.span), token.span))
    } else if lookahead.peek(Token![match]) {
        let token = input.parse::<Token![match]>()?;
        Ok((Ident::new("match_", token.span), token.span))
    } else if lookahead.peek(Token![const]) {
        let token = input.parse::<Token![const]>()?;
        Ok((Ident::new("const_", token.span), token.span))
    } else if lookahead.peek(Token![static]) {
        let token = input.parse::<Token![static]>()?;
        Ok((Ident::new("static_", token.span), token.span))
    } else if lookahead.peek(Token![mut]) {
        let token = input.parse::<Token![mut]>()?;
        Ok((Ident::new("mut_", token.span), token.span))
    } else if lookahead.peek(Token![let]) {
        let token = input.parse::<Token![let]>()?;
        Ok((Ident::new("let_", token.span), token.span))
    } else if lookahead.peek(Token![fn]) {
        let token = input.parse::<Token![fn]>()?;
        Ok((Ident::new("fn_", token.span), token.span))
    } else if lookahead.peek(Token![struct]) {
        let token = input.parse::<Token![struct]>()?;
        Ok((Ident::new("struct_", token.span), token.span))
    } else if lookahead.peek(Token![enum]) {
        let token = input.parse::<Token![enum]>()?;
        Ok((Ident::new("enum_", token.span), token.span))
    } else if lookahead.peek(Token![trait]) {
        let token = input.parse::<Token![trait]>()?;
        Ok((Ident::new("trait_", token.span), token.span))
    } else if lookahead.peek(Token![impl]) {
        let token = input.parse::<Token![impl]>()?;
        Ok((Ident::new("impl_", token.span), token.span))
    } else if lookahead.peek(Token![mod]) {
        let token = input.parse::<Token![mod]>()?;
        Ok((Ident::new("mod_", token.span), token.span))
    } else if lookahead.peek(Token![use]) {
        let token = input.parse::<Token![use]>()?;
        Ok((Ident::new("use_", token.span), token.span))
    } else if lookahead.peek(Token![pub]) {
        let token = input.parse::<Token![pub]>()?;
        Ok((Ident::new("pub_", token.span), token.span))
    } else if lookahead.peek(Token![crate]) {
        let token = input.parse::<Token![crate]>()?;
        Ok((Ident::new("crate_", token.span), token.span))
    } else if lookahead.peek(Token![super]) {
        let token = input.parse::<Token![super]>()?;
        Ok((Ident::new("super_", token.span), token.span))
    } else if lookahead.peek(Token![self]) {
        let token = input.parse::<Token![self]>()?;
        Ok((Ident::new("self_", token.span), token.span))
    } else if lookahead.peek(Token![Self]) {
        let token = input.parse::<Token![Self]>()?;
        Ok((Ident::new("Self_", token.span), token.span))
    } else if lookahead.peek(Token![extern]) {
        let token = input.parse::<Token![extern]>()?;
        Ok((Ident::new("extern_", token.span), token.span))
    } else if lookahead.peek(Token![return]) {
        let token = input.parse::<Token![return]>()?;
        Ok((Ident::new("return_", token.span), token.span))
    } else if lookahead.peek(Token![break]) {
        let token = input.parse::<Token![break]>()?;
        Ok((Ident::new("break_", token.span), token.span))
    } else if lookahead.peek(Token![continue]) {
        let token = input.parse::<Token![continue]>()?;
        Ok((Ident::new("continue_", token.span), token.span))
    } else if lookahead.peek(Token![move]) {
        let token = input.parse::<Token![move]>()?;
        Ok((Ident::new("move_", token.span), token.span))
    } else if lookahead.peek(Token![ref]) {
        let token = input.parse::<Token![ref]>()?;
        Ok((Ident::new("ref_", token.span), token.span))
    } else if lookahead.peek(Token![where]) {
        let token = input.parse::<Token![where]>()?;
        Ok((Ident::new("where_", token.span), token.span))
    } else if lookahead.peek(Token![unsafe]) {
        let token = input.parse::<Token![unsafe]>()?;
        Ok((Ident::new("unsafe_", token.span), token.span))
    } else if lookahead.peek(Token![as]) {
        let token = input.parse::<Token![as]>()?;
        Ok((Ident::new("as_", token.span), token.span))
    } else if lookahead.peek(Token![in]) {
        let token = input.parse::<Token![in]>()?;
        Ok((Ident::new("in_", token.span), token.span))
    } else {
        Err(lookahead.error())
    }
}

/// A procedural macro that transforms a conditional expression into a JSX-like syntax.
/// Supports both RSX nodes and literals, with conditional and match syntax.
/// Also supports let statements in conditions and braces/brackets for values.
///
/// # Examples
/// ```rust
/// use momenta::prelude::*;
///
/// let show = true;
/// let result: Result<i32, String> = Ok(42);
/// let option_value: Option<String> = Some("hello".to_string());
///
/// // Conditional syntax with RSX nodes
/// when!(show => <p>"Show me"</p>);
/// when!(show => <p>"Show me"</p> else <p>"Hidden"</p>);
///
/// // Conditional syntax with literals
/// when!(show => "Visible text");
/// when!(show => "Visible" else "Hidden");
///
/// // Support for let patterns in conditions (if let style)
/// when!(let Some(value) = option_value => <div>{value}</div>);
/// when!(let Ok(val) = result => <div>"Success: "{val}</div> else <div>"Error"</div>);
/// // when!(let Some(x) = get_option() => format!("Got: {}", x) else ("Nothing".to_string()));
///
/// // Support for blocks as values
/// let condition = true;
/// when!(condition => {
///     let msg = "Complex computation";
///     format!("{} result", msg)
/// });
///
/// // Support for arrays/vectors as values
/// let show_list = false;
/// when!(show_list => [1, 2, 3, 4] else [0, 8, 4, 6]);
/// ```
#[proc_macro]
pub fn when(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Either);
    let expanded = input.to_tokens();
    expanded.into()
}

enum EitherValue {
    RsxNode(RsxNode),
    Literal(Lit),
    Block(Block),
    Expression(Expr), // for arrays, function calls, etc.
}

impl Parse for EitherValue {
    fn parse(input: ParseStream) -> Result<Self> {
        // Try to parse as a literal first (simplest case)
        if let Ok(lit) = input.parse::<Lit>() {
            return Ok(EitherValue::Literal(lit));
        }

        // Try to parse as a block
        if input.peek(Brace) {
            let block = input.parse::<Block>()?;
            return Ok(EitherValue::Block(block));
        }

        // Try to parse as RSX node (starts with <)
        if input.peek(Token![<]) {
            let rsx_node = input.parse::<RsxNode>()?;
            return Ok(EitherValue::RsxNode(rsx_node));
        }

        // Parse as a general expression (arrays, function calls, etc.)
        let expr = input.parse::<Expr>()?;
        Ok(EitherValue::Expression(expr))
    }
}

impl EitherValue {
    fn to_tokens(&self) -> TokenStream2 {
        match self {
            EitherValue::RsxNode(node) => node.to_tokens(),
            EitherValue::Literal(lit) => {
                let span = lit.span();
                quote_spanned! { span=> #lit }
            }
            EitherValue::Block(block) => {
                let span = block.span();
                quote_spanned! { span=> #block }
            }
            EitherValue::Expression(expr) => {
                let span = expr.span();
                quote_spanned! { span=> #expr }
            }
        }
    }
}

struct MatchArm {
    pattern: Expr,
    value: EitherValue,
}

impl Parse for MatchArm {
    fn parse(input: ParseStream) -> Result<Self> {
        let pattern = input.parse()?;
        input.parse::<Token![=>]>()?;
        let value = input.parse()?;
        Ok(MatchArm { pattern, value })
    }
}

impl MatchArm {
    fn to_tokens(&self) -> TokenStream2 {
        let pattern = &self.pattern;
        let value = self.value.to_tokens();
        quote! { #pattern => #value }
    }
}

enum ConditionType {
    Regular(Expr),
    LetPattern(ExprLet),
}

enum Either {
    Conditional {
        condition: ConditionType,
        true_value: EitherValue,
        false_value: Option<EitherValue>,
    },
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
    },
}

impl Parse for Either {
    fn parse(input: ParseStream) -> Result<Self> {
        // Check if we start with 'let' for if-let style syntax
        if input.peek(Token![let]) {
            let expr = input.parse::<ExprLet>()?;

            // Must be followed by '=>'
            input.parse::<Token![=>]>()?;
            let true_value = input.parse()?;
            let false_value = if input.peek(Token![else]) {
                input.parse::<Token![else]>()?;
                Some(input.parse()?)
            } else {
                None
            };

            return Ok(Either::Conditional {
                condition: ConditionType::LetPattern(expr),
                true_value,
                false_value,
            });
        }

        // Parse regular expression
        let expr = input.parse::<Expr>()?;

        // Check if we have conditional syntax (=>) or match syntax ({)
        if input.peek(Token![=>]) {
            // Conditional syntax
            input.parse::<Token![=>]>()?;
            let true_value = input.parse()?;
            let false_value = if input.peek(Token![else]) {
                input.parse::<Token![else]>()?;
                Some(input.parse()?)
            } else {
                None
            };

            Ok(Either::Conditional {
                condition: ConditionType::Regular(expr),
                true_value,
                false_value,
            })
        } else if input.peek(Brace) {
            // Match syntax
            let content;
            braced!(content in input);

            let arms: Punctuated<MatchArm, Token![,]> =
                content.parse_terminated(MatchArm::parse, Token![,])?;

            Ok(Either::Match {
                expr,
                arms: arms.into_iter().collect(),
            })
        } else {
            Err(input.error("Expected '=>' for conditional or '{' for match syntax"))
        }
    }
}

impl Either {
    fn to_tokens(&self) -> TokenStream2 {
        match self {
            Either::Conditional {
                condition,
                true_value,
                false_value,
            } => {
                let true_tokens = true_value.to_tokens();
                let false_tokens = false_value.as_ref().map(|v| v.to_tokens());

                match condition {
                    ConditionType::Regular(cond_expr) => {
                        if let Some(false_tokens) = false_tokens {
                            quote! {
                                if #cond_expr {
                                    #true_tokens
                                } else {
                                    #false_tokens
                                }
                            }
                        } else {
                            quote! {
                                (#cond_expr).then(|| #true_tokens)
                            }
                        }
                    }
                    ConditionType::LetPattern(expr) => {
                        if let Some(false_tokens) = false_tokens {
                            quote! {
                                if #expr {
                                    #true_tokens
                                } else {
                                    #false_tokens
                                }
                            }
                        } else {
                            quote! {
                                if #expr {
                                    Some(#true_tokens)
                                } else {
                                    None
                                }
                            }
                        }
                    }
                }
            }
            Either::Match { expr, arms } => {
                let arm_tokens: Vec<_> = arms.iter().map(|arm| arm.to_tokens()).collect();
                quote! {
                    match #expr {
                        #(#arm_tokens),*
                    }
                }
            }
        }
    }
}

/// A procedural macro that transforms a rust function into a component.
///
/// # Examples
///
/// ```rust
/// use momenta::prelude::*;
///
/// #[component]
/// fn HelloWorld() -> Node {
///     rsx!(<div>Hello World</div>)
/// }
/// ```
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        vis,
        attrs,
        sig,
        block,
    } = parse_macro_input!(input as ItemFn);
    let Signature {
        ident,
        asyncness,
        constness,
        unsafety,
        mut inputs,
        output,
        fn_token,
        ..
    } = sig;

    if asyncness.is_some() || constness.is_some() || unsafety.is_some() {
        panic!("async, const, and unsafe functions are not supported");
    }

    if inputs.len() > 1 {
        panic!("Components can only take a single prop as input");
    }

    let prop_ty = inputs
        .iter()
        .map(|input| match input {
            FnArg::Typed(PatType { ty, .. }) => match &**ty {
                Type::Reference(TypeReference { elem, .. }) => elem,
                _ => ty,
            },
            _ => panic!("Only typed inputs are supported"),
        })
        .next();

    let prop_type = if let Some(prop_ty) = prop_ty {
        quote! {type Props = #prop_ty;}
    } else {
        quote! {type Props = ::momenta::nodes::DefaultProps;}
    };

    if inputs.is_empty() {
        inputs.push(FnArg::Typed(PatType {
            attrs: Vec::new(),
            pat: parse_quote!(_),
            colon_token: Colon::default(),
            ty: parse_quote!(&Self::Props),
        }));
    }

    let expanded = quote! {
        #vis #(#attrs)* struct #ident;

        impl ::momenta::nodes::Component for #ident {
            #prop_type
            #fn_token render(#inputs) #output #block
        }
    };

    expanded.into()
}

#[proc_macro_derive(SignalValue)]
pub fn derive_signal_value(input: TokenStream) -> TokenStream {
    let syn::DeriveInput {
        ident, generics, ..
    } = syn::parse_macro_input!(input as syn::DeriveInput);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::momenta::signals::SignalValue for #ident #ty_generics #where_clause {
            fn as_any(&self) -> Option<&dyn core::any::Any> {
                Some(self)
            }
        }
    };
    expanded.into()
}

/// A procedural macro that provides JSX-like syntax for creating HTML elements in Rust.
///
/// # Examples
///
/// ```rust ignore
/// use momenta::prelude::*;
/// // Fragment
/// rsx!(<>"Hello World"</>);
///
/// // Self-closing tag
/// rsx!(<div class="container" id="app" />);
///
/// // Tag with children
/// rsx!(<div class="container">
///     <h1>Title</h1>
///     <p>Paragraph text</p>
/// </div>);
///
/// // Expression
/// let name = "World";
/// rsx!(<div>Hello {name}</div>);
///
/// // Event handlers with on:eventname syntax
/// rsx!(<button on:click={handle_click}>Click me</button>);
///
/// // Keyword attributes (automatically converted with _ suffix)
/// rsx!(<input type="text" for="name" />);
/// ```
#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RsxNode);
    let expanded = input.to_tokens();
    expanded.into()
}

/// Represents the different types of JSX nodes
#[derive(Debug)]
enum RsxNode {
    Fragment(Vec<RsxNode>),
    Component {
        name: Ident,
        props: Vec<(Option<Ident>, Option<Expr>, Span)>,
        children: Vec<RsxNode>,
        close_tag: Option<Ident>,
        open_span: Span,
        close_span: Option<Span>,
    },
    Text(Expr),
    Block(Block),
    Empty,
    Comment(Expr), // HTML comments
}

/// Represents an attribute name-value pair
struct NodeValue {
    name: Option<Ident>,
    expr: Option<Expr>,
    span: Span,
}

impl Parse for NodeValue {
    fn parse(input: ParseStream) -> Result<Self> {
        // Handle `{ident}` and `{..ident}` patterns
        if input.peek(Brace) {
            let content;
            let brace_token = braced!(content in input);
            let full_span = brace_token.span.join();

            // Check for `{..ident}` pattern
            if content.peek(Token![..]) {
                let dotdot = content.parse::<Token![..]>()?;
                let ident: Expr = content.parse()?;
                // Create token stream for `..ident` manually
                let mut tokens = dotdot.to_token_stream();
                tokens.extend(ident.to_token_stream());

                return Ok(NodeValue {
                    span: full_span,
                    name: None,
                    expr: Some(syn::Expr::Verbatim(tokens)),
                });
            }

            // Handle `{expression}` pattern
            let parsed: Ident = content.parse()?;
            return Ok(NodeValue {
                span: parsed.span(),
                expr: Some(syn::Expr::Verbatim(parsed.to_token_stream())),
                name: Some(parsed),
            });
        }

        // Handle `name={expression or block}` and `name` patterns
        let (name, name_span) = parse_attribute_name(input)?;

        // If no `=`, just return the name
        if !input.peek(Token![=]) {
            return Ok(NodeValue {
                span: name_span,
                name: Some(name),
                expr: None,
            });
        }

        // Parse the `=` and then the expression/block
        input.parse::<Token![=]>()?;

        // check if next token is a literal
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            let expr: Expr = parse_quote! {#lit};
            let full_span = name_span.join(lit.span()).unwrap_or(lit.span());
            return Ok(NodeValue {
                span: full_span,
                name: Some(name),
                expr: Some(expr),
            });
        }

        // Parse any expression (including braced blocks)
        let block: Block = input.parse()?;
        let full_span = name_span.join(block.span()).unwrap_or(block.span());
        let expr = block.stmts.into_iter().next();
        let expr = match expr {
            Some(Stmt::Expr(expr, _)) => expr,
            _ => panic!("Expected expression"),
        };

        Ok(NodeValue {
            span: full_span,
            name: Some(name),
            expr: Some(expr),
        })
    }
}

struct RsxChildren {
    children: Vec<RsxNode>,
}

impl Parse for RsxChildren {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut children = Vec::with_capacity(4);
        let mut last_end = 0;
        while !(input.is_empty() || input.peek(Token![<]) && input.peek2(Token![/])) {
            if let Ok(child) = input.parse() {
                children.push(child);
                continue;
            }

            let span_info = format!("{:?}", input.span());
            let (start, end) = parse_range(&span_info).unwrap_or((0, 0));
            let mut value = String::new();
            let token = input.parse::<proc_macro2::TokenTree>()?;

            if !matches!(token, proc_macro2::TokenTree::Punct(_)) {
                let gap_size = start - last_end;
                if gap_size > 0 && last_end > 0 {
                    // Add spaces to represent the gap
                    value.push_str(&" ".repeat(gap_size));
                }
            }
            value.push_str(&token.to_string());

            children.push(RsxNode::Text(syn::Expr::Lit(ExprLit {
                attrs: Vec::new(),
                lit: Lit::Str(LitStr::new(&value, token.span())),
            })));
            last_end = end;
        }

        Ok(RsxChildren { children })
    }
}

impl Parse for RsxNode {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(RsxNode::Empty);
        }

        // Look ahead to see if we start with a '<'
        if input.peek(Token![<]) {
            let open_bracket_span = input.span();
            input.parse::<Token![<]>()?;

            // Comments: <!-- ... -->
            if input.peek(Token![!]) && input.peek2(Token![-]) && input.peek3(Token![-]) {
                input.parse::<Token![!]>()?;
                input.parse::<Token![-]>()?;
                input.parse::<Token![-]>()?;

                let mut nodes = Vec::new();
                while !(input.is_empty()
                    || input.peek(Token![-]) && input.peek2(Token![-]) && input.peek3(Token![>]))
                {
                    let mut comment = String::new();
                    let token = input.parse::<proc_macro2::TokenTree>()?;
                    comment.push(' ');
                    comment.push_str(&token.to_string());
                    nodes.push(LitStr::new(&comment, token.span()));
                }

                let token = input.parse::<Token![-]>()?;
                nodes.push(LitStr::new(" ", token.span()));
                input.parse::<Token![-]>()?;
                input.parse::<Token![>]>()?;

                // concat all nodes into a single lit
                // Convert each LitStr to an Expr::Lit
                let exprs: Vec<Expr> = nodes
                    .into_iter()
                    .map(|lit| {
                        Expr::Lit(syn::ExprLit {
                            attrs: vec![],
                            lit: syn::Lit::Str(lit),
                        })
                    })
                    .collect();

                // Build a binary expression tree with the + operator
                let mut result = syn::parse_str::<Expr>("String::new()").unwrap();

                for expr in exprs.into_iter() {
                    result = Expr::Binary(syn::ExprBinary {
                        attrs: vec![],
                        left: Box::new(result),
                        op: syn::BinOp::Add(syn::token::Plus::default()),
                        right: Box::new(expr),
                    });
                }

                return Ok(RsxNode::Comment(result));
            }

            // Fragment: <>...</>
            if input.peek(Token![>]) {
                input.parse::<Token![>]>()?;

                let RsxChildren { children } = input.parse()?;

                input.parse::<Token![<]>()?;
                input.parse::<Token![/]>()?;
                input.parse::<Token![>]>()?;

                return Ok(RsxNode::Fragment(children));
            }

            // Element: <tag ...>...</tag> or <tag ... />
            let tag = input.parse::<Ident>()?;
            let tag_span = tag.span();
            let open_start_span = open_bracket_span
                .join(tag_span)
                .unwrap_or(open_bracket_span);

            let mut attributes = Vec::with_capacity(4);
            while !input.peek(Token![>]) && !input.peek(Token![/]) {
                let NodeValue { name, expr, span } = input.parse::<NodeValue>()?;
                attributes.push((name, expr, span));
            }

            // Self-closing tag: <tag ... /> or <Component... />
            if input.peek(Token![/]) {
                input.parse::<Token![/]>()?;
                let close_bracket = input.parse::<Token![>]>()?;
                let close_span = close_bracket.span;
                let full_open_span = open_start_span.join(close_span).unwrap_or(open_start_span);

                return Ok(RsxNode::Component {
                    name: tag.clone(),
                    props: attributes,
                    children: Vec::new(),
                    close_tag: None,
                    open_span: full_open_span,
                    close_span: Some(close_span),
                });
            }

            // Opening tag ends: <tag ...>
            let open_close_bracket = input.parse::<Token![>]>()?;
            let full_open_span = open_start_span
                .join(open_close_bracket.span)
                .unwrap_or(open_start_span);

            let RsxChildren { children } = input.parse()?;

            // Closing tag: </tag>
            let close_open_bracket = input.parse::<Token![<]>()?;
            input.parse::<Token![/]>()?;
            let close_tag = input.parse::<Ident>()?;

            // Validate matching tags
            if tag != close_tag {
                return Err(syn::Error::new(
                    close_tag.span(),
                    format!(
                        "Closing tag </{}> doesn't match opening tag <{}>\n\
                        help: change the closing tag to </{}>",
                        close_tag, tag, tag
                    ),
                ));
            }

            let close_bracket = input.parse::<Token![>]>()?;
            let close_span = close_open_bracket
                .span
                .join(close_bracket.span)
                .unwrap_or(close_bracket.span);

            return Ok(RsxNode::Component {
                name: tag,
                props: attributes,
                children,
                close_tag: Some(close_tag),
                open_span: full_open_span,
                close_span: Some(close_span),
            });
        }

        // Text content or expression
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            let expr = parse_quote! {#lit};
            return Ok(RsxNode::Text(expr));
        }

        // Handle expressions wrapped in braces: {expression}
        if input.peek(Brace) {
            let content;
            braced!(content in input);
            let expr: Expr = content.parse()?;
            return Ok(RsxNode::Text(expr));
        }

        match input.parse::<Block>() {
            Ok(block) => Ok(RsxNode::Block(block)),
            Err(_) => Err(syn::Error::new(
                Span::call_site(),
                "Invalid RSX syntax\n\
                help: expected one of:\n\
                  - an HTML element: <div>...</div>\n\
                  - a string literal: \"text\"\n\
                  - an expression: {value}\n\
                  - a fragment: <>...</>",
            )),
        }
    }
}

impl RsxNode {
    fn to_tokens(&self) -> TokenStream2 {
        match self {
            RsxNode::Component {
                name,
                props,
                children,
                close_tag,
                open_span,
                close_span,
            } => {
                let is_element = name.to_string().starts_with(|c: char| !c.is_uppercase());

                let attrs = props
                    .iter() // filter out data- attributes for elements
                    .map(|(name, value, span)| {
                        let value = value
                            .as_ref()
                            .map(|v| {
                                let span = v.span();
                                quote_spanned! { span=> #v}
                            })
                            .or_else(|| Some(quote! {true}));
                        (name, value, *span)
                    });

                let data_props = (is_element
                    && props.iter().any(|(name, _, _)| {
                        name.as_ref()
                            .map(|name| name.to_string().starts_with("data_"))
                            .unwrap_or(false)
                    }))
                .then(|| {
                    let timestamp = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                        .as_nanos()
                        .to_string();
                    let ident = syn::Ident::new(&format!("attr_data_{}", timestamp), *open_span);
                    let data = attrs
                        .clone()
                        .filter(|(name, _, _)| {
                            name.as_ref()
                                .map(|name| name.to_string().starts_with("data_"))
                                .unwrap_or(false)
                        })
                        .map(|(name, value, span)| {
                            quote_spanned! {span=>
                                {
                                    let #name = #value;
                                    #ident.push((stringify!(#name).to_string(), #name.value()));
                                }
                            }
                        });
                    quote_spanned! { *open_span=>
                        data_: {
                            let mut #ident = vec![];
                            {
                                #[allow(unused)]
                                use ::momenta::nodes::Attribute;
                                #(#data)*
                            }
                            #ident
                        },
                    }
                });

                let props_tokens = attrs
                    .filter(|(name, _, _)| {
                        !(is_element
                            // filter out data- attributes for elements
                            && name
                                .as_ref()
                                .map(|name| name.to_string().starts_with("data_"))
                                .unwrap_or(false))
                    }) // filter out data- attributes for elements
                    .map(|(name, value, span)| {
                        if name.is_none() {
                            return quote_spanned! {span=> #value };
                        }
                        quote_spanned! {span=> #name: {#value}.into(), }
                    });

                let children_tokens = if !children.is_empty() || is_element {
                    let child_tokens = children.iter().map(|child| child.to_tokens());
                    Some(quote_spanned! { *open_span=>
                        children: vec![#(#child_tokens),*],
                    })
                } else {
                    None
                };

                let close_tag_validation =
                    close_tag
                        .as_ref()
                        .zip(*close_span)
                        .map(|(close_tag, close_span)| {
                            let close = if is_element {
                                quote_spanned! { close_span=> momenta::dom::elements::#close_tag }
                            } else {
                                quote_spanned! { close_span=> #close_tag }
                            };
                            quote_spanned! { close_span=>
                                {
                                    let _ = #close;
                                };
                            }
                        });

                let default_props =
                    is_element.then(|| quote_spanned! { *open_span=> ..Default::default()});

                let component = if !is_element {
                    quote_spanned! { *open_span=> #name }
                } else {
                    quote_spanned! { *open_span=> momenta::dom::elements::#name }
                };

                quote_spanned! { *open_span=>
                    {
                        type Props = <#component as ::momenta::nodes::Component>::Props;
                        {
                            #close_tag_validation
                            ::momenta::dom::component::<#component>(
                                Props {
                                    #(#props_tokens)*
                                    #children_tokens
                                    #data_props
                                    #default_props
                                }
                            )
                        }
                    }
                }
            }
            RsxNode::Fragment(children) => {
                let children_tokens = children.iter().map(|child| child.to_tokens());

                quote! {
                    {
                        ::momenta::nodes::Node::Fragment(vec![#(#children_tokens),*])
                    }
                }
            }
            RsxNode::Text(expr) => {
                quote! {
                    {
                        ::momenta::nodes::Node::from(#expr)
                    }
                }
            }
            RsxNode::Empty => {
                quote! {
                    ::momenta::nodes::Node::Empty
                }
            }
            RsxNode::Comment(expr) => {
                let span = expr.span();
                quote_spanned! { span=>
                    ::momenta::nodes::Node::Comment(#expr)
                }
            }
            RsxNode::Block(block) => {
                let span = block.span();
                quote_spanned! { span=>
                    ::momenta::nodes::Node::from(#block)
                }
            }
        }
    }
}

fn parse_range(input: &str) -> Option<(usize, usize)> {
    use regex::Regex;
    let re = Regex::new(r"(\d+)\.\.(\d+)").ok()?;
    let captures = re.captures(input)?;
    let start = captures.get(1)?.as_str().parse::<usize>().ok()?;
    let end = captures.get(2)?.as_str().parse::<usize>().ok()?;

    Some((start, end))
}
