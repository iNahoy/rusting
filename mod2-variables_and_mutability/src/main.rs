fn main() {
    let apples: i32 = 50;
    let oranges: i32 = 14 + 6;
    let _fruits: i32 = apples + oranges;

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
}
