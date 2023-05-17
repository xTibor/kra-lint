use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(NewTypeIter)]
pub fn newtype_iter_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_newtype_iter(&ast)
}

fn impl_newtype_iter(ast: &syn::DeriveInput) -> TokenStream {
    // pub struct ContainerType(
    //     pub Vec<FieldType>,
    // );

    let container_type = &ast.ident;

    let field_type = if let syn::Data::Struct(s) = &ast.data {
        let inner_type = &s.fields.iter().next().unwrap().ty;
        match inner_type {
            syn::Type::Path(ref p) => match p.path.segments.first().unwrap().arguments {
                syn::PathArguments::AngleBracketed(ref path_arg) => path_arg.args.first().unwrap(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    } else {
        panic!("NewTypeIter derive on a non-tuple struct type")
    };

    let gen = quote! {
        impl #container_type {
            pub fn iter(&self) -> impl Iterator<Item = &#field_type> {
                self.0.iter()
            }
        }
    };
    gen.into()
}
