fn main() {
    println!("Understanding ownership via strings!!");

    //string literal,immutable 
    
    //We can't make changes to the same value(ex-append). Only way -> store a new value completely.
    //We know the content at compile time, the text gets hardcoded into final executable.
    //So the text is fast and efficient but these properties come from string literal's immutability.
    
    let s = "hello";

    
    //String type, mutable

    //We can't put a random memory chunk into the binary for each piece of text with unknown size at compile time as it might change
    //For this we need allocation on heap. Therefore, 
    
    //1. The memory must be requested from the memory allocator at runtime and
    //2. There has to be a way to return this memory chunk to allocator when done.

    let mut st = String::from("Test");//First part is done by programmer here when calling String::from, it requests the memory needed
    st.push_str(" operation on String");
    //Second part of returning memory is generally solved by Garbage collectors or is programmer's responsibility which is an issue.
    println!("{}", st);

    //In Rust, memory is automatically returned once the variable goes out of scope

    rest_ownership();
    deep_copy();

}//Rust automatically calls drop at closing bracket where the author of String can put the code to return the memory

fn rest_ownership(){
    let x = 3;
    let y = x;//here value is copied and both x and then y are inserted into the stack as integers are of fixed size
    println!("{x}");//we can access x here

    let s1 = String::from("Hello");//Above case is not true here
    let s2 = s1;//the actual value doesn't get copied. s1 holds pointer to the heap memory that stores the String "Hello", s2 simply copies the same pointer
    //the original and the copied pointer stays on the stack and holds info of data in heap memory

    //Since both the pointers point to the same memory location, when both go out of scope, 
    //both will return the memory to the allocator and try to free the heap leading to "double free error"
    
    //To ensure, memory safety Rust considers s1 as no longer valid  after the line -> let s2 = s1;  (s1 was moved into s2)
    //Therefore Rust doesn't need to free anything when s1 goes out of scope
    println!("{s2}");
   //we can't use s1 here, it'll throw an error

}//here now only s2 will free the memory

fn deep_copy(){
    //In other languages when we copy complex data structure, then similarly pointer gets copied -> Shallow Copy
    //If all the data also gets copied to a new location -> Deep copy

    //In rust we are just copying the pointer data so it sounds like shallow copy. But, since the first variable is also invalidated, we call this a move

    //IMPORTANT
    //Rust will never automatically create a deep copy. Therefore, any automatic copying can be assumed as inexpensive in terms of runtime perf.

    //If we do need deep copy of the heap data not just the stack data, we use a method called "clone".
    let s1 = String::from("Test");
    let s2 = s1.clone();//expensive, some code is executed here in the bg to do all the allocation and copy

    println!("s1 = {}, s2 = {}", s1, s2);//here s1 is also available
    //this is because s2 is not a reference to the same heap memory location, it's a new location. Hence, s1 is still independent
    //and not invalidated


    //then what happens with data of fixed size?
    let x = 4;
    let y = x;
    println!("{x}");
    //since we are not using something like clone here, how is x still available?
    //Because integers and similar data types have fixed known size at compile time and they are entirely stored on stack
    //so copies are quick to make. There is no reason for invalidating here.
    //In other words, there is no difference b/w deep and shallow copy here.
}
