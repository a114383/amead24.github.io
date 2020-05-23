fn main() {

    // how are the basic data types used on a stack?

    // moving onto heaps - Strings
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    // there is no need to deallocate memory here as rust will `drop` the
    // memory once the string goes out of scope


    // Note to self, assignment points to the value of x
    // instead of the pointer to the value
    let x = 5;
    let y = x;
    println!("x = {}, and y = {}", x, y);
    let x = x + 1;
    println!("x = {}, and y = {}", x, y);

    // String = {ptr: *ptr->char[], len: int, capacity: int}
    // Note: setting s2 equal to s1 makes both point same char[] on heap
    // Problem1: one string editing the heap changes the value of the other pointer.
    // Problem2: double free - one string going out of scope tries to drop a working variable
    let s1 = String::from("hello");
    let s2 = s1;

    // the solution then is to `move` the variable to a new owner
    // println!("{}, world!", s1); // thus s1 doesn't own the data anymore
    
    // Rust will never deep copy, so every automatic copy is the cheapest possible
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world!", s1);

    let s3 = String::from("hello");
    takes_ownership(s3);
    // println!("{}, world!", s3); // s3 not valid as it was `moved` and `dropped`
    // versus an expression would return the ownership
    let s4 = String::from("hello");
    let s4 = take_and_giveback(s4);
    println!("{}, world!", s4);

    // You can avoid passing ownership by using the reference operator (&)
    // Passing by reference is called `borrowing`
    let s5 = String::from("hello");
    let len = get_length(&s5);
    println!("{}, world!", s5);

    let s6 = String::from("hello");
    let s7 = &s6;
    // let s8 = &mut s6; // cannot have references to previously immutable types
    // let s9 = &mut s6; // or multiple mutable references in same scope, data race

    // avoid reference danglers
    let s0 = dangler();

    // and lastly slices
    let hw = String::from("Hello World!");
    let first_space = slice_example(&hw); // returns 5

    // but when releasing hw, first space is useless
    // so instead define the variables in reference to hw
    let hello = &hw[0..5];   
    let world = &hw[5..];
    
    let len = hw.len();
    let hello_world = &hw[0..len];

    // see first_word() for returning a slice of the original
    let mut s = String::from("hello world");
    let word = first_word(&s);
    // s.clear(); // Error
    // you cannot clear s because word is an immutable reference
    // to the value pointed to by s
    

    // String literals vs. Strings
    let literal = "Hello world!"; // type(s) == &str
    let str_slice = String::from("Hello world!");

    let s_improved = first_word2(&str_slice[..]); // turning slice into str literal 
    let s_improved = first_word2(literal);        // string literals are slices
}


fn takes_ownership(some_string: String){
    println!("Inside takes_ownership() - {}.", some_string);
}


fn take_and_giveback(some_string: String) -> String {
    some_string
}


fn get_length(some_string: &String) -> usize {
    // but you cannot modify a value passed by reference
    // some_string.push_string(", world!"); // cannot borrow as mutable
    //
    // unless you specify the reference as mutable as well
    // get_length(some_string: &mut String)
    some_string.len() // some_string goes out of scope, but not dropped 
}

fn dangler() -> String {
    // if the return type was &String
    let s = String::from("hello");
    // trying to return &s
    // causes s to be dropped, and thus the reference doesn't exist
    // so just return the string itself to transfer ownership
    s
}


fn slice_example(s: &String) -> usize {
    // here we'll return the position of the word
    let bytes = s.as_bytes();

    // converting bytes to iterator. while
    // enum wraps each element in a tuple of index, item
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { return i; }
    }

    // returning len if a space not found
    s.len()
} 


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' { return &s[0..i]; }
    }

    &s[..]
}


fn first_word2(s: &str) -> &str {
    // writing the function to take a string literal
    // allows passing both String and str
    // passing a String would be a slice of the entire thing
    // while passing the reference would be a str literal
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' { return &s[0..i]; }
    }

    &s[..]
}
