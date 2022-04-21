use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn add_trait_bounds(mut generics: syn::Generics) -> syn::Generics {
    for param in &mut generics.params {
        if let syn::GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(syn::parse_quote!(TypeName));
        }
    }
    generics
}

use darling::FromDeriveInput;

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(typename))]
struct TypeNameArguments {
    #[darling(default)]
    type_name: Option<syn::Ident>,
    #[darling(default)]
    crate_name: Option<syn::Ident>,
    #[darling(default)]
    crate_module: Option<String>,
    #[darling(default)]
    crate_version: Option<String>,
    #[darling(default)]
    rustc_version: Option<String>,
    #[darling(default)]
    default_to_none: bool,
}

#[proc_macro_derive(TypeName, attributes(typename))]
pub fn derive_type_name(tokens: TokenStream) -> TokenStream {
    let derived = parse_macro_input!(tokens);
    let TypeNameArguments {
        type_name,
        crate_name,
        crate_module,
        crate_version,
        rustc_version,
        default_to_none,
    } = TypeNameArguments::from_derive_input(&derived).unwrap();
    let DeriveInput {
        ident,
        attrs: _,
        vis: _,
        generics,
        data: _,
    } = derived;
    let generics = add_trait_bounds(generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let type_name = type_name
        .map(|x| x.to_string())
        .unwrap_or_else(|| ident.to_string());
    let crate_name = if let Some(crate_name) = crate_name {
        quote!(Some(stringify!(#crate_name).to_owned()))
    } else if default_to_none {
        quote!(None)
    } else {
        quote!(Some(env!("CARGO_PKG_NAME").to_owned()))
    };
    let module_path_import;
    let crate_module = if let Some(crate_module) = crate_module {
        module_path_import = quote! {};
        quote!(Some(#crate_module.to_owned()))
    } else if default_to_none {
        module_path_import = quote! {};
        quote!(None)
    } else {
        module_path_import = quote! {use ::std::module_path as std_module_path_module_path;};
        quote!(Some(std_module_path_module_path!().to_owned()))
    };
    let crate_version = if let Some(crate_version) = crate_version {
        quote!(Some(
                <::typenaming::TypeNameSemverVersion as ::std::str::FromStr>::from_str(
                #crate_version
            )
            .expect(&format!("Failed to parse crate version argument '{}'", #crate_version)))
        )
    } else if default_to_none {
        quote!(None)
    } else {
        quote!(Some(::typenaming::new_semver_version(
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
            env!("CARGO_PKG_VERSION_PRE")
        )))
    };
    let rustc_version = if let Some(rustc_version) = rustc_version {
        quote!(Some(
            <::typenaming::TypeNameSemverVersion as ::std::str::FromStr>::from_str(
                #rustc_version
            )
            .expect(&format!("Failed to parse rustc version argument '{}'", #rustc_version))
        ))
    } else if default_to_none {
        quote!(None)
    } else {
        quote!(Some(
            ::typenaming::rustc_version().expect("Failed to fetch rustc version")
        ))
    };
    let generics = generics
        .type_params()
        .map(|x| x.ident.clone())
        .collect::<Vec<_>>();
    let generics = quote!(#(<#generics as ::typenaming::TypeName>::type_name_static()),*);
    let body = quote! {
        #module_path_import
        ::typenaming::TypeNameData::new(
            #type_name.to_owned(),
            #crate_name,
            #crate_module,
            #crate_version,
            #rustc_version,
            vec![
              #generics
            ]
        )
    };
    quote! {
        #[automatically_derived]
        impl #impl_generics TypeName for #ident #ty_generics #where_clause {
            fn type_name(&self) -> ::typenaming::TypeNameData {
                #body
            }
            fn type_name_static() -> ::typenaming::TypeNameData where Self: Sized {
                #body
            }
        }
    }
    .into()
}
