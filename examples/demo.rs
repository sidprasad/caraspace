use rust_viz::{diagram_with_annotations, cnd_annotations::*};

#[derive(serde::Serialize)]
struct Company {
    name: String,
    employees: Vec<Person>,
}

#[derive(serde::Serialize)]
struct Person {
    name: String,
    age: u32,
}

impl HasCndDecorators for Company {
    fn decorators() -> CndDecorators {
        CndDecoratorsBuilder::new()
            .attribute("name", None)
            .build()
    }
}

fn main() {
    let c = Company {
        name: "Acme Corp".into(),
        employees: vec![
            Person { name: "Alice".into(), age: 30 },
            Person { name: "Bob".into(), age: 25 },
        ],
    };

    // Uses CnD annotations from the HasCndDecorators trait
    diagram_with_annotations(&c);
}
