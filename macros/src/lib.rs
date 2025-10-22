use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Attribute, Data, Fields, Type, PathArguments, GenericArgument};

/// Collect decorators from field types at compile time
/// This walks the type tree and generates calls to collect decorators from nested types
fn collect_field_type_decorators(data: &Data, self_type_name: &str) -> Vec<proc_macro2::TokenStream> {
    let mut field_decorators = Vec::new();
    let mut seen_types = std::collections::HashSet::new();
    
    // Add the self type to seen_types to prevent self-referential includes
    seen_types.insert(self_type_name.to_string());
    
    if let Data::Struct(data_struct) = data {
        match &data_struct.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    field_decorators.extend(analyze_field_type(&field.ty, &mut seen_types));
                }
            }
            Fields::Unnamed(fields) => {
                for field in &fields.unnamed {
                    field_decorators.extend(analyze_field_type(&field.ty, &mut seen_types));
                }
            }
            Fields::Unit => {}
        }
    }
    
    field_decorators
}

/// Analyze a field type and extract decorator calls for nested types
fn analyze_field_type(ty: &Type, seen_types: &mut std::collections::HashSet<String>) -> Vec<proc_macro2::TokenStream> {
    match ty {
        // Handle Vec<T>, Option<T>, Box<T>, etc.
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                match segment.ident.to_string().as_str() {
                    "Vec" | "Option" | "Box" => {
                        // Extract the inner type T from Vec<T>, Option<T>, etc.
                        if let PathArguments::AngleBracketed(args) = &segment.arguments {
                            if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                                return analyze_inner_type(inner_type, seen_types);
                            }
                        }
                    }
                    type_name => {
                        // Check if this might be a type with decorators
                        if is_likely_decorated_type(type_name) && !seen_types.contains(type_name) {
                            seen_types.insert(type_name.to_string());
                            return vec![generate_decorator_call_for_type(type_name)];
                        }
                    }
                }
            }
        }
        _ => {}
    }
    
    Vec::new()
}

/// Analyze inner types (like T in Vec<T>)
/// This recursively handles nested generics like Option<Box<T>>
fn analyze_inner_type(ty: &Type, seen_types: &mut std::collections::HashSet<String>) -> Vec<proc_macro2::TokenStream> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            let type_name = segment.ident.to_string();
            
            // Check if this is another wrapper type (Box, Option, etc.)
            match type_name.as_str() {
                "Vec" | "Option" | "Box" => {
                    // Recursively unwrap nested wrappers like Box<Node> in Option<Box<Node>>
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                            return analyze_inner_type(inner_type, seen_types);
                        }
                    }
                }
                _ => {
                    // This is the actual type (like Node)
                    if is_likely_decorated_type(&type_name) && !seen_types.contains(&type_name) {
                        seen_types.insert(type_name.clone());
                        return vec![generate_decorator_call_for_type(&type_name)];
                    }
                }
            }
        }
    }
    Vec::new()
}

/// Check if a type name is likely to have CnD decorators
/// This is a heuristic - we look for non-primitive types
fn is_likely_decorated_type(type_name: &str) -> bool {
    match type_name {
        // Skip primitive types
        "i8" | "i16" | "i32" | "i64" | "i128" |
        "u8" | "u16" | "u32" | "u64" | "u128" |
        "f32" | "f64" | "bool" | "char" |
        "String" | "str" => false,
        
        // Skip common std types and collections
        "Vec" | "Option" | "Result" | "Box" | "HashMap" | "HashSet" => false,
        
        // Everything else might have decorators
        _ => {
            // Check if it starts with uppercase (typical struct naming)
            type_name.chars().next().map_or(false, |c| c.is_uppercase())
        }
    }
}

/// Generate a call to collect decorators from a specific type
fn generate_decorator_call_for_type(type_name: &str) -> proc_macro2::TokenStream {
    let type_ident = syn::Ident::new(type_name, proc_macro2::Span::call_site());
    
    // Generate a call that will include decorators from this type
    quote! {
        .include_decorators_from_type::<#type_ident>()
    }
}

/// Derive macro for implementing HasCndDecorators trait
/// 
/// This macro analyzes all spatial annotation attributes on a struct
/// and generates a single implementation of HasCndDecorators that includes
/// all the annotations.
/// 
/// # Supported Attributes
/// - `#[attribute(field = "field_name")]` - Adds attribute directive
/// - `#[flag(name = "flag_name")]` - Adds flag directive  
/// - `#[orientation(selector = "sel", directions = ["up", "down"])]` - Adds orientation constraint
/// - `#[align(selector = "sel", direction = "up")]` - Adds align constraint
/// - `#[cyclic(selector = "sel", direction = "up")]` - Adds cyclic constraint
/// - `#[group(selector = "sel", name = "group_name")]` - Adds selector-based group constraint
/// - `#[group(field = "field", group_on = 1, add_to_group = 2)]` - Adds field-based group constraint
/// - `#[atom_color(selector = "sel", value = "red")]` - Adds atom color directive
/// - `#[size(selector = "sel", height = 20, width = 30)]` - Adds size directive
/// - `#[icon(selector = "sel", path = "icon.png", show_labels = true)]` - Adds icon directive
/// - `#[edge_color(field = "field", value = "blue")]` - Adds edge color directive
/// - `#[projection(sig = "signature")]` - Adds projection directive
/// - `#[hide_field(field = "field")]` - Adds hide field directive
/// - `#[hide_atom(selector = "sel")]` - Adds hide atom directive
/// - `#[inferred_edge(name = "edge", selector = "sel")]` - Adds inferred edge directive
/// 
/// # Example
/// ```rust
/// use serde::Serialize;
/// use json_data_instance_export::CndDecorators;
/// 
/// #[derive(Serialize, CndDecorators)]
/// #[attribute(field = "name")]
/// #[flag(name = "important")]
/// struct Person {
///     name: String,
///     age: u32,
/// }
/// ```
#[proc_macro_derive(CndDecorators, attributes(attribute, flag, orientation, align, cyclic, group, atom_color, size, icon, edge_color, projection, hide_field, hide_atom, inferred_edge))]
pub fn derive_cnd_decorators(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Parse spatial annotation attributes for this type
    let mut decorator_calls = Vec::new();
    
    for attr in &input.attrs {
        match parse_spatial_attribute(attr) {
            Some(SpatialAttribute::Attribute { field }) => {
                decorator_calls.push(quote! {
                    .attribute(#field, None)
                });
            }
            Some(SpatialAttribute::Flag { name }) => {
                decorator_calls.push(quote! {
                    .flag(#name)
                });
            }
            Some(SpatialAttribute::Orientation { selector, directions }) => {
                decorator_calls.push(quote! {
                    .orientation(#selector, vec![#(#directions),*])
                });
            }
            Some(SpatialAttribute::Align { selector, direction }) => {
                decorator_calls.push(quote! {
                    .align(#selector, #direction)
                });
            }
            Some(SpatialAttribute::Cyclic { selector, direction }) => {
                decorator_calls.push(quote! {
                    .cyclic(#selector, #direction)
                });
            }
            Some(SpatialAttribute::GroupSelector { selector, name }) => {
                decorator_calls.push(quote! {
                    .group_selector_based(#selector, #name)
                });
            }
            Some(SpatialAttribute::GroupField { field, group_on, add_to_group }) => {
                decorator_calls.push(quote! {
                    .group_field_based(#field, #group_on, #add_to_group, None)
                });
            }
            Some(SpatialAttribute::AtomColor { selector, value }) => {
                decorator_calls.push(quote! {
                    .atom_color(#selector, #value)
                });
            }
            Some(SpatialAttribute::Size { selector, height, width }) => {
                decorator_calls.push(quote! {
                    .size(#selector, #height, #width)
                });
            }
            Some(SpatialAttribute::Icon { selector, path, show_labels }) => {
                decorator_calls.push(quote! {
                    .icon(#selector, #path, #show_labels)
                });
            }
            Some(SpatialAttribute::EdgeColor { field, value, selector }) => {
                let selector_arg = match selector {
                    Some(s) => quote! { Some(#s) },
                    None => quote! { None },
                };
                decorator_calls.push(quote! {
                    .edge_color(#field, #value, #selector_arg)
                });
            }
            Some(SpatialAttribute::Projection { sig }) => {
                decorator_calls.push(quote! {
                    .projection(#sig)
                });
            }
            Some(SpatialAttribute::HideField { field, selector }) => {
                let selector_arg = match selector {
                    Some(s) => quote! { Some(#s) },
                    None => quote! { None },
                };
                decorator_calls.push(quote! {
                    .hide_field(#field, #selector_arg)
                });
            }
            Some(SpatialAttribute::HideAtom { selector }) => {
                decorator_calls.push(quote! {
                    .hide_atom(#selector)
                });
            }
            Some(SpatialAttribute::InferredEdge { name, selector }) => {
                decorator_calls.push(quote! {
                    .inferred_edge(#name, #selector)
                });
            }
            None => {}
        }
    }
    
    // Second, analyze field types and collect their decorators at compile time
    // Only do this for structs - enums don't have fields to analyze
    let field_type_decorators = match &input.data {
        Data::Struct(_) => collect_field_type_decorators(&input.data, &name.to_string()),
        Data::Enum(_) | Data::Union(_) => Vec::new(), // Enums and unions just return empty decorators
    };
    
    // Combine own decorators with field type decorators
    decorator_calls.extend(field_type_decorators);

    // Generate the HasCndDecorators implementation
    let expanded = quote! {
        impl #impl_generics json_data_instance_export::cnd_annotations::HasCndDecorators for #name #ty_generics #where_clause {
            fn decorators() -> json_data_instance_export::cnd_annotations::CndDecorators {
                // Register this type automatically when decorators() is called
                static REGISTRATION: ::std::sync::Once = ::std::sync::Once::new();
                REGISTRATION.call_once(|| {
                    let decorators = json_data_instance_export::cnd_annotations::CndDecoratorsBuilder::new()
                        #(#decorator_calls)*
                        .build();
                    json_data_instance_export::cnd_annotations::register_type_decorators(
                        stringify!(#name), 
                        decorators.clone()
                    );
                });

                json_data_instance_export::cnd_annotations::CndDecoratorsBuilder::new()
                    #(#decorator_calls)*
                    .build()
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug)]
enum SpatialAttribute {
    Attribute { field: String },
    Flag { name: String },
    Orientation { selector: String, directions: Vec<String> },
    Align { selector: String, direction: String },
    Cyclic { selector: String, direction: String },
    GroupSelector { selector: String, name: String },
    GroupField { field: String, group_on: u32, add_to_group: u32 },
    AtomColor { selector: String, value: String },
    Size { selector: String, height: u32, width: u32 },
    Icon { selector: String, path: String, show_labels: bool },
    EdgeColor { field: String, value: String, selector: Option<String> },
    Projection { sig: String },
    HideField { field: String, selector: Option<String> },
    HideAtom { selector: String },
    InferredEdge { name: String, selector: String },
}

fn parse_spatial_attribute(attr: &Attribute) -> Option<SpatialAttribute> {
    let path = &attr.path();
    
    if path.is_ident("attribute") {
        parse_attribute_args(attr)
    } else if path.is_ident("flag") {
        parse_flag_args(attr)
    } else if path.is_ident("orientation") {
        parse_orientation_args(attr)
    } else if path.is_ident("align") {
        parse_align_args(attr)
    } else if path.is_ident("cyclic") {
        parse_cyclic_args(attr)
    } else if path.is_ident("group") {
        parse_group_args(attr)
    } else if path.is_ident("atom_color") {
        parse_atom_color_args(attr)
    } else if path.is_ident("size") {
        parse_size_args(attr)
    } else if path.is_ident("icon") {
        parse_icon_args(attr)
    } else if path.is_ident("edge_color") {
        parse_edge_color_args(attr)
    } else if path.is_ident("projection") {
        parse_projection_args(attr)
    } else if path.is_ident("hide_field") {
        parse_hide_field_args(attr)
    } else if path.is_ident("hide_atom") {
        parse_hide_atom_args(attr)
    } else if path.is_ident("inferred_edge") {
        parse_inferred_edge_args(attr)
    } else {
        None
    }
}

fn parse_attribute_args(attr: &Attribute) -> Option<SpatialAttribute> {
    // Simple parsing - look for field = "value"
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        if let Some(field) = extract_string_from_tokens(&token_str, "field") {
            return Some(SpatialAttribute::Attribute { field });
        }
    }
    
    Some(SpatialAttribute::Attribute { field: "name".to_string() })
}

fn parse_flag_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        if let Some(name) = extract_string_from_tokens(&token_str, "name") {
            return Some(SpatialAttribute::Flag { name });
        }
    }
    
    Some(SpatialAttribute::Flag { name: "important".to_string() })
}

fn parse_orientation_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let selector = extract_string_from_tokens(&token_str, "selector").unwrap_or_else(|| "".to_string());
        let directions = extract_array_from_tokens(&token_str, "directions")
            .unwrap_or_else(|| vec!["up".to_string(), "down".to_string()]);
        
        return Some(SpatialAttribute::Orientation { selector, directions });
    }
    
    None
}

fn parse_group_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        if token_str.contains("field =") {
            // Field-based grouping
            let field = extract_string_from_tokens(&token_str, "field").unwrap_or_else(|| "id".to_string());
            let group_on = extract_number_from_tokens(&token_str, "group_on").unwrap_or(1);
            let add_to_group = extract_number_from_tokens(&token_str, "add_to_group").unwrap_or(2);
            
            Some(SpatialAttribute::GroupField { field, group_on, add_to_group })
        } else {
            // Selector-based grouping
            let selector = extract_string_from_tokens(&token_str, "selector").unwrap_or_else(|| "".to_string());
            let name = extract_string_from_tokens(&token_str, "name").unwrap_or_else(|| "default".to_string());
            
            Some(SpatialAttribute::GroupSelector { selector, name })
        }
    } else {
        None
    }
}

fn parse_align_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let selector = extract_string_from_tokens(&token_str, "selector").unwrap_or_else(|| "".to_string());
        let direction = extract_string_from_tokens(&token_str, "direction").unwrap_or_else(|| "horizontal".to_string());
        
        Some(SpatialAttribute::Align { selector, direction })
    } else {
        None
    }
}

fn parse_cyclic_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let selector = extract_string_from_tokens(&token_str, "selector").unwrap_or_else(|| "".to_string());
        let direction = extract_string_from_tokens(&token_str, "direction").unwrap_or_else(|| "up".to_string());
        
        Some(SpatialAttribute::Cyclic { selector, direction })
    } else {
        None
    }
}

fn parse_atom_color_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let selector = extract_string_from_tokens(&token_str, "selector").unwrap_or_else(|| "".to_string());
        let value = extract_string_from_tokens(&token_str, "value").unwrap_or_else(|| "blue".to_string());
        
        Some(SpatialAttribute::AtomColor { selector, value })
    } else {
        None
    }
}

fn parse_size_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let selector = extract_string_from_tokens(&token_str, "selector").unwrap_or_else(|| "".to_string());
        let height = extract_number_from_tokens(&token_str, "height").unwrap_or(20);
        let width = extract_number_from_tokens(&token_str, "width").unwrap_or(30);
        
        Some(SpatialAttribute::Size { selector, height, width })
    } else {
        None
    }
}

fn parse_icon_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let selector = extract_string_from_tokens(&token_str, "selector").unwrap_or_else(|| "".to_string());
        let path = extract_string_from_tokens(&token_str, "path").unwrap_or_else(|| "icon.png".to_string());
        let show_labels = extract_bool_from_tokens(&token_str, "show_labels").unwrap_or(true);
        
        Some(SpatialAttribute::Icon { selector, path, show_labels })
    } else {
        None
    }
}

fn parse_edge_color_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let field = extract_string_from_tokens(&token_str, "field").unwrap_or_else(|| "relation".to_string());
        let value = extract_string_from_tokens(&token_str, "value").unwrap_or_else(|| "blue".to_string());
        let selector = extract_string_from_tokens(&token_str, "selector");
        
        Some(SpatialAttribute::EdgeColor { field, value, selector })
    } else {
        None
    }
}

fn parse_projection_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let sig = extract_string_from_tokens(&token_str, "sig").unwrap_or_else(|| "default".to_string());
        
        Some(SpatialAttribute::Projection { sig })
    } else {
        None
    }
}

fn parse_hide_field_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let field = extract_string_from_tokens(&token_str, "field").unwrap_or_else(|| "field".to_string());
        let selector = extract_string_from_tokens(&token_str, "selector");
        
        Some(SpatialAttribute::HideField { field, selector })
    } else {
        None
    }
}

fn parse_hide_atom_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let selector = extract_string_from_tokens(&token_str, "selector").unwrap_or_else(|| "".to_string());
        
        Some(SpatialAttribute::HideAtom { selector })
    } else {
        None
    }
}

fn parse_inferred_edge_args(attr: &Attribute) -> Option<SpatialAttribute> {
    if let Ok(meta) = attr.meta.require_list() {
        let tokens = &meta.tokens;
        let token_str = tokens.to_string();
        
        let name = extract_string_from_tokens(&token_str, "name").unwrap_or_else(|| "edge".to_string());
        let selector = extract_string_from_tokens(&token_str, "selector").unwrap_or_else(|| "".to_string());
        
        Some(SpatialAttribute::InferredEdge { name, selector })
    } else {
        None
    }
}

fn extract_string_from_tokens(tokens: &str, key: &str) -> Option<String> {
    // Try both with and without spaces around =
    let patterns = [
        format!("{} = \"", key),
        format!("{}=\"", key),
        format!("{} =\"", key),
        format!("{}= \"", key),
    ];
    
    for pattern in &patterns {
        if let Some(start) = tokens.find(pattern) {
            let start = start + pattern.len();
            if let Some(end) = tokens[start..].find('"') {
                return Some(tokens[start..start + end].to_string());
            }
        }
    }
    None
}

fn extract_number_from_tokens(tokens: &str, key: &str) -> Option<u32> {
    let pattern = format!("{} = ", key);
    if let Some(start) = tokens.find(&pattern) {
        let start = start + pattern.len();
        let rest = &tokens[start..];
        let end = rest.find([',', ' ', ')']).unwrap_or(rest.len());
        if let Ok(value) = rest[..end].trim().parse::<u32>() {
            return Some(value);
        }
    }
    None
}

fn extract_bool_from_tokens(tokens: &str, key: &str) -> Option<bool> {
    let pattern = format!("{} = ", key);
    if let Some(start) = tokens.find(&pattern) {
        let start = start + pattern.len();
        let rest = &tokens[start..];
        let end = rest.find([',', ' ', ')']).unwrap_or(rest.len());
        if let Ok(value) = rest[..end].trim().parse::<bool>() {
            return Some(value);
        }
    }
    None
}

fn extract_array_from_tokens(tokens: &str, key: &str) -> Option<Vec<String>> {
    // Try different patterns since the tokenizer might have different spacing
    let patterns = [
        format!("{}=[", key),
        format!("{} = [", key),
        format!("{}= [", key),
        format!("{} =[", key),
    ];
    
    for pattern in &patterns {
        if let Some(start) = tokens.find(pattern) {
            let start = start + pattern.len();
            let rest = &tokens[start..];
            if let Some(end) = rest.find(']') {
                let array_content = &rest[..end];
                let items: Vec<String> = array_content
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                return Some(items);
            }
        }
    }
    None
}
