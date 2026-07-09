fn main() {
    let season = "winter";

    if season == "summer" {
        println!("School's out!");
    } 
    else if season == "winter"{
        println!("It's cold!");
    }
    else {
        println!("Lots of rain!");
    }

    // Using the "match" option is it possible to create the same situation above
    match season {
        "summer" => println!("School's out!"),
        "winter" => println!("It's cold!"),
        _ => println!("Lots of rain!"),
    }
}
