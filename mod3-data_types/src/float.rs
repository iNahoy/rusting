pub fn executar() {
    let pi: f64 = 3.141592653589793;
    println!("{pi}");
    println!("{pi:.4}");

    let pi: f32 = 3.141592653589793;

    println!("{}", pi.floor());
    println!("{}", pi.ceil());
    println!("{}", pi.round());
}
