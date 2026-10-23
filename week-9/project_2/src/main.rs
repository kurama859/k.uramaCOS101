use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("PAU-SMIS.csv").unwrap();
    // Header of the Table
    let header = "Student Name, Matric Number, Department, Level\n";
    file.write_all(header.as_bytes()).expect("failed to write header");
    // row 1
    let row1 = "Oluchi Mordi, ACC10211111, Accounting, 300\n";
    file.write_all(row1.as_bytes()).expect("failed to write row1");
    // row 2
    let row2 = "Adamas Aliyu, ECO10110101, Economics, 100\n";
    file.write_all(row2.as_bytes()).expect("failed to write row2");
    // row 3
    let row3 = "Shania Bolade, CSC10328828, Computer, 200\n";
    file.write_all(row3.as_bytes()).expect("failed to write row3");
    // row 4
    let row4 = "Adekunle Gold,EEE11020202, Electrical, 200\n";
    file.write_all(row4.as_bytes()).expect("failed to write row4");
    // row 5
    let row5 = "Blanca Edeh, MEE10202001, Mechanical, 300\n";
    file.write_all(row5.as_bytes()).expect("failed to write row5");
     println!("\nData written");
}