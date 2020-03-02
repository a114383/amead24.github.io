# Networks

## OSI Model (internet protocol standard)

Application  (application)
Presentation (application)
Session      (transport)
Transport    (transport)
Network      (internet)
Data Link    (network access)
Physical     (network access)


## Addressing Networks
mac addresses, 48 bit long unique id
  six groups of two hex digits
  can be edited by the os system

ip address
  cidr - 192.168.1.1/26 
    where 26 signifies the number of 1's in the network mask
    2^32-26 = 2^6 = 64 possible ip addresses
  assigned by DHCP in a home network


## How DNS Works
dns server maintains mapping of human readable to ip address
dns server is /etc/resolv.conf
  this usually just points to the local dns server
  which then has prepopulated results, or knows who in the globe to ask


## Connection Models
connection-oriented - where a virtual connection is negotiated before sending data.  That negotiation includes determining connection parameters.  This is what TCP is today, it consists of headers and data section.

connectionless - messages bear no relation to one another, example of UDP where there's no guarantee of sequence or reliability.


# Rust

## Ownership

exactly one owner at any point in time, others either have to borrow or copy that resource.  anything not owned will be dropped.

1. both resources are created on the stack
2. integer stored on the stack,


## Reference/Borrowing

```
example(&value) - passing the reference, no ownership change
. . .
fn example(&s)
```


## Lifetimes

conditions:
two inputs should have the same lifetimes
return values should have same lifetime as that of input

special lifetimes - {static, fn}


## Generics

```rust
struct Tuple<T> {
    first: T,
    second: T,
}

fn sum<T>(tuple: Tuple<T>) -> T { tuple.first + tuple.second }

fn main() {
    let tuple_u32: Tuple<u32> = Tuple { first: 5u32, second: 6u32 };

    // The trait sum might not be implemented for all possible
    // types that can be supplied to the Tuple struct
    println!("{}", sum(tuple_u32));
}

// Solution - Constrain T to types which have implemented Add
use std::ops::Add;
fn sum<T: Add<Output = T>>(tuple: Tuple<T>) -> T { tuple.first + tuple.second }
```

## Traits

A trait is a language feature that tells the Rust compiler about functionality a type must provide.

```rust
trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

// define any arbitrary structs here
// but they must implement the trait HasArea
// else there be an error when trying to pass to print_area()

fn print_area<T: HasArea (+ X number of traits)>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}
```

You can also use traits to extend built in types:
```rust
trait Sawtooth { fn sawtooth(&self) -> Self; }
impl Sawtooth for f64 { ... }


// You can let the compiler create these using macros
#[derive(Display)]
struct Animal<T> { ... }

// Or you can implement std lib traits for your type
use::fmt::Display;
struct Animal<T> { ... }
impl<T> fmt::Display for Animal<T> where T: Display {}

```

## Errors 

When using `enum Object` returning the result (`.unwrap()`) only works once you've checked for the error case

or you can use the `Option` type:
```rust
fn divide(num: u32, den: u32) -> Option<u32> { if den == 0u32 { None } else { Some(num/den) }}
// This will fail
fn main() { println!("{:?}", divide(10, 0))}
fn main() { 
    let result = divide(10, 0);
    match result {
        None => println!("error"),
        Some(result) => println!("result == {}", result),
    }
}
```

The question mark (?) is a new operator that allows for cleaner error handling, for example this original code can be replaced with:
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    // Notice that file open returns a Result object
    // pub fn open<P: AsRef<Path>>(path: P) -> Result<File>
    let f = File::open("username.txt");

    // Therefore we must check the return type, and if it's valid
    // overwrite the original variable name "f", else raise error
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // Initializing an empty string, and then letting the file object
    // borrow it to update it's value it needs to be mutable
    // pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String>
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }

    // last object is returned by default, so we return a Result
}
```

can be replaced with:
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}
```


## Macros

1. Expanded to source code at compilation time.
2. Ends in !
3. Two versions, syntactic macros and newer
   1. `macro_rules! some_macro { ... }`
   2. Using the AST instead of compiler you can have more advanced code generation


## Closures

Think of them like lambda functions:
```rust
let plus_one = |x: i32| x + 1;
assert_eq!(2, plus_one(1));
```


## Concurrency

```rust
use std::thread;
fn main(){
    for i in 1..10 {
        let handle = thread::spawn(move || {  //Give it a closure
            println!("hello from thread {}", i);
        });
        let _ = handle.join();
    }
}


## Iterators

Iterators are lazy, meaning it does nothing until it's actually used.
```rust
let v1 = vec![1,2,3];
let v1_iter = v1.iter();   // nothing has been run
for val in v1_iter {. . .} // now n executions
```

If you implement your own iterator, you need to handle the borrowing yourself, and return an `Item` wrapped in `Some<Self::Item>` and `None` when the iteration is over.

```rust
// To understand the lazyness
let v1 = vec![1,2,3];
let sum = v1.iter().map(|x| + 1);           // doesn't compile
let sum = v1.iter().map(|x| + 1).collect(); // compiles
```

Read: https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html
```rust
// trying to flatten a 2d array:
let matrix: Vec<Vec<i32>> = vec![vec![1,0,1], vec![1,1,0], vec![1,1,0]];

// What's the difference?
let flatten: Vec<i32> = matrix.iter().flatten().collect(); // error
// a value of type `std::vec::Vec<i32>` cannot be built from an iterator over elements of type `&i32`

let flatten: Vec<i32> = matrix.into_iter().flatten().collect(); // works
```

`iter()` - "It is important to note that this Iter<'a, T> type only has a reference to T. This means that calling v.iter() will create a struct that borrows from v. Use the iter() function if you want to iterate over the values by reference."

vs. 

`into_iter()` - "Use the into_iter() function when you want to move, instead of borrow, your value. The .into_iter() function creates a IntoIter<T> type that now has ownership of the original value."

