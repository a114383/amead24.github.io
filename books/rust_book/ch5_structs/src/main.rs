
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn build_user(email: String, username: String) -> User{
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


// comparing structs vs. tuples
// don't have field names, but allows dynamic types across elements
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


fn main() {
    println!("Instance of struct User.");
    let user1 = User {
        email: String::from("example@example.com"),
        username: String::from("example_uname"),
        sign_in_count: 0,
        active: true,
    };
    println!("Instance username: {}", user1.username);

    // If you assign the struct as mutable you can reassign with accesors
    let mut user2 = User {
        email: String::from("example@example.com"),
        username: String::from("example_uname"),
        sign_in_count: 0,
        active: true,
    };
    println!("Original username: {}", user2.username);
    user2.username = String::from("different_name");
    println!("Changing with assignment: {}", user2.username);

    // using the .. syntax you can carry fields onwards
    let user3 = User{
        email: String::from("test3@example.com"),
        username: String::from("uname_test3"),
        ..user1
    };
    println!("get .active from other struct: {}", user3.active);

    // Example of accessing tuple elements
    let black = Color(0,0,0);
    println!("values of black: {}, {}, {}", black.1, black.1, black.2);

    tuple_example();   

    struct_example();

    format_example();

    method_example();
}


fn tuple_example(){
    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    
    let rect = (3, 5);
    println!("The area of the rect is {} pixels.",
             area(rect)
    );
}


fn struct_example(){
    // borrowing allows the higher stack to keep the object
    // this way we don't have to worry about losing rect after passing
    // it to the area function
    
    struct Rectangle{
        width: u32,
        height: u32,
    }

    fn area(rectangle: &Rectangle) -> u32{
        rectangle.width * rectangle.height
    }

    let rect = Rectangle{ width: 3, height: 5 };
    println!("The area of the rect is {} pixels.",
             area(&rect)
    ); 
}


fn format_example(){
    // traits to be used with the derive annoation
    // that allow you to tailor custom types
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle{ height: 5, width: 10};
    println!("rect is {:#?}", rect);
}


fn method_example(){
    #[derive(Debug)]
    struct Rectangle{
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect = Rectangle{ height: 5, width: 8 };
    println!("Area == {}", rect.area());
}
