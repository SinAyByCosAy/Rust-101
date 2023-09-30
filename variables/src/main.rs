use std::io;
fn main() {
    //immutable
    let k = 6;
    println!("value of x: {k}");

    // k = 5;//this line will throw an error
    // println!("value of x: {x}");

    //mutable
    let mut x = 5;
    println!("value of x: {x}");

    x = 6;
    println!("value of x: {x}");

    //Shadowing
    let y = 10;

    let y = y + 1;
    {
        let y = y * 2;
        println!("Value of y from inner scope is: {y}");
    }
    println!("Value of y: {y}");

    //we can also change type as it's basically a new initialization
    let spaces = "    ";
    let spaces = spaces.len();
    println!("Value of spaces: {spaces}");

    //we can't do change type of a mutable var

    //tuple
    let t = (14.2, false, 22);
    let (r,g,b) = t;
    println!("{r} {g} {b}");

    let r = t.0;
    let g = t.1;
    let b = t.2;

    //array
    let a = [1, 2, 3, 4, 5];
    println!("Enter an index");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("FAILED!");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number");

    let ele = a[index];
    println!("Value at idx: {index} is {ele}");
}
