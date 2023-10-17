use std::io;

fn main() {
    println!("Testing control flow");

    let y = 3;
    if y < 2{
        println!("Smallest condition");
    }else if y < 5{
        println!("Middle condition");
    }else{
        println!("Condition not met");
    }

    //since if is an expression, we can use it on tghe right side of a let statement to assign the outcome to a variable
    let n = 5;
    let len = if n < 3 { 0 } else { 10 };

    println!("Len based on n is {len}");

    //Loops
    //special - we can have value returned from loops, helps in assignment of value efficiently. We don't have to keep saving the value in every iteration
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2;//in rust, a 'break' statement can be followed by an expression to specify the value that the loop should break with(return).
                              //The semicolon at the end of it is optional
                              //One expression can directly follow the 'break' statement. For a more complex expression, we can call a function after the 'break'
        }
    };
    println!("Loop ended!! Counter: {counter}, Result: {result}");

    //just checking tuple assignment via returning values from loop
    let (a,b,c) = loop{
        break (10, 20, 30);
    };
    println!("Loop ended!! Tuple values: {a} {b} {c}");

    //another speciality is being able to break specified loops using LABEL PREFIX
    //being in a child nested loop, we can break the parent loop as well. We don't need a flag variable, memory effiecient again
    let mut count = 0;
    'counting_parent: loop{//label prefix
        println!("count = {count}");
        let mut remaining = 10;

        loop{
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 3{
                break 'counting_parent;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //  While loop
    let mut n = 3;
    while n != 0{
        println!("{n}");
        n -= 1;
    }

    //Loop through collections
    let a = [1, 2, 3, 4, 5];

    //Option 1:
    let mut idx = 0;
    while idx < 5 {
        println!{"Array element via while: {}", a[idx]};
        idx += 1;
    }
    //Option 2:
    for ele in a{
        println!("Array element directly via for: {ele}");
    }

    //Nth fibonnaci
    println!("Enter N");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input");

    let n: isize = n.trim().parse().expect("NaN!!");
    // println!("N: {n}");
    println!("{}", fibo(n));
}

fn fibo(mut n: isize) -> isize{
    let mut a = -1;
    let mut b = 1;
    let mut c = 0;
    while n != 0 {
        c = a + b;
        a = b;
        b = c;
        n -= 1;
    }
    c
}
