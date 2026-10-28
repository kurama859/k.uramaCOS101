//define structure of the calculator
use std::io;
struct AlE {
    quantity:u64,
    brand:String,
    price:u64
}

// logic to calculate total unit cost
impl AlE {
    fn unit_price(&self)->u64{
    self.quantity  * self.price
}

}
fn main() {
    println!("Alaba Electronics.");
    println!("Brand\t Quantity\t Price");
    println!("HP\t 10\t         N650,000");
    println!("IBM\t 6\t         N755,000");
    println!("Toshiba\t 10\t         N550,000");
    println!("Dell\t 4\t         N850,000");

    // user interface
    let mut input1 = String::new();
    println!("Input the brand you wish to purchase.(H, I, T, D)");
    io::stdin().read_line(&mut input1).expect("Failed to read input");

    // where the magic happens/ background work
    let brand = input1.trim().to_uppercase();
    
    let (brand, price) : (&str, u64) = match brand.as_str() {
        "H" => ( "HP", 650_000),
        "I" => ("IBM", 755_000),
        "T" => ("Toshiba", 550_000),
        "D" => ("Dell", 850_000),
        _ => {
            println!("Invalid Brand");
            return;
        }
    };

    let mut number = String::new();
    println!("Enter quantity");
    io::stdin().read_line(&mut number).expect("Failed to read input");

    let quantity: u64 = number.trim().parse().expect("Please enter a valid number");

    let x = AlE {
        brand: brand.to_string(),
        quantity: quantity,
        price: price
    };
    println!("Product name {}", x.brand);
    println!("Quantity bought {}", x.quantity);
    println!("Price per unit N{}", x.price);
    println!("The total price for your purchase is N{}", x.unit_price());
}
