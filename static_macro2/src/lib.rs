use once_cell::sync::Lazy;

pub fn create_proc_macro2() -> proc_macro2::TokenStream {
    let struct_fields = (*ATTRIBUTES)
        .iter()
        .map(|attribute| {
            let name = &attribute.name;
            let type_ = &attribute.type_;
            quote::quote!( #name: #type_ )
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    let default_fields = (*ATTRIBUTES)
        .iter()
        .map(|attribute| {
            let name = &attribute.name;
            let default_value = &attribute.default_value;
            quote::quote!( #name: #default_value )
        })
        .collect::<Vec<proc_macro2::TokenStream>>();

    quote::quote! {
        struct Html {
            #(#struct_fields,)*
        }

        const HTML_DEFAULT: Html = Html {
            #(#default_fields,)*
        };
    }
}

struct AttributeData {
    name: proc_macro2::Ident,
    type_: syn::Type,
    default_value: syn::Expr,
}

/// https://html.spec.whatwg.org/multipage/dom.html#global-attributes
const ATTRIBUTES: Lazy<Vec<AttributeData>> = Lazy::new(|| {
    vec![
        AttributeData {
            name: proc_macro2::Ident::new("accesskey", proc_macro2::Span::call_site()),
            type_: syn::parse_quote!(Vec<char>),
            default_value: syn::parse_quote!(Vec::new()),
        },
        AttributeData {
            name: proc_macro2::Ident::new("autofocus", proc_macro2::Span::call_site()),
            type_: syn::parse_quote!(bool),
            default_value: syn::parse_quote!(false),
        },
    ]
});

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            format!("{}", crate::create_proc_macro2()),
            format!(
                "{}",
                quote::quote!(
                    struct Html {
                        accesskey: Vec<char>,
                        autofocus: bool,
                    }

                    const HTML_DEFAULT: Html = Html {
                        accesskey: Vec::new(),
                        autofocus: false,
                    };
                )
            )
        );
    }

    #[test]
    fn sum() {
        assert_eq!((*crate::ATTRIBUTES).len(), 2)
    }
}

pub fn tower_png_path() -> proc_macro2::TokenStream {
    let path: String = {
        use sha2::Digest;
        let mut sha256 = sha2::Sha256::new();
        sha256.update(include_bytes!("../../assets/tower.png"));
        format!("/{:x}", sha256.finalize())
    };

    quote::quote!(#path)
}

pub fn client_js_path() -> proc_macro2::TokenStream {
    let path: String = {
        use sha2::Digest;
        let mut sha256 = sha2::Sha256::new();
        sha256.update(include_bytes!("../../client/pkg/client.js"));
        format!("/{:x}", sha256.finalize())
    };

    quote::quote!(#path)
}

pub fn client_wasm_bg_path() -> proc_macro2::TokenStream {
    let path: String = {
        use sha2::Digest;
        let mut sha256 = sha2::Sha256::new();
        sha256.update(include_bytes!("../../client/pkg/client_bg.wasm"));
        format!("/{:x}", sha256.finalize())
    };

    quote::quote!(#path)
}
