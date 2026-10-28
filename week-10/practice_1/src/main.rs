fn main() {
    
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in the heap

    // only a single variable owns a heap memory at a given time
    let _v2 = &v;
    // heere two variables own the heap memory at any given time
    //vtwo pointers to the same content is not allowed in rust

    // Rust is very smart in terms of memory access, so it detects a race condition
    // as two variables point to same heap

    println!("{:?}", v);
}
