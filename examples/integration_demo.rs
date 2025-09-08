use rust_viz::attribute;
use serde::Serialize;

/// This demo shows how spatial annotations integrate seamlessly with visualization
/// The procedural macros automatically generate spatial annotation data that
/// informs the visualization layout and styling.

#[derive(Debug, Serialize)]
#[attribute(field = "name")]
struct Company {
    name: String,
    departments: Vec<Department>,
}

#[derive(Debug, Serialize)]
struct Department {
    name: String,
    employees: Vec<Employee>,
}

#[derive(Debug, Serialize)]
struct Employee {
    name: String,
    role: String,
}

fn main() {
    println!("=== Integration Demo: Spatial Annotations + Visualization ===\n");

    // Create a sample organizational structure
    let company = Company {
        name: "Tech Corp".to_string(),
        departments: vec![
            Department {
                name: "Engineering".to_string(),
                employees: vec![
                    Employee { name: "Alice".to_string(), role: "Senior Developer".to_string() },
                    Employee { name: "Bob".to_string(), role: "Junior Developer".to_string() },
                ],
            },
            Department {
                name: "Marketing".to_string(),
                employees: vec![
                    Employee { name: "Carol".to_string(), role: "Marketing Manager".to_string() },
                ],
            },
        ],
    };

    println!("Visualizing company structure with spatial annotations...");
    println!("The @attribute(field=\"name\") annotation tells the visualizer to");
    println!("use the 'name' field as the primary identifier for layout and references.\n");
    
    // The spatial annotation automatically integrates with the diagram function
    // No need to manually specify CnD specs - they're generated from the annotations
    company.diagram();
    
    println!("A browser window should open with the visualization.");
    println!("The spatial annotations ensure consistent and meaningful layout.");
}