#[derive(Debug)]
struct Fruit {
    name: String,
    price: u64,
}

impl Fruit {
    fn new(name: String, price: u64) -> Fruit {
        Fruit { name, price }
    }

    fn _get_name(&self) -> &String {
        &self.name
    }

    fn _get_price(&self) -> u64 {
        self.price
    }

    fn _set_name(&mut self, name: String) {
        self.name = name;
    }

    fn _set_price(&mut self, price: u64) {
        self.price = price;
    }
}

fn main() {
    let apples: Vec<u64> = vec![1, 2, 3, 4, 5];
    let mut oranges: Vec<u64> = Vec::new();
    oranges.push(6);
    oranges.push(7);
    oranges.push(8);
    oranges.push(9);
    oranges.push(10);

    println!("apples: {:?}", apples);
    println!("oranges: {:?}", oranges);

    let apples: u8 = 10;
    let oranges: u8 = 10;
    let fruits = apples + oranges;
    println!("Data: {:?}", fruits);

    let mut apple: Fruit = Fruit::new(String::from("Apple"), 10);
    println!("apple: {:?}", apple);
    dbg!(apple);

    let _unuse_apple: Fruit = Fruit::new(String::from("Apple"), 10);

    let mut hym_reps: &u32 = &10;
    println!("hym_reps: {:?}", hym_reps);
    hym_reps = &11;
    println!("hym_reps: {:?}", hym_reps);
}
