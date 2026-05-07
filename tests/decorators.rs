use caraspace::spytial_annotations::{
    to_yaml, Constraint, Directive, HasSpytialDecorators,
};
use caraspace::SpytialDecorators;
use serde::Serialize;

#[derive(Serialize, SpytialDecorators)]
#[align(selector = "peer", direction = "horizontal")]
#[orientation(selector = "peer", directions = ["right"])]
#[flag(name = "important")]
struct DerivedNode {
    id: u32,
}

#[test]
fn derive_macro_emits_align_and_existing_decorators() {
    let decorators = DerivedNode::decorators();

    assert!(decorators.constraints.iter().any(|constraint| {
        matches!(constraint, Constraint::Align(align)
            if align.align.selector == "peer" && align.align.direction == "horizontal")
    }));
    assert!(decorators.constraints.iter().any(|constraint| {
        matches!(constraint, Constraint::Orientation(orientation)
            if orientation.orientation.selector == "peer"
            && orientation.orientation.directions == vec!["right".to_string()])
    }));
    assert!(decorators.directives.iter().any(|directive| {
        matches!(directive, Directive::Flag(flag) if flag.flag == "important")
    }));

    let yaml = to_yaml(&decorators).unwrap();
    assert!(yaml.contains("align:"));
    assert!(yaml.contains("direction: horizontal"));
    assert!(yaml.contains("orientation:"));
    assert!(yaml.contains("flag: important"));
}

#[derive(Serialize, SpytialDecorators)]
#[tag(to_tag = "Person", name = "status", value = "Person.status")]
struct TaggedPerson {
    name: String,
    status: String,
}

#[test]
fn tag_directive_single() {
    let decorators = TaggedPerson::decorators();

    let tag = decorators
        .directives
        .iter()
        .find_map(|d| match d {
            Directive::Tag(t) => Some(t),
            _ => None,
        })
        .expect("expected a Tag directive");

    assert_eq!(tag.tag.to_tag, "Person");
    assert_eq!(tag.tag.name, "status");
    assert_eq!(tag.tag.value, "Person.status");

    let yaml = to_yaml(&decorators).unwrap();
    assert!(yaml.contains("tag:"));
    assert!(yaml.contains("toTag: Person"));
    assert!(yaml.contains("name: status"));
    assert!(yaml.contains("value: Person.status"));
}

#[derive(Serialize, SpytialDecorators)]
#[tag(to_tag = "Person", name = "age", value = "Person.age")]
#[tag(to_tag = "Car", name = "owner", value = "Car.ownedBy")]
struct MultiTagged {
    id: u32,
}

#[derive(Serialize, SpytialDecorators)]
#[edge_style(field = "left", value = "#000000")]
struct EdgeStyledMinimal {
    id: u32,
}

#[test]
fn edge_style_directive_minimal() {
    let decorators = EdgeStyledMinimal::decorators();

    let edge = decorators
        .directives
        .iter()
        .find_map(|d| match d {
            Directive::EdgeStyle(e) => Some(&e.edge_style),
            _ => None,
        })
        .expect("expected an EdgeStyle directive");

    assert_eq!(edge.field, "left");
    assert_eq!(edge.value, "#000000");
    assert!(edge.style.is_none());
    assert!(edge.weight.is_none());
    assert!(edge.show_label.is_none());
    assert!(edge.hidden.is_none());
    assert!(edge.filter.is_none());
    assert!(edge.selector.is_none());

    let yaml = to_yaml(&decorators).unwrap();
    assert!(yaml.contains("edgeColor:"));
    assert!(yaml.contains("field: left"));
    // Optional fields are skipped when None.
    assert!(!yaml.contains("style:"));
    assert!(!yaml.contains("weight:"));
    assert!(!yaml.contains("showLabel:"));
    assert!(!yaml.contains("hidden:"));
}

#[derive(Serialize, SpytialDecorators)]
#[edge_style(
    field = "right",
    value = "blue",
    style = "dashed",
    weight = 2.5,
    show_label = false,
    hidden = true,
    filter = "Node3 -> Node1",
    selector = "Tree"
)]
struct EdgeStyledAllOptions {
    id: u32,
}

#[test]
fn edge_style_directive_all_options() {
    let decorators = EdgeStyledAllOptions::decorators();

    let edge = decorators
        .directives
        .iter()
        .find_map(|d| match d {
            Directive::EdgeStyle(e) => Some(&e.edge_style),
            _ => None,
        })
        .expect("expected an EdgeStyle directive");

    assert_eq!(edge.field, "right");
    assert_eq!(edge.value, "blue");
    assert_eq!(edge.style.as_deref(), Some("dashed"));
    assert_eq!(edge.weight, Some(2.5));
    assert_eq!(edge.show_label, Some(false));
    assert_eq!(edge.hidden, Some(true));
    assert_eq!(edge.filter.as_deref(), Some("Node3 -> Node1"));
    assert_eq!(edge.selector.as_deref(), Some("Tree"));

    let yaml = to_yaml(&decorators).unwrap();
    assert!(yaml.contains("edgeColor:"));
    assert!(yaml.contains("style: dashed"));
    assert!(yaml.contains("weight: 2.5"));
    assert!(yaml.contains("showLabel: false"));
    assert!(yaml.contains("hidden: true"));
    assert!(yaml.contains("filter: Node3"));
}

#[test]
fn tag_directive_multiple() {
    let decorators = MultiTagged::decorators();

    let tags: Vec<_> = decorators
        .directives
        .iter()
        .filter_map(|d| match d {
            Directive::Tag(t) => Some(&t.tag),
            _ => None,
        })
        .collect();

    assert_eq!(tags.len(), 2);
    assert!(tags.iter().any(|t| t.to_tag == "Person" && t.name == "age"));
    assert!(tags.iter().any(|t| t.to_tag == "Car" && t.name == "owner"));
}
