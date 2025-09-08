use rust_viz::{diagram, spytial_annotations::*};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GraphNode {
    id: String,
    label: String,
    children: Vec<GraphNode>,
}

impl HasSpytialDecorators for GraphNode {
    fn decorators() -> SpytialDecorators {
        SpytialDecoratorsBuilder::new()
            .orientation("children", vec!["below"])
            .atom_color("label", "#4CAF50")
            .size("label", 80, 120)
            .group_field_based("children", 0, 1, None)
            .flag("graph_node")
            .build()
    }
}

fn main() {
    println!("=== Integration Demo: Visualization + Spytial Annotations ===\n");

    // Create a sample graph structure
    let mut graph = GraphNode {
        id: "root".to_string(),
        label: "Root Node".to_string(),
        children: vec![
            GraphNode {
                id: "child1".to_string(),
                label: "Child 1".to_string(),
                children: vec![
                    GraphNode {
                        id: "grandchild1".to_string(),
                        label: "Grandchild 1".to_string(),
                        children: vec![],
                    }
                ],
            },
            GraphNode {
                id: "child2".to_string(),
                label: "Child 2".to_string(),
                children: vec![],
            },
        ],
    };

    // 1. Show the type-level spytial decorators
    println!("1. Type-level Spytial Decorators for GraphNode:");
    match to_yaml_for_type::<GraphNode>() {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // 2. Add instance-level annotations
    annotate_instance(&mut graph, AnnotationBuilder::orientation("self.children", vec!["horizontal"]));
    annotate_instance(&mut graph, AnnotationBuilder::atom_color("self.id", "#FF5722"));
    annotate_instance(&mut graph, AnnotationBuilder::flag("root_instance"));

    println!("2. Combined Decorators (Type + Instance):");
    match to_yaml_for_instance(&graph) {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => println!("Error: {}", e),
    }

    // 3. Create a CnD specification that could use these annotations
    let cnd_spec = r#"
layout default:
  nodes:
    graph_nodes: sourceData.nodes
      - position: hierarchically
      - shape: rectangle  
      - size: [120, 80]
      - color: lightgreen
      - label: this.label
  edges:
    parent_child: sourceData.edges ->* tuples
      - source: this.parent
      - target: this.child
      - color: lightblue
      - arrow: true
"#;

    println!("3. Visualizing the annotated structure...");
    // Note: This would generate and open an HTML visualization
    // For demo purposes, we're not actually opening the browser
    println!("   CnD Spec that could leverage spytial annotations:");
    println!("{}", cnd_spec);

    // 4. Show how spytial decorators could inform visualization
    let decorators = collect_decorators_for_instance(&graph);
    println!("4. Decorator Analysis:");
    println!("   - Total constraints: {}", decorators.constraints.len());
    println!("   - Total directives: {}", decorators.directives.len());
    
    // Analyze the decorators to extract visualization hints
    for constraint in &decorators.constraints {
        match constraint {
            Constraint::Orientation(orient) => {
                println!("   - Orientation hint: {} should be arranged {:?}", 
                        orient.orientation.selector, orient.orientation.directions);
            }
            Constraint::Group(group) => {
                match &group.group {
                    runtime::GroupParams::FieldBased { field, group_on, add_to_group, .. } => {
                        println!("   - Grouping hint: field '{}' groups {} into {}", 
                                field, group_on, add_to_group);
                    }
                    runtime::GroupParams::SelectorBased { selector, name } => {
                        println!("   - Grouping hint: '{}' forms group '{}'", selector, name);
                    }
                }
            }
            _ => {}
        }
    }
    
    for directive in &decorators.directives {
        match directive {
            Directive::AtomColor(color) => {
                println!("   - Color hint: {} should be {}", 
                        color.atom_color.selector, color.atom_color.value);
            }
            Directive::Size(size) => {
                println!("   - Size hint: {} should be {}x{}", 
                        size.size.selector, size.size.width, size.size.height);
            }
            Directive::Flag(flag) => {
                println!("   - Flag: {}", flag.flag);
            }
            _ => {}
        }
    }

    println!("\n5. In a full implementation, these decorators would be used to:");
    println!("   - Automatically generate appropriate CnD layout specifications");
    println!("   - Override default visualization parameters");
    println!("   - Provide semantic information for layout algorithms");
    println!("   - Enable consistent visualization across different data structures");
    
    // Uncomment the line below to actually generate and open the visualization
    // diagram(&graph, cnd_spec);
}