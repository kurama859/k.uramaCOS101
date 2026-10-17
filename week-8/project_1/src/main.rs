 use std::io::{self, Write};

fn check_staff_level(profession: &str, years: u32) {
    let professions = vec![
        "office admin",
        "academic",
        "lawyer",
        "teacher",
    ];

    let positions = vec![
        vec![
            "Intern",
            "Administrator",
            "Senior Administrator",
            "Office Manager",
            "Director",
            "CEO",
        ],
        vec![
            "Not specified",
            "Research Assistant",
            "PhD Candidate",
            "Post-Doc Researcher",
            "Senior Lecturer",
            "Dean",
        ],
        vec![
            "Paralegal",
            "Junior Associate",
            "Associate",
            "Senior Associate 1-2",
            "Senior Associate 3-4",
            "Partner",
        ],
        vec![
            "Placement",
            "Classroom Teacher",
            "Senior Teacher",
            "Leading Teacher",
            "Deputy Principal",
            "Principal",
        ],
    ];

    let profession = profession.trim().to_lowercase();
    let pos = professions.iter().position(|p| * p == profession);
    let index: usize;
    if pos.is_some() {
        index = pos.unwrap();
    } else {
        println!("Invalid profession!");
        println!("Choose: Office Admin, Academic, Lawyer, Teacher");
        return;
    } 
    let row: usize;
    if years <= 1 {
        row = 0;
    } else if years >= 2 && years <= 4 {
        row = 1;
    } else if years >= 5 && years <= 8 {
        row = 2;
    } else if years >= 9 && years <= 12 {
        row = 3;
    } else if years >= 13 && years <= 16 {
        row = 4;
    } else {
        row = 5;
    }

     let aps_level: &str;
    if years <= 1 {
        aps_level = "APS 1-2";
    } else if years >= 2 && years <= 4 {
        aps_level = "APS 3-4";
    } else if years >= 5 && years <= 8 {
        aps_level = "APS 5-8";
    } else if years >= 9 && years <= 12 {
        aps_level = "APS 9-12";
    } else if years >= 13 && years <= 16 {
        aps_level = "APS 13-16";
    } else {
        aps_level = "Senior level";
    }

    println!("\n--- STAFF LEVEL RESULT ---");
    println!("Profession: {}", professions[index]);
    println!("Years of experience: {}", years);
    println!("Position: {}", positions[index][row]);
    println!("Staff level: {}", aps_level);
}

fn main() {
    println!("Federal Government Aps Level Checker");
    println!("Welcome to the Staff level checker");
    println!("Choose your profession: Office Admin, Academic, Lawyer, Teacher");
     let mut profession = String::new();
    let mut experience = String::new();

    print!("Enter profession: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut profession)
        .expect("Failed to read profession");

    print!("Enter years of experience: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut experience)
        .expect("Failed to read experience");
let parsed = experience.trim().parse::<u32>();
    if parsed.is_ok() {
        let years = parsed.unwrap();
        check_staff_level(&profession, years);
    } else {
        println!("Invalid input! Enter a whole number of years.");
    }
}
