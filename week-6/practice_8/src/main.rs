fn main() {
    let num1: f32 = 10.0;
    let num2: f32 = 2.0;
    let mut result: f32;

    result = num1 + num2;
    println!("Sum: {}", result);
    result = num1 - num2;
    println!("Difference: {}", result);
    result = num1 * num2;
    println!("Product: {}", result);
    result = num1 / num2;
    println!("Quotient: {}", result);
    result = num1 % num2;
    println!("Remainder: {}", result);
}