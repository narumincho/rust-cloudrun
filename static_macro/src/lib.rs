#[proc_macro]
pub fn tower_png_path(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_macro2::tower_png_path().into()
}

#[proc_macro]
pub fn client_js_path(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_macro2::client_js_path().into()
}

#[proc_macro]
pub fn client_wasm_bg_path(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_macro2::client_wasm_bg_path().into()
}

#[proc_macro]
pub fn custom(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    static_macro2::create_proc_macro2().into()
}
