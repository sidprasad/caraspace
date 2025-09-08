use rust_viz::attribute;
use rust_viz::spytial_annotations::{to_yaml_for_type, to_yaml_for_instance};

/// Direct equivalent to the Python example:
/// ```python
/// @attribute(field="id")
/// class Node:
///     id: int                    # stable integer id (for debugging / refs)
///     v: Optional[str]         # None for constants; otherwise variable name
///     lo: Optional["Node"]       # 0-edge (None for constants)
///     hi: Optional["Node"]       # 1-edge (None for constants)
///
///     def is_const(self) -> bool:
///         return self.v is None
/// ```
#[attribute(field = "id")]
#[derive(Debug, Clone)]
struct Node {
    id: i32,                       // stable integer id (for debugging / refs)
    v: Option<String>,             // None for constants; otherwise variable name  
    lo: Option<Box<Node>>,         // 0-edge (None for constants)
    hi: Option<Box<Node>>,         // 1-edge (None for constants)
}

impl Node {
    fn new_const(id: i32) -> Self {
        Self {
            id,
            v: None,
            lo: None,
            hi: None,
        }
    }
    
    fn new_var(id: i32, name: String) -> Self {
        Self {
            id,
            v: Some(name),
            lo: None,
            hi: None,
        }
    }
    
    fn with_children(mut self, lo: Option<Node>, hi: Option<Node>) -> Self {
        self.lo = lo.map(Box::new);
        self.hi = hi.map(Box::new);
        self
    }

    fn is_const(&self) -> bool {
        self.v.is_none()
    }
}

fn main() {
    println!("=== Python-equivalent Node Structure with Procedural Macros ===\n");

    // Show the generated YAML for the type
    println!("Generated spatial annotations for Node:");
    match to_yaml_for_type::<Node>() {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // Create a binary tree structure exactly like the Python example would create
    let tree = Node::new_var(42, "root".to_string())
        .with_children(
            Some(Node::new_const(1)),  // Left child is a constant
            Some(Node::new_var(2, "variable_y".to_string())  // Right child is a variable
                .with_children(
                    Some(Node::new_const(3)),
                    Some(Node::new_const(4))
                ))
        );
    
    println!("\nExample tree structure:");
    println!("Root (id={}, var={:?}, is_const={})", 
             tree.id, tree.v, tree.is_const());
    
    if let Some(ref left) = tree.lo {
        println!("  Left child (id={}, var={:?}, is_const={})", 
                 left.id, left.v, left.is_const());
    }
    
    if let Some(ref right) = tree.hi {
        println!("  Right child (id={}, var={:?}, is_const={})", 
                 right.id, right.v, right.is_const());
        
        if let Some(ref left_grandchild) = right.lo {
            println!("    Right->Left grandchild (id={}, var={:?}, is_const={})", 
                     left_grandchild.id, left_grandchild.v, left_grandchild.is_const());
        }
        
        if let Some(ref right_grandchild) = right.hi {
            println!("    Right->Right grandchild (id={}, var={:?}, is_const={})", 
                     right_grandchild.id, right_grandchild.v, right_grandchild.is_const());
        }
    }

    println!("\nSpatial annotations will use the 'id' field for visualization references,");
    println!("just like the Python @attribute(field=\"id\") decorator!");
    
    // Show instance-level YAML as well
    println!("\nInstance-level spatial annotations:");
    match to_yaml_for_instance(&tree) {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }
}