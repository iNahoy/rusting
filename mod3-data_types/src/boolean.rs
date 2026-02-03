pub fn executar() {
    println!("--- AULA: Booleans ---");
    let e_bonito: bool = true;
    let e_bobo: bool = false;

    println!("Bonito {} Bobo {}", e_bonito, e_bobo);

    let age: i32 = 21;
    let is_young = age > 30;
    println!("{is_young}");

    let is_young = age < 30;
    println!("{is_young}");

    println!("{} {}", age.is_positive(), age.is_negative());
    println!()
}
