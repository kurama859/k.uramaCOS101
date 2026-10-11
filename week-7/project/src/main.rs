use std::io;

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, try again."),
        }
    }
}

// 1. Trapezium area = height / 2 * (base1 + base2)
fn trapezium_area() -> f64 {
    let height = read_number("Enter height:");
    let base1 = read_number("Enter base1:");
    let base2 = read_number("Enter base2:");
    height / 2.0 * (base1 + base2)
}

// 2. Rhombus area = 1/2 * diagonal1 * diagonal2
fn rhombus_area() -> f64 {
    let d1 = read_number("Enter diagonal1:");
    let d2 = read_number("Enter diagonal2:");
    0.5 * d1 * d2
}

// 3. Parallelogram area = base * altitude
fn parallelogram_area() -> f64 {
    let base = read_number("Enter base:");
    let altitude = read_number("Enter altitude:");
    base * altitude
}

// 4. Cube surface area = 6 * side * side
fn cube_surface_area() -> f64 {
    let side = read_number("Enter side:");
    6.0 * side * side
}

// 5. Cylinder volume = pi * radius * radius * height
fn cylinder_volume() -> f64 {
    let radius = read_number("Enter radius:");
    let height = read_number("Enter height:");
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("=== The Shape Calculator ===");
    println!("1. Trapezium, area");
    println!("2. Rhombus, area");
    println!("3. Parallelogram, area");
    println!("4. Cube, surface area");
    println!("5. Cylinder, volume");

    println!("\nEnter your choice (1-5):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
        .expect("Failed to read choice");

    match choice.trim() {
        "1" => {
            let area = trapezium_area();
            println!("Area of Trapezium = {:.2}", area);
        },
        "2" => {
            let area = rhombus_area();
            println!("Area of Rhombus = {:.2}", area);
        },
        "3" => {
            let area = parallelogram_area();
            println!("Area of Parallelogram = {:.2}", area);
        },
        "4" => {
            let area = cube_surface_area();
            println!("Surface Area of Cube = {:.2}", area);
        },
        "5" => {
            let vol = cylinder_volume();
            println!("Volume of Cylinder = {:.2}", vol);
        },
        _ => {
            println!("Invalid choice!");
        }
    }
}