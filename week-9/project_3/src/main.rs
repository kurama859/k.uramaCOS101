use std::io::Write;

fn main() {
    let mut file = std::fs::File::create("THE EFCC MERGE.txt").expect("failed to create");
    // commisioner vector
    let commissioners = vec![ "Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
    // ministers vector
    let ministries = vec![ "Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    // zones vector
    let zones = vec![ "South West", "North East", "South South", "South West", "South East"];

    for i in 0..commissioners.len(){
        //iterize through i on vector the vector
      let mut statement = String::new();
        statement.push_str("Commissioner ");
        statement.push_str(commissioners[i]);
        statement.push_str(", minister of ");
        statement.push_str(ministries[i]);
        statement.push_str("from");
        statement.push_str(zones[i]);
        statement.push_str(" region has been charged and convicted.\n");

        file.write_all(statement.as_bytes()).expect("failed to write statement");
        print!("{}", statement);
    }
        println!("\nData written to THE EFCC MERGE.txt");
    
}