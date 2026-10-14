fn main() {
    
    // Using Vec::new();
    let v1 : Vec<i64> = Vec::new();
    // printing the size of vector
    println!("\nThe lenght of Vec::new is: {}",v1.len() );

    // Using macro
    let v2 = vec!["Grace","Effiong","Basil","Kareem","Susan"];

// printing the size of the vector
println!("\nThe lenght of vec macro is: {}",v2.len() );
}