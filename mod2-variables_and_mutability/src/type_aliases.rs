type Meters = i32;
/* type aliases é o mesmo que descrever um tipo especifico para formatação
aqui utilizamos metros para ser equivalente ao interger, é útil para uma informação dita melhor*/
pub fn executar() {
    println!("--- AULA: Type Aliases ---");

    let mile_race: Meters = 1600;
    let two_mile_race: Meters = 3200;
    println!(
        "A one mile race is {} meters long and a two mile race is {}",
        mile_race, two_mile_race
    );

    println!()
}
