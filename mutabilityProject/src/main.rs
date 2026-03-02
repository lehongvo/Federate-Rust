const MAX_POINTS: u32 = 100_000;
const APPLE_PRICE: f64 = 1.0;
const ORANGE_PRICE: f64 = 2.0;

fn main() {
    let x = 10;
    let y = &x;
    println!("x: {}", x);
    println!("y: {}", y);

    let mut full_name: String = String::from("John");
    full_name.push_str(" Doe");
    let full_name_length = full_name.len();
    println!(
        "full_name: {}, full_name_length: {}",
        full_name, full_name_length
    );

    println!("MAX_POINTS: {}", MAX_POINTS);
    println!("APPLE_PRICE: {}", APPLE_PRICE);
    println!("ORANGE_PRICE: {}", ORANGE_PRICE);

    let name: String = String::from("John");
    let age: u32 = 20;
    println!("name: {}, age: {}", name, age);

    let var: u64 = 10;
    let _var = var + 1;
    let var: String = String::from("Hello");
    println!("var: {}", var);

    let new_number: u64 = get_number();
    println!("new_number: {}", new_number);

    {
        let x1 = 5;
        println!("x: {}", x1);
    }

    {
        type MyType = u64;
        let y: MyType = 10;
        println!("y: {}", y);
    }

    print_function();
}

pub fn get_number() -> u64 {
    10
}

pub fn print_function() {
    println!("print_function");
}
