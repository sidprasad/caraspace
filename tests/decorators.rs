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
