use json_data_instance_export::{export_json_instance};
use serde::Serialize;
use std::collections::{HashMap, HashSet, BTreeSet};

#[derive(Serialize)]
struct SequenceDemo {
    // Indexable sequences - use idx(container, position, element)
    array: [i32; 3],           // Fixed size array
    vector: Vec<String>,       // Dynamic vector
    tuple: (String, i32, bool), // Heterogeneous tuple
    
    // Key-value mapping
    map: HashMap<String, i32>,
    
    // Sets - would use member(set, element) if we supported them
    // hash_set: HashSet<String>,  // Unordered
    // btree_set: BTreeSet<i32>,   // Ordered by value
}

#[derive(Serialize)]
struct Point(f64, f64); // Tuple struct

fn main() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), 10);
    map.insert("y".to_string(), 20);

    let demo = SequenceDemo {
        array: [1, 2, 3],
        vector: vec!["first".to_string(), "second".to_string()],
        tuple: ("hello".to_string(), 42, true),
        map,
    };

    let point = Point(3.14, 2.71);

    println!("=== SEQUENCE DEMO ===");
    let json_instance = export_json_instance(&demo);
    
    println!("=== ATOMS ===");
    for atom in &json_instance.atoms {
        println!("{}: {} ({})", atom.id, atom.label, atom.r#type);
    }
    
    println!("\n=== RELATIONS ===");
    for relation in &json_instance.relations {
        println!("{}:", relation.name);
        for tuple in &relation.tuples {
            println!("  {:?} -> {:?}", tuple.atoms, tuple.types);
        }
    }

    println!("\n=== POINT (TUPLE STRUCT) ===");
    let point_instance = export_json_instance(&point);
    
    println!("=== ATOMS ===");
    for atom in &point_instance.atoms {
        println!("{}: {} ({})", atom.id, atom.label, atom.r#type);
    }
    
    println!("\n=== RELATIONS ===");
    for relation in &point_instance.relations {
        println!("{}:", relation.name);
        for tuple in &relation.tuples {
            println!("  {:?} -> {:?}", tuple.atoms, tuple.types);
        }
    }
}
