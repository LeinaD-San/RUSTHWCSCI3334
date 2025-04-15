/*
Assignment
Write a closure named operation that multiplies two integers and returns the result. Test it with 10 * 5 and print the result.
*/
fn main() {
    let operation = |a: i32, b: i32| {
        return a * b;
    };

    println!("Result: {}", operation(10, 5));
}

