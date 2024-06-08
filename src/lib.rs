use proc_macro::TokenStream;
use quote::quote;

/// Embed a file into the binary as a string.
/// This is useful for HTML files or other static files
/// that need to be represented as a string.
/// 
/// The path is derived relative to the project root, which makes
/// it easier to import from /static, /public, or other directories.
#[proc_macro]
pub fn embed(item: TokenStream) -> TokenStream {
    let path = item.to_string().replace('\"', "");
    let resolved_path = std::fs::canonicalize(path).expect("Invalid path!");
    let contents = std::fs::read(&resolved_path).unwrap_or_else(|_| panic!("Could not read contents at {}", resolved_path.display()));
    let contents_string = String::from_utf8(contents).unwrap();
    quote! {
        #contents_string
    }.into()
}

/// Embed a file into the binary as a byte array.
/// This is useful for binary files that need to be represented
/// as a byte array.
///
/// This is similar to [`std::core::include_bytes`], but the path
/// is derived relative to the project root, which makes it easier
/// to import from /static, /public, or other directories.
#[proc_macro]
pub fn embed_binary(item: TokenStream) -> TokenStream {
    let path = item.to_string().replace('\"', "");
    let resolved_path = std::fs::canonicalize(path).expect("Invalid path!");
    let contents = std::fs::read(&resolved_path).unwrap_or_else(|_| panic!("Could not read contents at {}", resolved_path.display()));
    quote! {
        &[#(#contents),*]
    }.into()
}
