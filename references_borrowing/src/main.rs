fn main() {
    println!("Getting to know references and borrowing");

    //in ownership, we saw that once the value is passed to a function, it needs to be returned back to use it further in the calling fn
    //this is not the most optimal and easy way for it

    //We can use reference -> temporary view of data
    //A ref is like a pointer in that it's an address we can follow to access the data stored at that address
    //that data is owned by some other variable
    //Unlike pointer, a ref is always guaranteed to point to a valid value of a type for the life of that reference.

    let s1 = String::from("Test 1");
    let len = cal_len(&s1);//this creates a reference that refers to the value of s1 but does not own it.
    //Because it does not own it, the VALUE it points to will not be dropped when the reference stops being used.
    //VALUE is only dropped when the ownership is transferred
    //  &s1 -> s1 -> data

    println!("The length of '{}' is {}", s1, len);
}

//These ampersands represent references and the allow us to refer to some value without taking ownership of it
fn cal_len(s: &String) -> usize {//s is a reference to a String
    s.len()
    //since we passed a refernence to the parameter and not transferred the ownership completely,
    //we don't need to return the value to give back the ownership, like pass by reference in Java

}//s goes out of scope but VALUE is not dropped as s never had the ownership of it

