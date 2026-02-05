pub fn executar() {
    println!("--- AULA: Character ---");
    let first_intial: char = 'B';
    let emoji = '😀';
    println!("{} {}", first_intial.is_alphabetic(), emoji.is_alphabetic());
    println!("{} {}", first_intial.is_uppercase(), emoji.is_uppercase());
    println!("{} {}", first_intial.is_lowercase(), emoji.is_lowercase());
    println!()
}
