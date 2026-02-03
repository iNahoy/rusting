pub fn executar() {
    println!("--- AULA: Assignment Operators ---");
    let mut year: i32 = 2026;
    year = year + 1;
    println!("the new year is gonna be {}", year);

    year -= 1;
    println!("the current year is {}", year);

    year *= 2;
    println!("ano atual vezes 2 = {year}");

    year /= 2;
    println!("ano atual dividido por 2 = {year}");

    println!()
}
