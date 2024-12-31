#[proc_macro]
pub fn custom(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_macro2::create_proc_macro2().into()
}
