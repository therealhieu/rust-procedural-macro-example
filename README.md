# Procedural Macros Example
## Key concepts

The library [`proc_macro`](https://doc.rust-lang.org/proc_macro/) provided by the standard distribution, provides the types consumed in the interfaces of procedurally defined macro definitions such as function-like macros `#[proc_macro]`, macro attributes `#[proc_macro_attribute]` and custom derive attributes`#[proc_macro_derive]`.

`#[proc_macro]` is a procedural macro that creates **function-like macros**.*Function-like procedural macros* are procedural macros that are invoked using the macro invocation operator (`!`).These macros are defined by a [public](https://doc.rust-lang.org/reference/visibility-and-privacy.html)  [function](https://doc.rust-lang.org/reference/items/functions.html) with the `proc_macro`  [attribute](https://doc.rust-lang.org/reference/attributes.html) and a signature of `(TokenStream) -> TokenStream`.

`#[proc_macro_derive]` is a procedural macro that creates custom derive attributes. Derive macros can add additional [attribute](https://doc.rust-lang.org/reference/attributes.html) into the scope of the [item](https://doc.rust-lang.org/reference/items.html) they are on. They can be **used to generate new implementations for traits,** such as the `Debug` trait, or to generate new constructs.

`#[proc_macro_attribute]` is a procedural macro that creates macro attributes. Macro attributes are annotations that can be applied to items in Rust code, such as functions or structs. They can be used to modify the behavior of the annotated item, or to generate new constructs.

## Examples

**Code repository:** https://github.com/therealhieu/rust-procedural-macro-example

### `#[proc_macro]`

Suppose we want to define a macro that takes a struct definition like this:

```rust
struct Person {
    name: String,
    age: u32,
}
```

And generates a new struct definition with an additional `greet` method:

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn greet(&self) {
        println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
    }
}
```

To do this, we can define a function with the `#[proc_macro` attribute that takes a `TokenStream`
 as input and returns a `TokenStream`  as output. Here's what the macro definition might look like:

```rust
use proc_macro::TokenStream;
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
```

`TokenStream` is a type defined in the `proc_macro`crate that represents a sequence of tokens in Rust source code. A `TokenStream` can be thought of as a sequence of code tokens that can be manipulated programmatically.

syn is a parsing library for parsing a stream of Rust tokens into a syntax tree of Rust source code.

`[quote!](https://docs.rs/quote/latest/quote/macro.quote.html)` macro  is used to turn Rust syntax tree data structures into tokens of source code.

With this macro defined, we can use it like this:

```rust
use derive::add_greet;

add_greet!(
    struct Person {
        name: String,
        age: u32,
    }
);

fn main() {
    let hieu = Person {
        name: "Hieu".to_string(),
        age: 30,
    };

    hieu.greet(); # Hello, my name is Hieu and I am 30 years old.
}
```

Macro expand:

```rust
// Recursive expansion of add_greet! macro
// ========================================

struct Person {
    name: String,
    age: u32,
}
impl Person {
    fn greet(&self) {
        {
            $crate::io::_print($crate::fmt::Arguments::new_v1(
                &[],
                &[
                    $crate::fmt::ArgumentV1::new(&(self.name), $crate::fmt::Display::fmt),
                    $crate::fmt::ArgumentV1::new(&(self.age), $crate::fmt::Display::fmt),
                ],
            ));
        };
    }
}
```

### `#[proc_macro_attribute]`

In `derive/src/lib.rs`:

```rust
use darling::FromMeta;

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
```

`darling` is a tool for declarative attribute parsing in proc macro implementations. It simplifies the process of extracting attribute arguments and constructing `GreetArgs`. If not use `darling`, here are things we have to do:

```rust
#[derive(Debug, Clone)]
pub enum NestedMeta {
    Meta(syn::Meta),
    Lit(syn::Lit),
}

// https://docs.rs/syn/latest/syn/enum.Meta.html
// pub enum Meta {
//     Path(Path), # A meta path is like the test in #[test].
//     List(MetaList), # A meta list is like the derive(Copy) in #[derive(Copy)].
//     NameValue(MetaNameValue), # A name-value meta is like the path = "..." in #[path = "sys/windows.rs"].
// }

impl NestedMeta {
	  // 
    pub fn parse_meta_list(tokens: TokenStream) -> syn::Result<Vec<Self>> {
        syn::punctuated::Punctuated::<NestedMeta, Token![,]>::parse_terminated
            .parse2(tokens)
            .map(|punctuated| punctuated.into_iter().collect())
    }
}
```

The value of `attr_args` (`Vec<NestedMetadata>`) looks like this:

```rust
attr_args: [
    Meta(
        Meta::NameValue {
            path: Path {
                leading_colon: None,
                segments: [
                    PathSegment {
                        ident: Ident {
                            ident: "content",
                            span: #0 bytes(658..665),
                        },
                        arguments: PathArguments::None,
                    },
                ],
            },
            eq_token: Eq,
            value: Expr::Lit {
                attrs: [],
                lit: Lit::Str {
                    token: "Hello, my name is {name}  and I a {age} years old.",
                },
            },
        },
    ),
]
```

Now we have a list of `NestedMetadata`, all we have to do is iterating through this list  and construct GreetArgs. To check if the ident is `"content"`, we can call `meta.path.is_ident("content")` then get the meta value from `meta.value`.

In `app/src/main.rs`:

```rust
use derive::greet;

#[greet(content = "Hello, my name is {name}  and I a {age} years old.")]
struct Person {
    name: String,
    age: u32,
}

pub fn main() {
    let hieu = Person {
        name: "Hieu".to_string(),
        age: 30,
    };

    hieu.greet();
}
```

Macro expand:

```rust
// Recursive expansion of greet macro
// ===================================

struct Person {
    name: String,
    age: u32,
}
impl Person {
    fn greet(&self) {
        println!(
            "Hello, my name is {name}  and I a {age} years old.",
            name = self.name,
            age = self.age
        );
    }
}
```

### `#[proc_macro_derive]`

In `derive/src/lib.rs`:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Greet)]
pub fn greet_derive(input: TokenStream) -> TokenStream {
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
        impl #struct_name {
            fn greet(&self) {
                println!("Hello, my name is {} and I am {} years old.", self.name, self.age);
            }
        }
    };

    // Return the generated code as a TokenStream
    TokenStream::from(expanded)
}
```

Note that we no longer need to generate a new struct definition, since we are implementing the `greet`method for the existing struct. Also, since we are using a derive macro, the macro name is now `Greet` instead of `add_greet`. The rest of the macro definition is similar to the previous example.

In `app/src/main.rs`: 

```rust
use derive::Greet;

#[derive(Greet)]
struct PerSon {
    name: String,
    age: u32,
}

fn main() {
    let hieu = PerSon {
        name: "Hieu".to_string(),
        age: 30,
    };

    hieu.greet();
}
```

Macro expand:

```rust
// Recursive expansion of Greet macro
// ===================================

impl PerSon {
    fn greet(&self) {
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}
```

### `#[proc_macro_derive]` with attributes

Once again, when working with attribute arguments, `darling` helps us a lot. Look at the example below:

```rust
use darling::FromDeriveInput;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(greet2))]
struct Greet2Args {
    content: String,
}

#[proc_macro_derive(Greet2, attributes(greet2))]
pub fn greet2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let greet2_args = Greet2Args::from_derive_input(&input).expect("Failed to parse");

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
```

The value of `input` is:

```rust
DeriveInput {
    attrs: [
        Attribute {
            pound_token: Pound,
            style: AttrStyle::Outer,
            bracket_token: Bracket,
            meta: Meta::List {
                path: Path {
                    leading_colon: None,
                    segments: [
                        PathSegment {
                            ident: Ident {
                                ident: "greet2",
                                span: #0 bytes(1035..1041),
                            },
                            arguments: PathArguments::None,
                        },
                    ],
                },
                delimiter: MacroDelimiter::Paren(
                    Paren,
                ),
                tokens: TokenStream [
                    Ident {
                        ident: "content",
                        span: #0 bytes(1042..1049),
                    },
                    Punct {
                        ch: '=',
                        spacing: Alone,
                        span: #0 bytes(1050..1051),
                    },
                    Literal {
                        kind: Str,
                        symbol: "Hello, my name is {name}  and I a {age} years old.",
                        suffix: None,
                        span: #0 bytes(1052..1104),
                    },
                ],
            },
        },
    ],
    vis: Visibility::Inherited,
    ident: Ident {
        ident: "Person",
        span: #0 bytes(1118..1124),
    },
    generics: Generics {...},
    data: Data::Struct {...},
}
```

In this example, we focus on `attrs`,  therefore it is unnecessary to care about other fields.

As you can see,  attributes are already parsed as `[Attribute](https://docs.rs/syn/latest/syn/struct.Attribute.html),` . The algorithm to construct `Greet2Args` is iterating over `Attributes` → check the `meta.path` ident, get the key-value argument from `meta.tokens` `TokenStream` as in [`#[proc_macro_attribute]` ](https://www.notion.so/proc_macro_attribute-98855aa1b425421b8f6eb6938714e0bd).  `darling`

handles this for us.

In `app/src/main.rs`:

```rust
use derive::Greet2;

#[derive(Greet2)]
#[greet2(content = "Hello, my name is {name}  and I a {age} years old.")]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let hieu = Person {
        name: "Hieu".to_string(),
        age: 30,
    };

    hieu.greet();
}
```

Macro expand:

```rust
// Recursive expansion of Greet2 macro
// ====================================

impl Person {
    fn greet(&self) {
        println!(
            "Hello, my name is {name}  and I a {age} years old.",
            name = self.name,
            age = self.age
        );
    }
}
```

## References

[GitHub - dtolnay/proc-macro-workshop: Learn to write Rust procedural macros  [Rust Latam conference, Montevideo Uruguay, March 2019]](https://github.com/dtolnay/proc-macro-workshop#derive-macro-derivebuilder)

[https://github.com/imbolc/rust-derive-macro-guide](https://github.com/imbolc/rust-derive-macro-guide)

[Procedural macros in Rust - LogRocket Blog](https://blog.logrocket.com/procedural-macros-in-rust/)