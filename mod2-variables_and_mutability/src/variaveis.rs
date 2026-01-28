pub fn executar() {
    println!("--- AULA: Variáveis e Interpolação ---");

    let apples: i32 = 50;
    let oranges: i32 = 14 + 6;
    let _fruits: i32 = apples + oranges; // O _ evita o aviso de "variável não usada"

    // 1. Primeira forma de interpolação
    println!("My garden has {} apples", apples);

    // 2. Segunda forma de interpolação (mais moderna)
    println!("My garden has {apples} apples");

    // 3. Composta
    println!("My garden has {} apples and {} oranges.", apples, oranges);
    println!("My garden has {apples} apples and {oranges} oranges.");

    // 4. Posicional (0 = apples, 1 = oranges)
    println!("My garden has {0} apples and {1} oranges.", apples, oranges);
    println!("My garden has {1} apples and {0} oranges.", apples, oranges);

    println!(); // Pula uma linha no final pra ficar bonito
}
