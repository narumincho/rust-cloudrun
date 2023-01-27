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
    type_: proc_macro2::TokenStream,
    default_value: proc_macro2::TokenStream,
}

/// https://html.spec.whatwg.org/multipage/dom.html#global-attributes
const ATTRIBUTES: Lazy<Vec<AttributeData>> = Lazy::new(|| {
    vec![
        AttributeData {
            name: proc_macro2::Ident::new("accesskey", proc_macro2::Span::call_site()),
            type_: quote::quote!(Vec<char>),
            default_value: quote::quote!(vec![]),
        },
        AttributeData {
            name: proc_macro2::Ident::new("autofocus", proc_macro2::Span::call_site()),
            type_: quote::quote!(bool),
            default_value: quote::quote!(false),
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
                )
            )
        );
    }
}
