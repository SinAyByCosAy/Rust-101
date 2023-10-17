fn main() {
    println!("Learning functions, statements and expressions");
    test_function();
    value_function(10, 'k');

    //statement, doesn't return a value
    let y = 3; //in rust we can't have x = y = 3 like in C++ or Java where the assignment returns the value of the assignment

    //expression, evaluates to a value
    let n = {
        let x = 5;
        x + 1 //Expressions do not include ending semicolons otherwise it will not return a value -> Important
    };
    println!("Value of n is {n}");

    //testing return functions
    let x = return_function();
    println!("Returned value : {}", return_function());
    println!("Returned value + 1 : {}", x+1);

    //testing parameterized return function
    let x = parameter_return_fn(68);
    println!("Returned value from parameterized fn : {x}");
    
}

fn test_function(){
    println!("Called fn");
}

//compulsory to declare type of each parameter
fn value_function(n: usize, ch: char){
    println!("The passed value of int is : {n} and char is : {ch}");
}

fn return_function() -> usize{//return type
    8 //expression, no ';' needed for return
}

fn parameter_return_fn(n: usize) -> usize{
    n + 1
}
