const COOK_TIME: &str = "10 minutes";
const PI: f64 = 3.14;
const TAX_RATE: f64 = 0.05;
type ProteinGrams = f64;

fn main() {
    #[allow(unused_variables)]
    let x: u128 = 10;
    let x: u128 = 20;
    println!("x: {:?}", x);

    #[allow(unused_variables)]
    let x: u128 = 30;
    let x: u128 = 30;
    println!("x: {:?}", x);

    let x: u128 = 40;
    println!("x: {:?}", x);

    let grams_of_protein: &str = "100.23";
    println!("grams_of_protein: {:?}", grams_of_protein);

    let mut grams_of_protein: f64 = 200.45;
    println!("grams_of_protein: {:?}", grams_of_protein);
    grams_of_protein = "300.67".parse().unwrap();
    println!("grams_of_protein: {:?}", grams_of_protein);

    let _coffee_beans: &str = "100.23";

    {
        let cook_time: &str = "10 minutes";
        println!("cook_time: {:?}", cook_time);
    }

    println!("cook_time: {:?}", COOK_TIME);
    println!("PI: {:?}", PI);
    println!("TAX_RATE: {:?}", TAX_RATE);

    let carrots: ProteinGrams = 100.23;
    println!("carrots: {:?}", carrots);
}
