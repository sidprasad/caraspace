use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// A procedural macro that supports multiple spatial annotations on a single struct
/// 
/// Usage examples:
/// ```rust
/// #[spatial_annotations(
///     attribute(field = "id"),
///     orientation(field = "children", directions = ["above", "below"]),
///     atom_color(selector = "name", value = "blue")
/// )]
/// struct Node {
///     id: i32,
///     value: String,
/// }
/// ```

#[proc_macro_attribute]
pub fn spatial_annotations(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &input_struct.ident;
    
    // Parse the arguments - this is a simplified version
    let args_str = args.to_string();
    
    // For now, let's implement a simple version that handles one annotation type
    // In a more complete implementation, we'd parse multiple annotations
    let decorator_methods = parse_multiple_annotations(&args_str);
    
    let expanded = quote! {
        #input_struct

        impl rust_viz::spytial_annotations::HasSpytialDecorators for #struct_name {
            fn decorators() -> rust_viz::spytial_annotations::SpytialDecorators {
                let builder = rust_viz::spytial_annotations::SpytialDecoratorsBuilder::new();
                #(#decorator_methods)*
                builder.build()
            }
        }
    };

    TokenStream::from(expanded)
}

// Individual attribute macros for simpler usage
#[proc_macro_attribute]
pub fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("attribute", args, input)
}

#[proc_macro_attribute]
pub fn orientation(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("orientation", args, input)
}

#[proc_macro_attribute]
pub fn cyclic(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("cyclic", args, input)
}

#[proc_macro_attribute]
pub fn group(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("group", args, input)
}

#[proc_macro_attribute]
pub fn atom_color(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("atom_color", args, input)
}

#[proc_macro_attribute]
pub fn size(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("size", args, input)
}

#[proc_macro_attribute]
pub fn icon(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("icon", args, input)
}

#[proc_macro_attribute]
pub fn edge_color(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("edge_color", args, input)
}

#[proc_macro_attribute]
pub fn projection(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("projection", args, input)
}

#[proc_macro_attribute]
pub fn hide_field(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("hide_field", args, input)
}

#[proc_macro_attribute]
pub fn hide_atom(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("hide_atom", args, input)
}

#[proc_macro_attribute]
pub fn inferred_edge(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("inferred_edge", args, input)
}

#[proc_macro_attribute]
pub fn flag(args: TokenStream, input: TokenStream) -> TokenStream {
    spatial_annotation_impl("flag", args, input)
}

fn spatial_annotation_impl(annotation_type: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as ItemStruct);
    let struct_name = &input_struct.ident;
    
    // Parse the arguments
    let args_str = args.to_string();
    
    // Generate the HasSpytialDecorators implementation
    let decorator_method = match annotation_type {
        "attribute" => {
            let field = extract_string_arg(&args_str, "field").unwrap_or_else(|| "self".to_string());
            quote! {
                .attribute(#field, None)
            }
        },
        "orientation" => {
            let field = extract_string_arg(&args_str, "field").unwrap_or_else(|| "self".to_string());
            let directions = extract_array_arg(&args_str, "directions").unwrap_or_else(|| vec!["above".to_string()]);
            quote! {
                .orientation(#field, vec![#(#directions),*])
            }
        },
        "cyclic" => {
            let selector = extract_string_arg(&args_str, "selector").unwrap_or_else(|| "self".to_string());
            let direction = extract_string_arg(&args_str, "direction").unwrap_or_else(|| "clockwise".to_string());
            quote! {
                .cyclic(#selector, #direction)
            }
        },
        "group" => {
            let field = extract_string_arg(&args_str, "field").unwrap_or_else(|| "self".to_string());
            let group_on = extract_int_arg(&args_str, "group_on").unwrap_or(0);
            let add_to_group = extract_int_arg(&args_str, "add_to_group").unwrap_or(1);
            quote! {
                .group_field_based(#field, #group_on, #add_to_group, None)
            }
        },
        "atom_color" => {
            let selector = extract_string_arg(&args_str, "selector").unwrap_or_else(|| "self".to_string());
            let value = extract_string_arg(&args_str, "value").unwrap_or_else(|| "blue".to_string());
            quote! {
                .atom_color(#selector, #value)
            }
        },
        "size" => {
            let selector = extract_string_arg(&args_str, "selector").unwrap_or_else(|| "self".to_string());
            let width = extract_int_arg(&args_str, "width").unwrap_or(100) as u32;
            let height = extract_int_arg(&args_str, "height").unwrap_or(50) as u32;
            quote! {
                .size(#selector, #height, #width)
            }
        },
        "icon" => {
            let selector = extract_string_arg(&args_str, "selector").unwrap_or_else(|| "self".to_string());
            let value = extract_string_arg(&args_str, "value").unwrap_or_else(|| "default".to_string());
            quote! {
                .icon(#selector, #value)
            }
        },
        "edge_color" => {
            let selector = extract_string_arg(&args_str, "selector").unwrap_or_else(|| "self".to_string());
            let value = extract_string_arg(&args_str, "value").unwrap_or_else(|| "black".to_string());
            quote! {
                .edge_color(#selector, #value)
            }
        },
        "projection" => {
            let value = extract_string_arg(&args_str, "value").unwrap_or_else(|| "default".to_string());
            quote! {
                .projection(#value)
            }
        },
        "hide_field" => {
            let field = extract_string_arg(&args_str, "field").unwrap_or_else(|| "self".to_string());
            quote! {
                .hide_field(#field, None)
            }
        },
        "hide_atom" => {
            let selector = extract_string_arg(&args_str, "selector").unwrap_or_else(|| "self".to_string());
            quote! {
                .hide_atom(#selector)
            }
        },
        "inferred_edge" => {
            let selector = extract_string_arg(&args_str, "selector").unwrap_or_else(|| "self".to_string());
            let value = extract_string_arg(&args_str, "value").unwrap_or_else(|| "default".to_string());
            quote! {
                .inferred_edge(#selector, #value)
            }
        },
        "flag" => {
            let value = extract_string_arg(&args_str, "value").unwrap_or_else(|| "default".to_string());
            quote! {
                .flag(#value)
            }
        },
        _ => quote! {}
    };

    let expanded = quote! {
        #input_struct

        impl rust_viz::spytial_annotations::HasSpytialDecorators for #struct_name {
            fn decorators() -> rust_viz::spytial_annotations::SpytialDecorators {
                rust_viz::spytial_annotations::SpytialDecoratorsBuilder::new()
                    #decorator_method
                    .build()
            }
        }

        // Override the diagram function for this specific type
        impl #struct_name {
            /// Generate a diagram with spatial annotations for this type
            pub fn diagram(&self) where Self: serde::Serialize {
                let cnd_spec = rust_viz::spytial_annotations::to_yaml_for_instance(self).unwrap_or_default();
                rust_viz::diagram_impl(self, &cnd_spec);
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_multiple_annotations(_args: &str) -> Vec<proc_macro2::TokenStream> {
    // This is a placeholder for parsing multiple annotations
    // A complete implementation would properly parse the argument list
    vec![]
}

// Helper functions to extract arguments from the macro attribute
fn extract_string_arg(args: &str, key: &str) -> Option<String> {
    // Simple string extraction - in a real implementation you'd want proper parsing
    if let Some(start) = args.find(&format!("{} = \"", key)) {
        let start = start + key.len() + 4; // Skip 'key = "'
        if let Some(end) = args[start..].find("\"") {
            return Some(args[start..start + end].to_string());
        }
    }
    None
}

fn extract_int_arg(args: &str, key: &str) -> Option<i32> {
    if let Some(start) = args.find(&format!("{} = ", key)) {
        let start = start + key.len() + 3; // Skip 'key = '
        let end = args[start..].find(',').unwrap_or(args.len() - start);
        if let Ok(value) = args[start..start + end].trim().parse::<i32>() {
            return Some(value);
        }
    }
    None
}

fn extract_array_arg(args: &str, key: &str) -> Option<Vec<String>> {
    if let Some(start) = args.find(&format!("{} = [", key)) {
        let start = start + key.len() + 4; // Skip 'key = ['
        if let Some(end) = args[start..].find("]") {
            let array_content = &args[start..start + end];
            let items: Vec<String> = array_content
                .split(',')
                .map(|s| s.trim().trim_matches('"').to_string())
                .filter(|s| !s.is_empty())
                .collect();
            return Some(items);
        }
    }
    None
}