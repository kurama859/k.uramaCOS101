use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("Nigeria_breweries.csv").unwrap();
    // Header of the Table
    let header = "Lager, Stout, Non-Alcholic";
    file.write_all(header.as_bytes()).expect("failed to write header");
    // row 1
    let row1 = "33 Export, Legend, Maltina";
    file.write_all(row1.as_bytes()).expect("failed to write row1");
    // row 2
    let row2 = "Desperados, Turbo King, Amstel Malt";
    file.write_all(row2.as_bytes()).expect("failed to write row2");
    // row 3
    let row3 = "Goldberg, Williams, Malta Gold";
    file.write_all(row3.as_bytes()).expect("failed to write row3");
    // row 4
    let row4 = "Gulder,, Fayrouz";
    file.write_all(row4.as_bytes()).expect("failed to write row4");
    // row 5
    let row5 = "Heineken,,";
    file.write_all(row5.as_bytes()).expect("failed to write row5");
    // row 6
    let row6 = "Star,,";
    file.write_all(row6.as_bytes()).expect("failed to write row6");
    println!("\nData written");
}