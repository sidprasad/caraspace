use rust_viz::spytial_annotations::{
    SpytialDecoratorsBuilder, AnnotationBuilder, HasSpytialDecorators,
    annotate_instance, collect_decorators_for_instance, to_yaml_for_type, to_yaml_for_instance,
    SpytialDecorators,
};

// Example struct with manual decorator implementation
#[derive(Debug)]
struct Node {
    value: String,
    children: Vec<Node>,
}

impl HasSpytialDecorators for Node {
    fn decorators() -> SpytialDecorators {
        SpytialDecoratorsBuilder::new()
            .orientation("value", vec!["above"])
            .group_field_based("children", 0, 1, None)
            .atom_color("value", "lightblue")
            .flag("example_node")
            .build()
    }
}

// Example struct with different decorators
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl HasSpytialDecorators for Person {
    fn decorators() -> SpytialDecorators {
        SpytialDecoratorsBuilder::new()
            .cyclic("self", "clockwise")
            .size("name", 100, 50)
            .hide_field("age", None)
            .build()
    }
}

fn main() {
    println!("=== Spytial Annotations Demo ===\n");

    // 1. Demonstrate type-level decorators
    println!("1. Type-level decorators for Node:");
    match to_yaml_for_type::<Node>() {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    println!("2. Type-level decorators for Person:");
    match to_yaml_for_type::<Person>() {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // 2. Demonstrate instance-level annotations
    println!("3. Instance-level annotations:");
    
    let mut node = Node {
        value: "Root".to_string(),
        children: vec![
            Node {
                value: "Child1".to_string(),
                children: vec![],
            },
            Node {
                value: "Child2".to_string(),
                children: vec![],
            },
        ],
    };

    // Add runtime annotations to the instance
    annotate_instance(&mut node, AnnotationBuilder::orientation("self.children", vec!["horizontal"]));
    annotate_instance(&mut node, AnnotationBuilder::atom_color("self.value", "red"));
    annotate_instance(&mut node, AnnotationBuilder::flag("runtime_annotated"));

    println!("Node with both type and instance decorators:");
    match to_yaml_for_instance(&node) {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // 3. Demonstrate creating decorators with the builder
    println!("4. Custom decorators created with builder:");
    let custom_decorators = SpytialDecoratorsBuilder::new()
        .orientation("items", vec!["vertical", "stack"])
        .group_selector_based("self.elements", "main_group")
        .projection("complex_layout")
        .inferred_edge("connection", "self.links")
        .build();

    match rust_viz::spytial_annotations::to_yaml(&custom_decorators) {
        Ok(yaml) => {
            println!("Custom decorators:");
            println!("{}", yaml);
        },
        Err(e) => println!("Error: {}", e),
    }

    // 4. Demonstrate collecting decorators
    println!("5. Collecting combined decorators:");
    let combined = collect_decorators_for_instance(&node);
    println!("Combined constraint count: {}", combined.constraints.len());
    println!("Combined directive count: {}", combined.directives.len());
}