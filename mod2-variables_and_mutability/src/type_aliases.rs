type Meters = i32;
/* type aliases é o mesmo que descrever um tipo especifico para formatação
aqui utilizamos metros para ser equivalente ao interger, é útil para uma informação dita melhor*/
#[allow(unused_variables)]
pub fn executar() {
    println!("--- AULA: Type Aliases ---");

    // pouco de uso de compiler directives
    /*aqui tamo brincando um pouco com variaveis que não são utilizadas
     basicamente, podemos usar isso nas linhas que na seguinte terão variáveis que não são utilizadas ou
    colocar algo como coloquei antes da função */
    // #[allow(unused_variables)]
    let variavelnaousada = "only a test";
    // #[allow(unused_variables)]
    let variavelnaousada = "hahaha";

    let mile_race: Meters = 1600;
    let two_mile_race: Meters = 3200;
    println!(
        "A one mile race is {} meters long and a two mile race is {}",
        mile_race, two_mile_race
    );

    println!()
}
