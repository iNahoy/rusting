pub fn executar() {
    println!("--- AULA: Imutabilidade (let vs let mut) ---");

    /*
     * let = imutável (padrão do Rust)
     * mut = mutável (permite reescrita)
     */

    // Caso 1: Variável Imutável
    let gymreps: i32 = 10;
    println!("I plan to do {gymreps} reps in this training (Imutável)");
    // gymreps = 15; // Isso daria erro de compilação!

    // Caso 2: Variável Mutável
    let mut gym_reps = 10;
    println!("I plan to do {gym_reps} reps in this training (Mutável - Início)");

    gym_reps = 32; // Aqui funciona porque usamos 'mut'
    println!("I plan to do {gym_reps} reps in this training (Mutável - Alterado)");
}
