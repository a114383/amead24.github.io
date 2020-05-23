fn main() {
    println!("boolean values: true & false");
    println!("and char types use single quotes instead of double");

    // always immutable
    const MAX_VALUE: u32 = 100_000; // same as 100,000
    println!("constant value is {}", MAX_VALUE);
   
    // mutations - not same as const
    // you can overwrite with 'mut' but cant change type
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // shadowing - can overwrite non-mut types
    let y = 5;
    let y = y +5;
    let y = y * y;
    println!("Orignally y was 5, now it is {}", y);

    // shadowing can also help solve dynamic typing problems
    // let mut spaces = " ";
    // spaces = spaces.len();     // cnnot mutate type
    let spaces = " ";          
    let spaces = spaces.len();    // but you can reassign the variable to new type

    // Typing - For the most part okay, but not everything can be infered
    // let value = "42".parse().expect("Not a number");      // non-inferable
    let value: u32 = "42".parse().expect("Not a number");    // compiler is happy

    // containers - arrays and tuples can hold multiple different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z)= tup; //unpacking or 'destructing'
    println!("The value of (x, y, z) is {} {} {}", x, y, z);
    println!("Or accessing tuple with dot notation tup.0, tup.1, tup.2 == {} {} {}",
             tup.0, tup.1, tup.2);

    // arrays - every element must have the same type
    // they also enforce a static size, once initialized they cannot grow/shrink
    let arr = [1, 2, 3, 4, 5]; // on stack rather than heap
    println!("Arrays accesed using bracket indexing.  arr[2]=={}", arr[2]);

    // functions
    let num: u32 = 24;
    print_some_number(num);

    // or evaluate expressions out of scope
    // let x = let y = 6 // doesn't work
    let j = {
        let k = 5;
        k + 1
    };
    println!("out of scope example, x=={}", j);

    // by not adding the semi colon the scope operator keeps the
    // code as a valid 'expression' rather than a usual 'statement'
    let f = five();
    println!("f before add one: {}", f);
    let f = add_one(f);
    println!("f after add one: {}", f);

    // control flow
    // if statements
    let mut number = 3;

    if number < 5 {
        println!("Usual greater than or less than symbols, not parenthesis");
    } else if number == 5{
        println!("if, else if, else...");
    } else {
        println!("and if number not valid, Rust won't convert implicitly.")
    }

    // combining the expression concept you can make ternary operators
    // but they must keep the same type across all conditions
    let example = if true { 5 } else { 6 };

    // loop, while, for
    loop {
        println!("looping again, and again, and again, and ...");
        break;
    }

    while(number != 5){
        println!("while loop incrementor at {}", number);
        number = number + 1;
    }

    let arry = [1, 2, 3, 4, 5];
    for element in arry.iter() {
        println!("the value is : {} ", element);
    }

    // or using the range operator
    for i in (1..5).rev() { //.rev() for reverse
        println!("lift in {}", i);
    }
}


fn print_some_number(num: u32){
    println!("The answer is {}", num);
}


fn five() -> i32 {
    5
}


fn add_one(a: i32) -> i32 {
    a + 1
}
