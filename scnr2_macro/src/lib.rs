use proc_macro::TokenStream;

#[proc_macro]
pub fn scanner(input: TokenStream) -> TokenStream {
    scnr2_generate::generate::generate(input.into()).into()
}
