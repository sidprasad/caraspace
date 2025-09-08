use crate::spytial_annotations::*;

struct TestStruct {
    field1: String,
    field2: i32,
}

impl HasSpytialDecorators for TestStruct {
    fn decorators() -> SpytialDecorators {
        SpytialDecoratorsBuilder::new()
            .orientation("field1", vec!["above", "below"])
            .atom_color("field1", "blue")
            .flag("test_struct")
            .build()
    }
}

#[test]
fn test_builder_pattern() {
    let decorators = SpytialDecoratorsBuilder::new()
        .orientation("selector1", vec!["left", "right"])
        .cyclic("selector2", "counterclockwise")
        .group_field_based("field", 1, 2, Some("sel"))
        .group_selector_based("sel2", "group_name")
        .atom_color("atoms", "red")
        .size("boxes", 50, 100)
        .icon("icons", "/path/to/icon", true)
        .edge_color("edges", "green", None)
        .projection("proj_sig")
        .attribute("attr", Some("attr_sel"))
        .hide_field("hidden", None)
        .hide_atom("hidden_atoms")
        .inferred_edge("edge_name", "edge_sel")
        .flag("builder_test")
        .build();

    assert_eq!(decorators.constraints.len(), 4);
    assert_eq!(decorators.directives.len(), 10);
}

#[test]
fn test_annotation_builder() {
    let orientation = AnnotationBuilder::orientation("test_sel", vec!["up", "down"]);
    assert_eq!(orientation.annotation_type, "orientation");
    assert!(orientation.params.contains_key("selector"));
    assert!(orientation.params.contains_key("directions"));

    let cyclic = AnnotationBuilder::cyclic("test_sel", "clockwise");
    assert_eq!(cyclic.annotation_type, "cyclic");

    let atom_color = AnnotationBuilder::atom_color("test_sel", "yellow");
    assert_eq!(atom_color.annotation_type, "atomColor");

    let flag = AnnotationBuilder::flag("test_flag");
    assert_eq!(flag.annotation_type, "flag");
}

#[test]
fn test_instance_annotation() {
    let mut test_struct = TestStruct {
        field1: "test".to_string(),
        field2: 42,
    };

    // Test annotation with orientation
    let annotation = AnnotationBuilder::orientation("self.field1", vec!["horizontal"]);
    annotate_instance(&mut test_struct, annotation);

    // Test annotation with atom_color
    let annotation2 = AnnotationBuilder::atom_color("self.field2", "orange");
    annotate_instance(&mut test_struct, annotation2);

    // Test annotation with flag
    let annotation3 = AnnotationBuilder::flag("instance_flag");
    annotate_instance(&mut test_struct, annotation3);

    // Collect decorators
    let combined = collect_decorators_for_instance(&test_struct);
    
    // Should have type decorators (1 orientation) + instance decorators (1 orientation) = 2
    assert_eq!(combined.constraints.len(), 2);
    
    // Should have type decorators (1 atom_color, 1 flag) + instance decorators (1 atom_color, 1 flag) = 4
    assert_eq!(combined.directives.len(), 4);
}

#[test]
fn test_yaml_output() {
    let decorators = SpytialDecoratorsBuilder::new()
        .orientation("test", vec!["above"])
        .flag("yaml_test")
        .build();

    let yaml = to_yaml(&decorators).unwrap();
    assert!(yaml.contains("orientation"));
    assert!(yaml.contains("test"));
    assert!(yaml.contains("above"));
    assert!(yaml.contains("flag"));
    assert!(yaml.contains("yaml_test"));
}

#[test]
fn test_has_spytial_decorators() {
    let yaml = to_yaml_for_type::<TestStruct>().unwrap();
    assert!(yaml.contains("orientation"));
    assert!(yaml.contains("field1"));
    assert!(yaml.contains("above"));
    assert!(yaml.contains("below"));
    assert!(yaml.contains("atomColor"));
    assert!(yaml.contains("blue"));
    assert!(yaml.contains("flag"));
    assert!(yaml.contains("test_struct"));
}

#[test]
fn test_self_reference_in_instance_annotation() {
    let mut test_struct = TestStruct {
        field1: "test".to_string(),
        field2: 42,
    };

    let annotation = AnnotationBuilder::orientation("self.children", vec!["horizontal"]);
    annotate_instance(&mut test_struct, annotation);

    let combined = collect_decorators_for_instance(&test_struct);
    let yaml = to_yaml(&combined).unwrap();
    
    // Should contain both original selector and the self-substituted selector
    assert!(yaml.contains("field1")); // from type decorator
    assert!(yaml.contains("obj_")); // from instance decorator with self substitution
}

#[test]
fn test_group_constraints() {
    let decorators = SpytialDecoratorsBuilder::new()
        .group_field_based("items", 0, 1, None)
        .group_selector_based("elements", "main_group")
        .build();

    assert_eq!(decorators.constraints.len(), 2);
    
    let yaml = to_yaml(&decorators).unwrap();
    assert!(yaml.contains("group"));
    assert!(yaml.contains("items"));
    assert!(yaml.contains("groupOn"));
    assert!(yaml.contains("addToGroup"));
    assert!(yaml.contains("elements"));
    assert!(yaml.contains("main_group"));
}

#[test]
fn test_multiple_instance_annotations() {
    let mut test_struct1 = TestStruct {
        field1: "test1".to_string(),
        field2: 1,
    };
    let mut test_struct2 = TestStruct {
        field1: "test2".to_string(),
        field2: 2,
    };

    // Annotate different instances
    annotate_instance(&mut test_struct1, AnnotationBuilder::flag("instance1"));
    annotate_instance(&mut test_struct2, AnnotationBuilder::flag("instance2"));

    let combined1 = collect_decorators_for_instance(&test_struct1);
    let combined2 = collect_decorators_for_instance(&test_struct2);

    // Both should have type decorators + their own instance decorators
    assert_eq!(combined1.directives.len(), 3); // 2 type + 1 instance
    assert_eq!(combined2.directives.len(), 3); // 2 type + 1 instance

    let yaml1 = to_yaml(&combined1).unwrap();
    let yaml2 = to_yaml(&combined2).unwrap();

    assert!(yaml1.contains("instance1"));
    assert!(!yaml1.contains("instance2"));
    assert!(yaml2.contains("instance2"));
    assert!(!yaml2.contains("instance1"));
}