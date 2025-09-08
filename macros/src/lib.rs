use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Procedural macro to add spatial annotations to struct fields
/// 
/// Usage: #[attribute(field = "field_name")]
#[proc_macro_attribute]
pub fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Parse the args to extract field name
    let field_name = if args.is_empty() {
        "name".to_string()
    } else {
        parse_field_name_from_args(args)
    };
    
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate the HasCndDecorators implementation
    let expanded = quote! {
        #input

        impl #impl_generics json_data_instance_export::cnd_annotations::HasCndDecorators for #name #ty_generics #where_clause {
            fn decorators() -> json_data_instance_export::cnd_annotations::CndDecorators {
                json_data_instance_export::cnd_annotations::CndDecoratorsBuilder::new()
                    .attribute(#field_name, None)
                    .build()
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_field_name_from_args(args: TokenStream) -> String {
    // Simple parser for field = "value" pattern
    let args_str = args.to_string();
    if let Some(start) = args_str.find("field = \"") {
        let start = start + 9; // Length of "field = \""
        if let Some(end) = args_str[start..].find('"') {
            return args_str[start..start + end].to_string();
        }
    }
    "name".to_string() // default
}
