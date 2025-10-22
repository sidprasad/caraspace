use json_data_instance_export::{diagram, CndDecorators};
use serde::Serialize;


#[derive(Serialize, CndDecorators)]
struct RBTree {
    root: Option<Box<Node>>,
}

/// Node in the red-black tree with decorators that will be automatically
/// included when processing any type that contains Node fields.
#[derive(Serialize, CndDecorators)]
#[attribute(field = "key")]
#[attribute(field = "color")]
#[orientation(selector="{x, y : Node | x->y in left}", directions=["left", "below"])]
#[orientation(selector="{x, y : Node | x->y in right}", directions=["right", "below"])]
#[hide_atom(selector="Color + u32 + None")]
#[atom_color(selector="{x : Node | @:(x.color) = Red}", value="red")]
#[atom_color(selector="{x : Node | @:(x.color) = Black}", value="black")]
struct Node {
    key: u32,
    color: Color,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

/// Color of a node in the red-black tree
/// Deriving CndDecorators on enums is supported - they just have empty decorators
#[derive(Serialize, CndDecorators, Debug, Clone, Copy)]
enum Color {
    Red,
    Black,
}

impl Node {
    fn new(key: u32, color: Color) -> Self {
        Node {
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
                    self.left = Some(Box::new(Node::new(key, Color::Red)));
                }
            }
        } else if key > self.key {
            match &mut self.right {
                Some(right) => right.insert(key),
                None => {
                    self.right = Some(Box::new(Node::new(key, Color::Red)));
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
                self.root = Some(Box::new(Node::new(key, Color::Black)));
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