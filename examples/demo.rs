use rust_viz::attribute;

#[derive(serde::Serialize)]
#[attribute(field = "name")]
struct Company {
    name: String,
    employees: Vec<Person>,
}

#[derive(serde::Serialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let c = Company {
        name: "Acme Corp".into(),
        employees: vec![
            Person { name: "Alice".into(), age: 30 },
            Person { name: "Bob".into(), age: 25 },
        ],
    };

    // The procedural macro generates a diagram() method that automatically
    // includes spatial annotations from the macro
    c.diagram();
}
