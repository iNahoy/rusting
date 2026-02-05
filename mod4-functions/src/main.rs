fn open_store(neighbourhood: &str) {
    println!("Opening my pizza store!\nIn {neighbourhood}!")
}
fn closing_store() {
    println!("Closing my pizza store!")
}

fn main() {
    open_store("Brooklyn");
    bake_pizza(8, "pepperoni");
    open_store("Amaraji");
    bake_pizza(21, "sinistras");
    open_store("Itaquera");
    closing_store();

    let result = square(2);
    println!("{}", result)
}
fn bake_pizza(number: u8, topping: &str) {
    println!("Baking {} {} pizzas!", number, topping)
}

fn square(num: u8) -> u8 {
    return num * num;
}
