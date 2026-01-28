pub fn executar() {
    /*shadowing é basicamente, sombrear as variáveis com outros tipos
    aqui temos exemplo de uma variavel que pode se transformar em outras e ainda terá o mesmo resultado
    */
    let grams_of_protein: &str = "100.345";
    let grams_of_protein: f64 = 100.345;
    let grams_of_protein: i32 = 100;
}
