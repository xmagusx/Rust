// The match statement can react to all possible variants of a value.
fn main() {
    let _evaluation = true;

    let _value = match _evaluation {
        // A pattern or arm is one possible option to compare the match value against.
        true => 20,
        false => 40,
    };

    println!("The value for evaluation is {_value}");
}
