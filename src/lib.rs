use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Store)]
pub fn store(input: TokenStream) -> TokenStream {
	// Parse the token stream
	let input = parse_macro_input!(input as DeriveInput);
	// Fetch the struct name
	let name = &input.ident;
	// Generate the output
	let output = quote! {

		impl From<#name> for Vec<u8> {
			fn from(v: #name) -> Vec<u8> {
				msgpack::to_vec(&v).unwrap()
			}
		}

		impl From<&#name> for Vec<u8> {
			fn from(v: &#name) -> Vec<u8> {
				msgpack::to_vec(&v).unwrap()
			}
		}

		impl From<Vec<u8>> for #name {
			fn from(v: Vec<u8>) -> Self {
				msgpack::from_slice::<Self>(&v).unwrap()
			}
		}

	};

	output.into()
}
