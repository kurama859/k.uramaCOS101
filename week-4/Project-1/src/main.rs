// Rust program to calculate the discriminant
use std::io;

fn main()
{
     println!("\nPlease only use for quadratic equations alone");
      println!("\nIf your value for (x^2) is 0 do not use");
       println!("\nThis cannot colve polynomials!");
        println!("\nPlease adhere to instructions");
         println!("\nPlease do not add the variable when giving the value, i do not need the letter");
          println!("\nAlso write the number in numbers not word");
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("\nEnter the value of a which is the number carrying the square of the unknown variable: ");
    io::stdin().read_line(&mut a).expect("Not a valid string");
    let a:f32 = a.trim().parse().expect("Not a valid number, try making it a decimal.");

    println!("Enter the value of b which should just carry the unknown variable: ");
    io::stdin().read_line(&mut b ).expect("Not a valid string");
    let b:f32 = b.trim().parse().expect("Not a valid number, try making it a decimal");

    println!("Enter the value of the number that carrys no variable: ");
    io::stdin().read_line(&mut c).expect("Not a valid string");
    let c:f32 = c.trim().parse().expect("Not a valid number, try making it a decimal");
    // calculate discriminant
    let d:f32 = b*b - 4.0*a*c ;
    // calculate the roots
   // {:.3} tells it to print in 3 d.p
    if d ==0.0 
    {
     let x = -b / (2.0 * a);
     println!("d: {:.3} Exactly one real root",d);
     println!("x = {:.3}", x);
    }
    else if d > 0.0
    {
        
        let x1 = (-b + d.sqrt()) / (2.0 * a);
        let x2 = (-b - d.sqrt()) / (2.0 * a);
        println!("d: {:.3} Two distinct roots",d, );
        println!("x1 = {:.3}", x1);
        println!("x2 = {:.3}", x2);
    }
    else if d < 0.0
    {
        println!("d: {:.3} No real roots",d, );
    }
    else
    {
        println!("Something went wrong");
    }
}