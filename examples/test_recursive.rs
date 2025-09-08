use json_data_instance_export::{diagram, CndDecorators};
use json_data_instance_export::cnd_annotations::HasCndDecorators;
use serde::Serialize;

#[derive(Serialize, CndDecorators)]
#[attribute(field = "name")]
#[flag(name="company_flag")]
struct Company {
    name: String,
    employees: Vec<Person>,
}

#[derive(Serialize, CndDecorators)]
#[attribute(field = "name")]
#[flag(name="person_flag")]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    println!("=== Testing decorator registration ===");
    
    // Force registration by calling decorators
    let company_decorators = Company::decorators();
    println!("Company decorators: {} constraints, {} directives", 
             company_decorators.constraints.len(), 
             company_decorators.directives.len());
    
    let person_decorators = Person::decorators();
    println!("Person decorators: {} constraints, {} directives", 
             person_decorators.constraints.len(), 
             person_decorators.directives.len());
    
    // Check type registry
    if let Some(registered_company) = json_data_instance_export::cnd_annotations::get_type_decorators("Company") {
        println!("Registered Company decorators: {} constraints, {} directives", 
                 registered_company.constraints.len(), 
                 registered_company.directives.len());
    } else {
        println!("Company not found in type registry");
    }
    
    if let Some(registered_person) = json_data_instance_export::cnd_annotations::get_type_decorators("Person") {
        println!("Registered Person decorators: {} constraints, {} directives", 
                 registered_person.constraints.len(), 
                 registered_person.directives.len());
    } else {
        println!("Person not found in type registry");
    }
    
    println!("\n=== Testing diagram generation ===");
    
    let company = Company {
        name: "Acme Corp".to_string(),
        employees: vec![
            Person { name: "Alice".to_string(), age: 30 },
            Person { name: "Bob".to_string(), age: 25 },
        ],
    };

    diagram(&company);
}