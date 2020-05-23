use std::io;

fn main() {

    let mut nth = String::new();
    
    println!("Enter the nth number of fib sequence you want: ");
    io::stdin().read_line(&mut nth)
        .expect("Bad input.");

    let nth: u64 = match nth.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("not a number"),
    };
    
    let num = calculate_fib(nth);
    println!("{}th fib is: {}", nth, num);

    let rec = rec_fib(nth);
    println!("{}nth fib is: {}", nth, rec);
}


fn calculate_fib(nth: u64) -> u64{
    let mut count = 1;
    let mut prev1 = 0;
    let mut prev2 = 1;

    let mut n = 1;
    while n!=nth {
        count = prev1 + prev2;
        prev1 = prev2;
        prev2 = count;

        n = n + 1;
    }

    return count;
}

fn rec_fib(nth: u64) -> u64 {
    if(nth<=1){ return nth; }
    return rec_fib(nth-1)+rec_fib(nth-2);
}
