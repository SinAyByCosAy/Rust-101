fn main() {
    println!("Getting to know references and borrowing");

    //in ownership, we saw that once the value is passed to a function, it needs to be returned back to use it further in the calling fn
    //this is not the most optimal and easy way for it

    //We can use reference -> temporary view of data, This is called BORROWING

    //A ref is like a pointer in that it's an address we can follow to access the data stored at that address
    //that data is owned by some other variable
    //Unlike pointer, a ref is always guaranteed to point to a valid value of a type for the life of that reference.

    //Reference scope : Starts from where it is introduced and continues through the last time it is used.
    
    let s1 = String::from("Test 1");
    let len = cal_len(&s1);//this creates a reference that refers to the value of s1 but does not own it.
    //Because it does not own it, the VALUE it points to will not be dropped when the reference stops being used.
    //VALUE is only dropped when the ownership is transferred
    //  &s1 -> s1 -> data

    println!("The length of '{}' is {}", s1, len);
    immutable_modify(&s1);

    //For Mutable reference:
    let mut s = String::from("Hey ");
    mutable_modify(&mut s);
    println!("{s}");

    //Resreiction with mutable references -> There can't be multiple mutable references to a value at the same time IMPORTANT
    //We can't have mutable and immutable reference to a value at the same time
    //as users of an immutable reference don't expect the value to suddbenly change out from under them
    //Multiple immutable reference are okay!!

    //NO DATA RACES at compile time
    let st = &mut s;
    //let st1 = &mut s; // -> this line will throw an error
    
    //the first reference has to be used before we can create another one. In between the creation of the mutable reference and it's usage
    //we can't have another reference
    println!("{st}");
}

//These ampersands represent references and the allow us to refer to some value without taking ownership of it
fn cal_len(s: &String) -> usize {//s is a reference to a String
    s.len()
    //since we passed a refernence to the parameter and not transferred the ownership completely,
    //we don't need to return the value to give back the ownership, like pass by reference in Java

}//s goes out of scope but VALUE is not dropped as s never had the ownership of it

fn immutable_modify(s: &String) {
    //s.push_str("dummy"); -> This throws an error as references are also immutable by default

    //we are not allowed to  modify something we have a reference to
}

fn mutable_modify(s: &mut String) {
    s.push_str("Baby!");
}