fn main() {
    println!("Hello, world!");
    
    let x = 5;
    println!("The value of x is: {}", x);
    
    let result = add(3, 7);
    println!("The sum is: {}", result);
    
    for i in 1..=5 {
        println!("Loop iteration: {}", i);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}