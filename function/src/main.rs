// main fuction
fn main() {
    open_store(/* setting argument */ "Brooklyn");
    open_store(/* setting argument */ "Queens");
    bake_pizza(/* setting two arguments */ 20, "pepperoni");
    bake_pizza(/* setting two arguments */ 15, "mushroom");
    swim_in_profit();
    
}
// function open_store
fn open_store(/* setting parameter */ neighborhood: &str) {
    println!("Opening my pizza store in {neighborhood}");
}
// function bake_pizza
fn bake_pizza(/* setting two parameters */ number: i32, topping: &str) {
    println!("Baking {number} {topping} pizza");
}
// function swim_in_profit
fn swim_in_profit() {
    println!("So much $$$, so little time");
}