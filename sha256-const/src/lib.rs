#[proc_macro]
pub fn sha_256_hash_from_file_path(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match syn::parse::<syn::LitStr>(input) {
        Ok(literal) => {
            let string_literal_value = literal.value();
            let vec_result = std::fs::read(string_literal_value);
            match vec_result {
                Ok(vec) => {
                    use sha2::Digest;

                    let mut sha256 = sha2::Sha256::new();
                    sha256.update(vec);
                    let hex_string = format!("{:x}", sha256.finalize());
                    quote::quote! {
                        #hex_string
                    }.into()
                }
                Err(_) => quote::quote! {compile_error!("not found")}.into(),
            }
        }
        Err(_) => quote::quote! {compile_error!("sha_256_hash_from_file_path macro support string literal")}.into(),
    }
}
