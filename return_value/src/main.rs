// A return value is the output of a function
fn main() {
    let result = square(5);
    println!("The square of 5 is {result}");

    let result = square(7);
    println!("The square of 7 is {result}");

    let result = implicit_square(5);
    println!("The square of 5 is {result}");

    let result = implicit_square(7);
    println!("The square of 7 is {result}");
}
// Return Value
fn square(number: i32) -> i32 {
    return number * number;
}

// Implicit Return Value
fn implicit_square(number: i32) -> i32 {
    number * number
}