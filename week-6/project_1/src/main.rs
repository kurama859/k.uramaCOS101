use std::io;

fn main() {
    println!("\n========== RESTAURANT MENU ==========");
    println!("P - Poundo Yam / Efo Riro Soup   ₦3,200");
    println!("F - Fried Rice & Chicken         ₦3,000");
    println!("A - Amala & Ewedu Soup           ₦2,500");
    println!("E - Eba & Egusi Soup             ₦2,000");
    println!("W - White Rice & Stew             ₦2,500");
    println!("=====================================");

    println!("\nEnter food type (P, F, A, E or W):");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed to read input");
    let food = food.trim().to_uppercase();

    println!("\nEnter quantity (0-255):");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity: u8 = quantity.trim().parse().expect("Please enter a number between 0-255");

    let price: f64 = match food.as_str() {
        "P" => 3200.0,
        "F" => 3000.0,
        "A" => 2500.0,
        "E" => 2000.0,
        "W" => 2500.0,
        _ => {
            println!("Invalid food type!");
            return;
        }
    };

    let total = price * quantity as f64;

    println!("\nTotal before discount: ₦{:.2}", total);

    if total > 100000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;
        println!("Discount (5%): ₦{:.2}", discount);
        println!("Final amount: ₦{:.2}", final_total);
    } else {
        println!("\nNo discount applied.");
        println!("\nFinal amount: ₦{:.2}", total);
    }
}