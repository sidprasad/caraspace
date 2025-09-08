use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, Meta, NestedMeta, Lit};

/// Procedural macro to add spatial annotations to struct fields
/// 
/// Usage: #[attribute(field = "field_name")]
#[proc_macro_attribute]
pub fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as DeriveInput);
    
    // Extract field name from args
    let field_name = extract_field_name(&args);
    
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate the HasCndDecorators implementation
    let expanded = quote! {
        #input

        impl #impl_generics crate::cnd_annotations::HasCndDecorators for #name #ty_generics #where_clause {
            fn get_cnd_decorators(&self) -> crate::cnd_annotations::CndDecorators {
                crate::cnd_annotations::CndDecoratorsBuilder::new()
                    .attribute(#field_name, None)
                    .build()
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            /// Generate a diagram of this data structure
            pub fn diagram(&self) 
            where 
                Self: serde::Serialize + crate::cnd_annotations::HasCndDecorators
            {
                crate::diagram_with_annotations(self);
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_field_name(args: &AttributeArgs) -> String {
    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(name_value)) = arg {
            if name_value.path.is_ident("field") {
                if let Lit::Str(lit_str) = &name_value.lit {
                    return lit_str.value();
                }
            }
        }
    }
    "name".to_string() // default field name
}
