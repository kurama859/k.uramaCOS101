fn main() {
    let applicants: Vec<(&str, u8)> = vec![("Ada", 8), ("Kelechi", 6), ("Emeka", 9), ("Luke", 7)];

    // check for empty vector
    if applicants.is_empty() {
        println!("No applicants were found.");
    } else {
        // assume first person is the most experienced
        let mut most_experienced = applicants[0];

        // checks the applicants
        for applicant in &applicants {
            println!("Checking: {}", applicant.0);

            // looks for who has more years
            if applicant.1 > most_experienced.1 {
                most_experienced = *applicant;
            }
        }

        // display the answer - MUST be inside else
        println!("\nMost experienced applicant: {}", most_experienced.0);
        println!("Years of experience: {}", most_experienced.1);
    }
}