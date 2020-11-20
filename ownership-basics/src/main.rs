fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        println!("{}", s); // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    //println!("{}", s); // not found in scope


    // Complex types for mutability and storing stuff on the heap
    // String types from string literals
    let s = String::from("hello");

    // Strings are mutable, string literals are not
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // Data on the heap has to be freed somehow
    // Rust does this automatically through scopes:
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }   // this scope is now over. s is no longer valid, and the memory is freed on the heap


    // This pushes two 5's onto the stack:
    let x = 5;
    let y = x;


    // This pushes two group of data consisting of (name, pointer, length and capacity) onto the stack
    // Both pointers in these groups of data point to the same String on the heap
    // (s2 = s1 just copies from the stack, not the heap)
    let s1 = String::from("hello");
    let s2 = s1;

    // Here's where Rust behaves differently. (!!!!!)
    // After the s2 = s1 line, s1 is no longer valid
    // "s1 was *moved* into s2"

    //println!("{}", s1); //NOT VALID
    println!("{}", s2); //VALID

    // This is done (partly?) to prevent *double free* errors where memory from a heap is freed twice

    // The clone method lets us deep copy heap data
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Types with a known size at compile time are stored entirely on the stack
    // This means we don't have to use clone (if they also have the Copy trait)
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);



}
