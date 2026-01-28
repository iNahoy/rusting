fn main() {
    let apples: i32 = 50;
    let oranges: i32 = 14 + 6;
    let _fruits: i32 = apples + oranges; // escrito em _fruits pois não estaremos utilizando, é uma forma do próprio rust de não impacar com isso

    // primeira forma de interpolação
    println!("My garden has {} apples", apples);
    //segunda forma de interpolação
    println!("My garden has {apples} apples");

    //composta agora
    // primeira forma
    println!("My garden has {} apples and {} oranges.", apples, oranges);
    // segunda forma
    println!("My garden has {apples} apples and {oranges} oranges.");
    /* terceira forma, puxa a ordem da linguagem pra que funcione
     * 0 = apples
     * 1 = oranges
     */
    println!("My garden has {0} apples and {1} oranges.", apples, oranges);
    println!("My garden has {1} apples and {0} oranges.", apples, oranges);

    /*
     * quebra de código, apartir daqui veremos sobre imutabilidade do rust
     * let = imutavel
     * mut = mutavel
     * o que significa? os valores poderão ser reescritos
     */
    let gymreps: i32 = 10;
    println!("I plan to do {gymreps} reps in this training");
    // gymreps = 15; comentando pois dá erro na execução caso não seja executada

    let mut gym_reps = 10;
    println!("I plan to do {gym_reps} reps in this training");
    gym_reps = 32;
    println!("I plan to do {gym_reps} reps in this training");
}
