fn main() {
    
    let v = vec![10, 20, 30];
    // vector v owns the object in the heap

    let v2 = v;   // moves ownership to v2

    display(&v2);
    // v2 is moved to display and v2 is invalidated

    println!("in the main {:?}", v2);
    // v2 is no longer usuable here
}

fn display(v:&Vec<i32>){
    println!("inside display {:?}", v);
}