use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Expr, Fields, parse_macro_input};

// Proedural Macros
// Procedural Macros: These are more powerful and complex. They operate on a TokenStream and allow you to inspect, modify, and generate arbitrary Rust code. Procedural macros come in three flavors:
// Function-like macros: #[proc_macro]
// Derive macros: #[proc_macro_derive]
// Attribute macros: #[proc_macro_attribute]

#[proc_macro]
pub fn reverse_exprs(input: TokenStream) -> TokenStream {
    let exprs = parse_macro_input!(input with syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated);
    let mut expr_vec: Vec<_> = exprs.into_iter().collect();
    expr_vec.reverse();
    let output = quote! {(#(#expr_vec),*)};
    output.into()
}

#[proc_macro_derive(MyDebug)]
pub fn my_debug_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;

    let code = match ast.data {
        Data::Struct(data_struct) => {
            let fields = match data_struct.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|f| {
                        let field_name = &f.ident;
                        quote! {
                            print!("{}: {:?}, ", stringify!(#field_name), self.#field_name);
                        }
                    })
                    .collect::<Vec<_>>(),
                Fields::Unnamed(fields) => fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _f)| {
                        let index = syn::Index::from(i);
                        quote! {
                            print!("{}: {:?}, ", #i, self.#index);
                        }
                    })
                    .collect::<Vec<_>>(),
                Fields::Unit => {
                    vec![quote! { print!("(unit struct)"); }]
                }
            };
            quote! {
                impl MyDebug for #name {
                    fn my_fmt(&self) {
                        print!("{}: {{ ", stringify!(#name));
                        #(#fields)*
                        println!("}}");
                    }
                }
            }
        }
        _ => {
            panic!("MyDebug can only be derived for structs");
        }
    };
    code.into()
}

#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;
    let fn_sig = &input_fn.sig;
    let fn_vis = &input_fn.vis;
    let fn_attrs = &input_fn.attrs;

    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_sig {
            println!("Calling function `{}`", stringify!(#fn_name));
            let result = #fn_block;
            println!("Function `{}` returned", stringify!(#fn_name));
            result
        }
    };

    expanded.into()
}
