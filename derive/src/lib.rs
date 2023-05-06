use proc_macro::TokenStream;

use darling::{export::NestedMeta, FromDeriveInput, FromMeta};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro]
pub fn add_greet(input: TokenStream) -> TokenStream {
    // Parse the input struct definition
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name and fields
    let struct_name = input.ident;
    let fields = match input.data {
        syn::Data::Struct(ref s) => s.fields.iter().collect::<Vec<_>>(),
        _ => panic!("Greet can only be derived for structs"),
    };

    // Generate the new struct definition with the greet method
    let expanded = quote! {
        struct #struct_name {
            #(#fields),*
        }

        impl #struct_name {
            fn greet(&self) {
                println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
            }
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}

#[proc_macro_derive(Greet)]
pub fn greet_derive(input: TokenStream) -> TokenStream {
    // Parse the input struct definition
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the struct name and fields
    let struct_name = input.ident;

    // Generate the new struct definition with the greet method
    let expanded = quote! {
        impl #struct_name {
            fn greet(&self) {
                println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
            }
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}

#[derive(Debug, FromMeta)]
struct GreetArgs {
    content: String,
}

/// Example #[greet(content = "Hello, my name is {self.name} and I am {self.age} years old.")]
#[proc_macro_attribute]
pub fn greet(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let greet_args = match GreetArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident.clone();
    let content = greet_args.content;

    let expanded = quote! {
        #input

        impl #struct_name {
            fn greet(&self) {
                println!(#content, name = self.name, age = self.age);
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(greet2))]
struct Greet2Args {
    content: String,
}

#[proc_macro_derive(Greet2, attributes(greet2))]
pub fn greet2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    eprintln!("{:#?}", input);
    let greet_attr_args = Greet2Args::from_derive_input(&input).unwrap();

    let struct_name = input.ident;
    let content = greet_attr_args.content;

    let expanded = quote! {
        impl #struct_name {
            fn greet(&self) {
                println!(#content, name = self.name, age = self.age);
            }
        }
    };

    TokenStream::from(expanded)
}
