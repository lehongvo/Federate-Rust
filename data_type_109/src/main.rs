use rust_decimal::prelude::*;
use std::any::type_name;
use std::fmt::Display;

type EightBit = i8;

fn main() {
    let eight_bit = -127_12;
    print_type_of(eight_bit);

    let tax = 12.21f32;
    print_type_of(tax);

    let is_true = true;
    print_type_of(is_true);

    let is_false = false;
    print_type_of(is_false);

    let name = "John";
    print_type_of(name);

    let age = 30;
    print_type_of(age);

    let height = 1.75;
    print_type_of(height);

    let is_student = true;
    print_type_of(is_student);

    let is_teacher = false;
    print_type_of(is_teacher);

    let is_admin = true;
    print_type_of(is_admin);

    let is_guest = false;
    print_type_of(is_guest);

    let is_admin = true;
    print_type_of(is_admin);

    let is_guest = false;
    print_type_of(is_guest);

    let eight_bit: EightBit = -127;
    print_type_of(eight_bit);

    let value_1m: u128 = 1_000_000;
    print_type_of(value_1m);

    // usize
    let mut list_fruits = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
    ];
    list_fruits.push("Apple".to_string());
    let list_fruits_size = list_fruits.len();
    print_type_of(list_fruits_size);

    // isize
    let list_fruits_size_isize: isize = -(list_fruits_size as isize) as isize;
    print_type_of(list_fruits_size_isize);

    let string_value1 = "Hello";
    let string_value2 = String::from("World");
    let string_value3 = string_value1.to_string() + &string_value2;
    print_type_of(string_value3);

    let s = String::new();
    println!("s is empty: {}", s.is_empty());

    let raw = "\tLine 1,\nLine 2 with, \"quotes\" and backslashes";
    println!("raw: {}", raw);

    let value: i32 = 15;
    println!("value: {}", value);
    println!("power of 2: {}", value.pow(2));
    println!("abs: {}", value.abs());
    println!("is_positive: {}", value > 0);
    println!("is_negative: {}", value < 0);
    println!("is_zero: {}", value == 0);
    println!("is_even: {}", is_even(value));
    println!("is_odd: {}", is_odd(value));
    println!("is_prime: {}", is_prime(value));
    println!("is_perfect_square: {}", is_perfect_square(value));
    println!("is_perfect_cube: {}", is_perfect_cube(value));

    let mut empty_string_value = String::from("    value01  ");
    empty_string_value = empty_string_value.trim().to_string();
    println!("empty_string_value: {}", empty_string_value);

    let pi: f64 = 3.14159265358979323846;
    println!("pi: {}", pi);
    println!("pi rounded: {}", pi.round());
    println!("pi truncated: {}", pi.trunc());
    println!("pi ceil: {}", pi.ceil());
    println!("pi floor: {}", pi.floor());
    println!("pi max: {}", pi.max(3.14));
    println!("pi min: {}", pi.min(3.14));
    println!("pi abs: {}", pi.abs());
    println!("pi signum: {}", pi.signum());
    println!("pi is_nan: {}", pi.is_nan());
    print_type_of(pi);

    // f64, if > 0 -> to u32 else 0, using rust_decimal for precise 3-decimal rounding
    let value: f64 = 1000.1123123123;
    let decimal_value = Decimal::from_f64(value).unwrap();
    let rounded_decimal = decimal_value.round_dp(3);
    println!("value: {}", rounded_decimal); // prints with 3 decimal places
    let value_u32: u32 = if rounded_decimal > Decimal::ZERO {
        rounded_decimal.trunc().to_u32().unwrap_or(0)
    } else {
        0
    };
    print_type_of(value_u32);
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn is_odd(n: i32) -> bool {
    n % 2 != 0
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as i32;
    let mut i = 3;
    while i <= limit {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn is_perfect_square(n: i32) -> bool {
    if n < 0 {
        return false;
    }
    let root = (n as f64).sqrt() as i32;
    root * root == n
}

fn is_perfect_cube(n: i32) -> bool {
    let abs_n = n.abs();
    let root = (abs_n as f64).cbrt().round() as i32;
    root * root * root == abs_n
}

pub fn print_type_of<T: Display>(value: T) {
    println!("{} is of type {}", value, type_name::<T>());
}
