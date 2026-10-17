use std::io;

fn main() {
    let mut input = String::new();

    println!("How many candidates are you interviewing?");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number_of_candidates: usize =
        input.trim().parse().expect("Enter a valid number");

    if number_of_candidates == 0 {
        println!("No candidates were entered.");
        return;
    }

    let mut candidates: Vec<(String, u32)> = Vec::new();

    // Collect candidate details
    for i in 0..number_of_candidates {
        let mut name = String::new();

        println!("Enter the name of candidate {}:", i + 1);

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read name");

        let name = name.trim().to_string();

        let mut experience_input = String::new();

        println!("Enter years of programming experience:");

        io::stdin()
            .read_line(&mut experience_input)
            .expect("Failed to read experience");

        let years: u32 = experience_input
            .trim()
            .parse()
            .expect("Enter a valid whole number");

        candidates.push((name, years));
    }

    // Find the candidate with the highest experience
    let mut highest_index = 0;

    for i in 1..candidates.len() {
        if candidates[i].1 > candidates[highest_index].1 {
            highest_index = i;
        }
    }

    // Display the result
    println!(
        "Most experienced candidate: {}",
        candidates[highest_index].0
    );

    println!(
        "Years of programming experience: {}",
        candidates[highest_index].1
    );
}