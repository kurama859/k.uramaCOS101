use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("PAU-SMIS.csv").expect("failed to create");
    // Header of the Table
    let header = "Student Name, Matric Number, Department, Level\n";
    file.write_all(header.as_bytes()).expect("failed to write header");
    print!("{}", header);
    // rows of the table
    let rows = vec!["Oluchi Mordi, ACC10211111, Accounting, 300\n"
   ,"Adamas Aliyu, ECO10110101, Economics, 100\n"
    ,"Shania Bolade, CSC10328828, Computer, 200\n"
    ,"Adekunle Gold,EEE11020202, Electrical, 200\n"
    ,"Blanca Edeh, MEE10202001, Mechanical, 300\n"
    ];

      // make a loop for the vectors
    for row in &rows {
        file.write_all(row.as_bytes()).expect("failed to write row");
        print!("{}", row);
    }
    
    println!("\nData written to PAU-SMIS.csv");
}