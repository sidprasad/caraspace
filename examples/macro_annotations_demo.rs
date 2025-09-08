use rust_viz::{attribute, orientation, atom_color, flag, size, cyclic, spatial_annotations};
use rust_viz::spytial_annotations::{to_yaml_for_type, to_yaml_for_instance, annotate_instance, AnnotationBuilder};

/// Example struct demonstrating the attribute decorator
/// Similar to Python: @attribute(field="id")
#[attribute(field = "id")]
#[derive(Debug)]
struct Node {
    id: i32,
    v: Option<String>,  // None for constants; otherwise variable name
    lo: Option<Box<Node>>,  // 0-edge (None for constants)
    hi: Option<Box<Node>>,  // 1-edge (None for constants)
}

impl Node {
    fn is_const(&self) -> bool {
        self.v.is_none()
    }
}

/// Example struct with orientation annotation
#[orientation(field = "children", directions = ["vertical", "stack"])]
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    children: Vec<Person>,
}

/// Example struct with atom color annotation
#[atom_color(selector = "name", value = "lightgreen")]
#[derive(Debug)]
struct ColoredPerson {
    name: String,
    age: u32,
}

/// Example struct with size annotation
#[size(selector = "self", width = 120, height = 60)]
#[derive(Debug)]
struct SizedObject {
    value: String,
}

/// Example struct with cyclic layout
#[cyclic(selector = "self", direction = "clockwise")]
#[derive(Debug)] 
struct CircularNode {
    value: String,
    connections: Vec<CircularNode>,
}

/// Example struct with flag annotation
#[flag(value = "special_node")]
#[derive(Debug)]
struct FlaggedNode {
    data: String,
}

/// Example struct using the combined spatial_annotations macro
#[spatial_annotations(
    attribute(field = "id"),
    orientation(field = "children", directions = ["horizontal"])
)]
#[derive(Debug)]
struct CombinedNode {
    id: i32,
    children: Vec<CombinedNode>,
}

fn main() {
    println!("=== Procedural Macro Spatial Annotations Demo ===\n");

    // 1. Demonstrate Node with attribute decorator
    println!("1. Node struct with #[attribute(field = \"id\")]:");
    match to_yaml_for_type::<Node>() {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // Create an instance and show it's working
    let node = Node {
        id: 42,
        v: Some("variable_x".to_string()),
        lo: Some(Box::new(Node {
            id: 1,
            v: None,  // constant
            lo: None,
            hi: None,
        })),
        hi: Some(Box::new(Node {
            id: 2,
            v: Some("variable_y".to_string()),
            lo: None,
            hi: None,
        })),
    };

    println!("Node example - is_const() works: left={}, right={}", 
             node.lo.as_ref().map(|n| n.is_const()).unwrap_or(false),
             node.hi.as_ref().map(|n| n.is_const()).unwrap_or(false));

    // 2. Demonstrate Person with orientation
    println!("\n2. Person struct with orientation annotation:");
    match to_yaml_for_type::<Person>() {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // 3. Demonstrate ColoredPerson with atom_color
    println!("3. ColoredPerson struct with atom_color annotation:");
    match to_yaml_for_type::<ColoredPerson>() {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // 4. Demonstrate SizedObject with size
    println!("4. SizedObject struct with size annotation:");
    match to_yaml_for_type::<SizedObject>() {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // 5. Demonstrate CircularNode with cyclic layout
    println!("5. CircularNode struct with cyclic layout:");
    match to_yaml_for_type::<CircularNode>() {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // 6. Demonstrate FlaggedNode with flag
    println!("6. FlaggedNode struct with flag annotation:");
    match to_yaml_for_type::<FlaggedNode>() {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // 7. Show that runtime annotations still work
    println!("7. Runtime annotations still work:");
    let mut person = Person {
        name: "Alice".to_string(),
        age: 30,
        children: vec![
            Person {
                name: "Bob".to_string(),
                age: 5,
                children: vec![],
            }
        ],
    };

    // Add runtime annotation
    annotate_instance(&mut person, AnnotationBuilder::atom_color("self.name", "red"));
    annotate_instance(&mut person, AnnotationBuilder::flag("runtime_modified"));

    println!("Person with runtime annotations:");
    match to_yaml_for_instance(&person) {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Benefits of Procedural Macro Approach ===");
    println!("✓ Clean, decorator-like syntax similar to Python sPyTial");
    println!("✓ No manual trait implementation required");
    println!("✓ Compile-time validation of decorator parameters");
    println!("✓ Maintains compatibility with runtime annotations");
    println!("✓ Self-documenting code with decorators visible at struct definition");
    println!("✓ Each struct can have a single annotation type or use the combined macro");
}