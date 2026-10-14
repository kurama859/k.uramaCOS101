fn main() {
    
    let v = vec!['c','o','m','p','u','t','e','r'];

    let mut input1 = String::new();

    println!("Enter an index value btw(0-7)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    //index is tht non negative value which is smaller than the vector
    let index:usize=input1.trim().parse().expect("Invalid inpurt");

    //getting value at given index value
    let ch: char = v[index];

    print!("{} is the character for index[{}]\n",ch,index);
}
