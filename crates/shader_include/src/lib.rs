mod generator;
mod input;
mod layout;

use proc_macro::TokenStream;

#[proc_macro]
pub fn include_spirv(stream: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(
        stream as input::IncludeSpirvInput
    );

    generator::Generator::load(input).into()
}
