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
         let statement = format!("Commissioner {} Minister {} from the {} region has been convicted based on investigations carried out on the Abuja incident.\n",commissioners[i],ministries[i],zones[i]);
         file.write_all(statement.as_bytes()).expect("failed to write row");
         print!("{} \n", statement);
    }

    print!("Data written")
}