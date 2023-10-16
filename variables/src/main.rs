use std::io;
fn main() {
    //immutable
    let k = 6;
    println!("value of k: {k}");

    // k = 5;//this line will throw an error
    // println!("value of x: {x}");

    //mutable
    let mut x = 5;
    println!("value of x: {x}");

    x = 6;
    println!("value of x: {x}");

    //Shadowing
    //Shadowing helps to perform some operartions on the variable and then have the variable as immutable again after that. If it's declared
    //mutable then we can still make changes later.
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
    //we can't change type of a mutable var


    println!("{}",-5%7);//if(a<0) -> a%b = a%b + b for correct answer

    //Rust is a statically typed language and it must know the data type of all variables at compile time
    //tuple, once declared can't grow or shrink in size.
    let t: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}",t.0);

    let t = (14.2, false, 22);
    let (r,g,b) = t;  //destructuring
    println!("{r} {g} {b}");


    //array, data on stack memory and not heap
    let a = [1, 2, 3, 4, 5];
    println!("Enter an index");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)//readline appends and not overwrites
        .expect("FAILED!");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number");

    let ele = a[index];
    println!("Value at idx: {index} is {ele}");

    let b = [3; 5];//makes an array of 5 elements with all values as 3 -> [3, 3, 3, 3, 3]
    println!("{} {}",b[0],b[1]);
}
