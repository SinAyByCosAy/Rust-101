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

}//Rust automatically calls drop at closing bracket where the of String can put the code to return the memory
