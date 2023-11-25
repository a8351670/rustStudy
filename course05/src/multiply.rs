macro_rules! multiply {
    ($x:expr, $y:expr) => {
        $x * $y
    };
}

fn main() {
    let result = multiply!(4, 5);
    println!("Result: {}", result);
}