#![allow(unused_variables)]

fn main() {
    let width: i32 = 4;
    let height: i32 = 7;
    let depth: i32 = 10;

    let area: i32 = area_of(width, height);

    println!("Area is {}", area);
    println!("Volume is {}", volume(width, height, depth));
}

fn volume(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}

fn area_of(x: i32, y: i32) -> i32 {
    return x * y;
}