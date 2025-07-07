use json_data_instance_export::export_json_instance;

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

    let result = export_json_instance(&c);
    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
