use json_data_instance_export::spytial_annotations::{
    to_yaml, Constraint, Directive, HasSpytialDecorators,
};
use json_data_instance_export::SpytialDecorators;
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
