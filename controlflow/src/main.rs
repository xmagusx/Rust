// Control Flow refers to how a program will execute (the "flow" of code)
fn main() {

    seasons();

    let _some_condition = true;

    if _some_condition {
        println!("This line will be output");
    }

    if false {
        println!("This line will not be output");
    }
}

fn seasons () {
    let season = "summer";

    if season == "summer" {
        println!("School's out!");
    } 
    
    else if season == "winter" {
        println!("So cold!");
    }

    else if season =="fall" {
        println!("Leaves falling");
    }

    else if season == "spring" {
        println!("Lots of rain!");
    }
    /* "else" here works like a fallbak option.
    If nothing will be executed before, this "else" will respond.
    */
    else {
        println!("I don't know what season is!")
    }
}