use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("The EFCC Merge.csv").unwrap();
    // Header of the Table
    let header = "Commisioner, Ministry, Zone";
    file.write_all(header.as_bytes()).expect("failed to write header");
    // row 1
    let row1 = "Aigbogun Alamba Daudu, Internal Affairs, South West";
    file.write_all(row1.as_bytes()).expect("failed to write row1");
    // row 2
    let row2 = "Murtala Afeez Bendu, Justice, North East";
    file.write_all(row2.as_bytes()).expect("failed to write row2");
    // row 3
    let row3 = "Okorocha Calistus Ogbona, Defense, South South";
    file.write_all(row3.as_bytes()).expect("failed to write row3");
    // row 4
    let row4 = "Adewale Jimoh Akambi, Power & Steel, South West";
    file.write_all(row4.as_bytes()).expect("failed to write row4");
    // row 5
    let row5 = "Osazuwa Faith Etieye, South East,";
    file.write_all(row5.as_bytes()).expect("failed to write row5");
        println!("\nData written");
}