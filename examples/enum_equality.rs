#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Black,
}

fn main() {
    let c1 = Color::Red;
    let c2 = Color::Red;
    let c3 = Color::Black;
    
    // Are two Red values the same?
    println!("Color::Red == Color::Red: {}", c1 == c2);
    println!("Color::Red == Color::Black: {}", c1 == c3);
    
    // What about memory addresses?
    println!("\nMemory addresses:");
    println!("c1 address: {:p}", &c1);
    println!("c2 address: {:p}", &c2);
    
    // More complex: enums with data
    #[derive(Debug, PartialEq)]
    enum Value {
        Number(u32),
        Text(String),
    }
    
    let v1 = Value::Number(42);
    let v2 = Value::Number(42);
    let v3 = Value::Number(99);
    
    println!("\nWith data:");
    println!("Number(42) == Number(42): {}", v1 == v2);
    println!("Number(42) == Number(99): {}", v1 == v3);
    
    // Unit variants are like unit structs
    #[derive(Debug, PartialEq)]
    struct UnitStruct;
    
    let u1 = UnitStruct;
    let u2 = UnitStruct;
    println!("\nUnit structs:");
    println!("UnitStruct == UnitStruct: {}", u1 == u2);
}
