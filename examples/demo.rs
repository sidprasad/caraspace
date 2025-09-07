use rust_viz::visualize;

#[derive(serde::Serialize)]
struct Person {
    name: String,
    age: u32,
}

#[derive(serde::Serialize)]
struct Company {
    name: String,
    employees: Vec<Person>,
}

fn main() {
    let c = Company {
        name: "Acme Corp".into(),
        employees: vec![
            Person { name: "Alice".into(), age: 30 },
            Person { name: "Bob".into(), age: 25 },
        ],
    };

    visualize(&c, "");
}
