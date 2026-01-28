pub fn executar() {
    /* shadowing é basicamente, sombrear as variáveis com outros tipos
       aqui temos exemplo de uma variavel que pode se transformar em outras
       e ainda terá o mesmo resultado (o nome da variável)
    */

    // 1. Começa como String slice (&str)
    let grams_of_protein: &str = "100.345";
    println!("{grams_of_protein}");

    // 2. Shadowing para Float (f64)
    let grams_of_protein: f64 = 100.345;
    println!("{grams_of_protein}"); // CORRIGIDO AQUI (Parêntese fora das aspas)

    // 3. Shadowing para Inteiro (i32)
    let grams_of_protein: i32 = 100;
    println!("{grams_of_protein}");

    // 4. Shadowing de novo (para mudar valor mantendo tipo)
    // ATENÇÃO: Se você tirar o 'let' daqui, vai dar erro de imutabilidade!
    let grams_of_protein = 103;
    println!("Gramas de proteina: {} ", grams_of_protein);
}
