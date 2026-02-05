fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs")
}
fn is_even(number: i32) -> bool {
    number % 2 == 0
}
fn alphabet(text: &str) -> (bool, bool) {
    (text.contains('a'), text.contains('z'))
}

fn main() {
    apply_to_jobs(21, "Rust Developer");
    println!("{}", is_even(8));
    println!("{}", is_even(9));

    println!("aardvark: {:?}", alphabet("aardvark"));
    println!("zoology:  {:?}", alphabet("zoology"));
    println!("zebra:    {:?}", alphabet("zebra"));
}
