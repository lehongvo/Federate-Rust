fn main() {
    let a: u64 = 10;

    println!("a! is equal to {:?}", a);
}

/**
 * This is a public function.
 *
 * # Arguments
 *
 * * `a` - A u64 value.
 *
 * # Returns
 *
 * * `a` - A u64 value.
 */
pub fn public_function(a: u64) -> u64 {
    println!("public_function");
    a
}
