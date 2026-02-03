pub fn executar() {
    let addition = 5 + 4;
    let subtraction = 10 - 6;
    let multiplication = 3 * 4;

    println!("+: {}\n-: {}\n*: {}", addition, subtraction, multiplication);

    let floor_division: i32 = 5 / 3;
    println!("{floor_division}");

    let decima_division: f64 = 5.0 / 3.0;
    println!("{decima_division}");

    let remainder: i32 = 8 % 2;
    println!("{remainder}");
}
