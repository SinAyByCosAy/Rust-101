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
}
