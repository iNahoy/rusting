#[allow(unused_variables)]
fn main() {
    let interger32: i32 = 1_337;
    let interger16: i16 = interger32 as i16;
    println!("{}", interger16);

    let pi: f32 = 3.141516;
    println!("{:.3}", pi);

    let with_milk = true;
    let with_sugar = false;

    let is_my_type_of_coffee = with_milk && with_sugar;
    let is_acceptable_coffee = with_milk || with_sugar;

    let interger8: [i8; 4] = [8, 21, 77, 94];
    dbg!(interger8);

    let tuple = (interger32, pi, is_acceptable_coffee, interger8);

    dbg!(tuple);
}
