//define dimensions of a rectangle
struct Rectangle {
    width:u32, height:u32
}

// logic to calculate area of a rectangle
impl Rectangle {
    fn area(&self)->u32{
    // add code here
    self.width * self.height
}
}
fn main() {
    // instanatiate the structure
    let small = Rectangle{
        width:10,
        height:30
    };
    // print the rectangle's area
    println!("width is {}\n height is {}\narea of Rectangle is {}", small.height, small.width, small.area());
}
