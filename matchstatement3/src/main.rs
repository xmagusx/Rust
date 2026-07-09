fn main() {
    let number = 8;

    // The match option with multiple values and conditions
    match number {
        even if even % 2 == 0 => print!("{even} is an even number"),
        odd if odd % 2 != 0 => print!("{odd} is an odd number"),
        _ => print!("Unknown"),
    }
}
