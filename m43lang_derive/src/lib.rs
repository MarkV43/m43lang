
#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data, Fields};

#[proc_macro_derive(AsCode)]
pub fn as_code_derive(input: TokenStream) -> TokenStream { 
    // Parse into a sintax tree
    let input = parse_macro_input!(input as DeriveInput);

    impl_as_code_macro(&input)
}

fn impl_as_code_macro(ast: &DeriveInput) -> TokenStream {
    if let Data::Enum(data) = ast.data.clone() {
        let fields = data.variants.iter();
        let name = &ast.ident;
        let branches = fields.clone().map(|var| {
            // Each one must be `<enum>::<branch> => format!("<enum>::<branch>")`
            let branch = &var.ident;
            match &var.fields {
                Fields::Unit => quote! {
                    #name::#branch => concat!(stringify!(#name), "::", stringify!(#branch)).to_string()
                },
                Fields::Unnamed(f) => {
                    let fs = f.unnamed.iter().enumerate().map(|(i, _)| {
                        format_ident!("v{}", i)
                    }).collect::<Vec<_>>();
                    let formats = vec!["{}"; f.unnamed.len()].join(", ");
                    quote! {
                        #name::#branch(#(#fs),*) => 
                            format!(
                                concat!(stringify!(#name), "::", stringify!(#branch), "(", #formats, ")"),
                                #(AsCode::as_code(#fs)),*
                            )
                    }
                },
                Fields::Named(_) => panic!("Named fields are not supported"),
            }
        }).collect::<Vec<_>>();
        let gen = quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl AsCode for #name {
                fn as_code(&self) -> String {
                    match self {
                        #(#branches),*
                    }
                }
            }
        };
        gen.into()
    } else {
        panic!("Not an enum");
    }
}

#[proc_macro_derive(Decodable)]
pub fn decodable_derive(input: TokenStream) -> TokenStream { 
    // Parse into a sintax tree
    let input = parse_macro_input!(input as DeriveInput);

    impl_decodable_macro(&input)
}

fn impl_decodable_macro(ast: &DeriveInput) -> TokenStream {
    if let Data::Enum(data) = ast.data.clone() {
        let fields = data.variants.iter();
        let name = &ast.ident;
        #[allow(unused_mut)]
        let mut branches = fields.clone().map(|var| {
            // Each one must be `<enum>::<branch> => format!("<enum>::<branch>")`
            let branch = &var.ident;
            match &var.fields {
                Fields::Unit => {
                    let text = branch.to_string();
                    let left = if text.len() > 1 {
                        let fl = text.chars().next().unwrap().to_string();
                        quote! {
                            #fl | #text
                        }
                    } else {
                        quote! {
                            #text
                        }
                    };
                    quote! {
                        #left => #name::#branch
                    }
                },
                Fields::Unnamed(f) => {
                    let text = branch.to_string();
                    let left = if text.len() > 1 {
                        let fl = text.chars().next().unwrap().to_string();
                        quote! {
                            #fl | #text
                        }
                    } else {
                        quote! {
                            #text
                        }
                    };
                    let fs = f.unnamed.iter().map(|x| {
                        let ty = &x.ty;
                        quote! {
                            #ty::decode(iter)
                        }
                        // ident!("{}::decode::<'a, _>{}", stringify!(x.ty), x.ident)
                    }).collect::<Vec<_>>();
                    quote! {
                        #left => #name::#branch(#(#fs),*)
                    }
                },
                Fields::Named(_) => panic!("Named fields are not supported"),
            }
        }).collect::<Vec<_>>();
        // let msg = concat!("Unknown ", stringify!(#name), ": {}");
        // branches.push(quote!{ _ => panic!(format!("Unkown {} : {}", stringify!(#name), text)) });
        branches.push(quote! { _ => panic!("Unkown {} : {}", stringify!(#name), text) });
        let gen = quote! {
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl Decodable for #name {
                fn decode<'a, I: Iterator<Item = &'a str>>(iter: &mut I) -> Self {
                    let text = iter.next().unwrap();
                    match Self::treat_inp(text) {
                        #(#branches),*
                    }
                }
            }
        };
        gen.into()
    } else {
        panic!("Not an enum");
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
