use json_data_instance_export::{diagram, SpytialDecorators};
use serde::Serialize;


#[derive(Serialize, SpytialDecorators)]
struct RBTree {
    root: Option<Box<RBNode>>,
}

/// RBNode in the red-black tree with decorators that will be automatically
/// included when processing any type that contains RBNode fields.
#[derive(Serialize, SpytialDecorators)]
#[attribute(field = "key")]
#[attribute(field = "color")]
#[orientation(selector="{x, y : RBNode | x->y in left}", directions=["left", "below"])]
#[orientation(selector="{x, y : RBNode | x->y in right}", directions=["right", "below"])]
#[hide_atom(selector="Color + u32 + None")]
#[atom_color(selector="{x : RBNode | @:(x.color) = Red}", value="red")]
#[atom_color(selector="{x : RBNode | @:(x.color) = Black}", value="black")]
struct RBNode {
    key: u32,
    color: Color,
    left: Option<Box<RBNode>>,
    right: Option<Box<RBNode>>,
}

/// Color of a node in the red-black tree
/// Deriving SpytialDecorators on enums is supported - they just have empty decorators
#[derive(Serialize, SpytialDecorators, Debug, Clone, Copy)]
enum Color {
    Red,
    Black,
}

impl RBNode {
    fn new(key: u32, color: Color) -> Self {
        RBNode {
            key,
            color,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, key: u32) {
        if key < self.key {
            match &mut self.left {
                Some(left) => left.insert(key),
                None => {
                    self.left = Some(Box::new(RBNode::new(key, Color::Red)));
                }
            }
        } else if key > self.key {
            match &mut self.right {
                Some(right) => right.insert(key),
                None => {
                    self.right = Some(Box::new(RBNode::new(key, Color::Red)));
                }
            }
        }
    }
}

impl RBTree {
    fn new() -> Self {
        RBTree { root: None }
    }

    fn insert(&mut self, key: u32) {
        match &mut self.root {
            Some(root) => root.insert(key),
            None => {
                self.root = Some(Box::new(RBNode::new(key, Color::Black)));
            }
        }
    }
}

fn main() {

    //
    let mut tree = RBTree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);

    diagram(&tree);
}