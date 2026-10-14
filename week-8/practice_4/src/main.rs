fn main() {
    
    //Name vector
    let name =  vec!["Marry","Micheal","James","Paul","James","Daniel","Sally","Luke"];

    //Age vector
    let age = vec![16,17,18,19,220,21,22,23];
    print!("\nAge allocation:\n");

    //loop to iterate items in vector
    for i in 0..age.len(){
        //iterize through i on vector the vector
        print!("{} is {} years old\n",name[i],age[i]);
    }
}
