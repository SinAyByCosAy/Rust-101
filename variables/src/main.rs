fn main() {
    //immutable
    let x = 6;
    println!("value of x: {x}");

    // x = 5;//this line will throw an error
    // println!("value of x: {x}");

    //mutable
    let mut x = 5;
    println!("value of x: {x}");

    x = 6;
    println!("value of x: {x}");
}
