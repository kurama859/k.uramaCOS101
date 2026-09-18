// Rust program to read the age of a person
// and then print the person level of experience


use std::io;

fn main()
{
     println!("Incentive Calculator");

    //experience input
    println!("\nIs employee experienced? (yes/no): ");
    let mut exp = String::new();
    io::stdin().read_line(&mut exp).expect("Failed");
    let exp = exp.trim().to_lowercase();
    
    let is_experienced = exp == "yes" || exp == "y" || exp == "experienced";
    // Age input
    let mut age = String::new();
    println!("\nEnter Your Age : ");
    io::stdin().read_line(&mut age).expect("Not a valid string");
    let age:u8 = age.trim().parse().expect("Not a valid number");
    
    let incentive:u32;
    if !is_experienced
     {
        incentive = 100_000;
    }
     else if age >= 40 
    {
        incentive = 1_560_000;
    }
     else if age >= 30 && age <= 39
      {
        incentive = 1_480_000;
    }
     else if age < 30 
     {
      // this covers <28 and also 28-29 gap which i am assuming is part
        incentive = 1_300_000;
    }
     else
      {
        incentive = 0;
    }

    println!("\nAnnual Incentive: N{}", incentive);
}