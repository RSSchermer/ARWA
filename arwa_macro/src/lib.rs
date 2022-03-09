#![feature(proc_macro_diagnostic)]

use arwa_parse::dom_token::Token;
use arwa_parse::request_method::RequestMethod;
use arwa_parse::selector::Selector;
use arwa_parse::xml_name::{Name, NonColonName, QualifiedName};
use oxilangtag::LanguageTag;
use proc_macro::{Diagnostic, Level, TokenStream};
use quote::quote;
use syn::{parse_macro_input, LitStr};
use url::{Origin, Url};

#[proc_macro]
pub fn lang(tokens_in: TokenStream) -> TokenStream {
    let lang_string = parse_macro_input!(tokens_in as LitStr);

    match LanguageTag::parse(lang_string.value()) {
        Ok(tag) => {
            let raw = tag.as_str();
            let raw_ptr = raw.as_ptr();

            let language_end = tag.primary_language().len();
            let extlang_end = if let Some(extlang) = tag.extended_language() {
                let ptr = extlang.as_ptr();
                let start = unsafe { ptr.offset_from(raw_ptr) };
                let end = start as usize + extlang.len();

                end
            } else {
                language_end
            };
            let script_end = if let Some(script) = tag.script() {
                let ptr = script.as_ptr();
                let start = unsafe { ptr.offset_from(raw_ptr) };
                let end = start as usize + script.len();

                end
            } else {
                extlang_end
            };
            let region_end = if let Some(region) = tag.region() {
                let ptr = region.as_ptr();
                let start = unsafe { ptr.offset_from(raw_ptr) };
                let end = start as usize + region.len();

                end
            } else {
                script_end
            };
            let variant_end = if let Some(variant) = tag.variant() {
                let ptr = variant.as_ptr();
                let start = unsafe { ptr.offset_from(raw_ptr) };
                let end = start as usize + variant.len();

                end
            } else {
                region_end
            };
            let extension_end = if let Some(extension) = tag.extension() {
                let ptr = extension.as_ptr();
                let start = unsafe { ptr.offset_from(raw_ptr) };
                let end = start as usize + extension.len();

                end
            } else {
                variant_end
            };

            let tokens_out = quote! {
                arwa::lang::LanguageTag::from_statically_parsed(arwa::lang::StaticallyParsedLanguageTag {
                    raw: #lang_string,
                    language_end: #language_end,
                    extlang_end: #extlang_end,
                    script_end: #script_end,
                    region_end: #region_end,
                    variant_end: #variant_end,
                    extension_end: #extension_end
                })
            };

            tokens_out.into()
        }
        Err(err) => {
            Diagnostic::spanned(lang_string.span().unwrap(), Level::Error, err.to_string()).emit();

            TokenStream::new()
        }
    }
}

#[proc_macro]
pub fn name(tokens_in: TokenStream) -> TokenStream {
    let name_string = parse_macro_input!(tokens_in as LitStr);

    if let Err(err) = Name::parse(&name_string.value()) {
        Diagnostic::spanned(name_string.span().unwrap(), Level::Error, err.to_string()).emit();
    }

    let tokens_out = quote! {
        arwa::dom::Name::from_statically_parsed(arwa::dom::StaticallyParsedName {
            name: #name_string
        })
    };

    tokens_out.into()
}

#[proc_macro]
pub fn non_colon_name(tokens_in: TokenStream) -> TokenStream {
    let name_string = parse_macro_input!(tokens_in as LitStr);

    if let Err(err) = NonColonName::parse(&name_string.value()) {
        Diagnostic::spanned(name_string.span().unwrap(), Level::Error, err.to_string()).emit();
    }

    let tokens_out = quote! {
        arwa::dom::NonColonName::from_statically_parsed(arwa::dom::StaticallyParsedNonColonName {
            name: #name_string
        })
    };

    tokens_out.into()
}

#[proc_macro]
pub fn qualified_name(tokens_in: TokenStream) -> TokenStream {
    let name_string = parse_macro_input!(tokens_in as LitStr);

    match QualifiedName::parse(&name_string.value()) {
        Ok(qualified_name) => {
            let colon_pos = if let Some(colon_pos) = qualified_name.colon_position() {
                quote!(Some(#colon_pos))
            } else {
                quote!(None)
            };

            let tokens_out = quote! {
                arwa::dom::QualifiedName::from_statically_parsed(arwa::dom::StaticallyParsedQualifiedName {
                    name: #name_string,
                    colon_pos: #colon_pos
                })
            };

            tokens_out.into()
        }
        Err(err) => {
            Diagnostic::spanned(name_string.span().unwrap(), Level::Error, err.to_string()).emit();

            TokenStream::new()
        }
    }
}

#[proc_macro]
pub fn request_method(tokens_in: TokenStream) -> TokenStream {
    let request_method_string = parse_macro_input!(tokens_in as LitStr);

    if let Err(err) = RequestMethod::parse(&request_method_string.value()) {
        Diagnostic::spanned(
            request_method_string.span().unwrap(),
            Level::Error,
            err.to_string(),
        )
        .emit();
    }

    let tokens_out = quote! {
        arwa::fetch::RequestMethod::from_statically_parsed(arwa::dom::StaticallyParsedRequestMethod {
            request_method: #request_method_string
        })
    };

    tokens_out.into()
}

#[proc_macro]
pub fn selector(tokens_in: TokenStream) -> TokenStream {
    let selector_string = parse_macro_input!(tokens_in as LitStr);

    if let Err(err) = Selector::parse(&selector_string.value()) {
        Diagnostic::spanned(
            selector_string.span().unwrap(),
            Level::Error,
            err.to_string(),
        )
        .emit();
    }

    let tokens_out = quote! {
        arwa::dom::Selector::from_statically_parsed(arwa::dom::StaticallyParsedSelector {
            selector: #selector_string
        })
    };

    tokens_out.into()
}

#[proc_macro]
pub fn token(tokens_in: TokenStream) -> TokenStream {
    let token_string = parse_macro_input!(tokens_in as LitStr);

    if let Err(err) = Token::parse(&token_string.value()) {
        Diagnostic::spanned(token_string.span().unwrap(), Level::Error, err.to_string()).emit();
    }

    let tokens_out = quote! {
        arwa::dom::Token::from_statically_parsed(arwa::dom::StaticallyParsedToken {
            token: #token_string
        })
    };

    tokens_out.into()
}

#[proc_macro]
pub fn url(tokens_in: TokenStream) -> TokenStream {
    let url_string = parse_macro_input!(tokens_in as LitStr);

    match Url::parse(&url_string.value()) {
        Ok(url) => {
            let raw = url.as_str();
            let raw_ptr = raw.as_ptr();

            // Derive component ranges from url::Url's component &str pointers. Note that url::Url
            // conventions do not match browser conventions for some components:
            //
            // - url::Url does not include the trailing `:` in scheme, browsers do.
            // - url::Url does not include the leading `?` in query, browsers do.
            // - url::Url does not include the leading `#` in fragment, browsers do.
            //
            // We adjust the ranges here to conform to the browser conventions.

            let scheme_end = url.scheme().len() + 1;

            let username = url.username();
            let username_range = if username.is_empty() {
                quote!(None)
            } else {
                let ptr = username.as_ptr();
                let start = unsafe { ptr.offset_from(raw_ptr) } as usize;
                let end = start + username.len();

                quote!(Some(#start..#end))
            };

            let password_range = if let Some(password) = url.password() {
                let ptr = password.as_ptr();
                let start = unsafe { ptr.offset_from(raw_ptr) } as usize;
                let end = start + password.len();

                quote!(Some(#start..#end))
            } else {
                quote!(None)
            };

            let host_range = if let Some(host) = url.host_str() {
                let ptr = host.as_ptr();
                let start = unsafe { ptr.offset_from(raw_ptr) } as usize;
                let end = start + host.len();

                quote!(Some(#start..#end))
            } else {
                quote!(None)
            };

            let port = if let Some(port) = url.port() {
                quote!(Some(#port))
            } else {
                quote!(None)
            };

            let path_start = unsafe { url.path().as_ptr().offset_from(raw_ptr) as usize };

            let query_start = if let Some(query) = url.query() {
                let start = unsafe { query.as_ptr().offset_from(raw_ptr) as usize - 1 };

                quote!(Some(#start))
            } else {
                quote!(None)
            };

            let fragment_start = if let Some(fragment) = url.fragment() {
                let start = unsafe { fragment.as_ptr().offset_from(raw_ptr) as usize - 1 };

                quote!(Some(#start))
            } else {
                quote!(None)
            };

            let origin_kind = match url.origin() {
                Origin::Opaque(_) => quote!(arwa::url::OriginKind::Opaque),
                Origin::Tuple(_, _, _) => quote!(arwa::url::OriginKind::Tuple),
            };

            let tokens_out = quote! {
                arwa::url::Url::from_statically_parsed(arwa::url::StaticallyParsedUrl {
                    raw: #raw,
                    scheme_end: #scheme_end,
                    username_range: #username_range,
                    password_range: #password_range,
                    host_range: #host_range,
                    port: #port,
                    path_start: #path_start,
                    query_start: #query_start,
                    fragment_start: #fragment_start,
                    origin_kind: #origin_kind,
                })
            };

            tokens_out.into()
        }
        Err(err) => {
            Diagnostic::spanned(url_string.span().unwrap(), Level::Error, err.to_string()).emit();

            TokenStream::new()
        }
    }
}
