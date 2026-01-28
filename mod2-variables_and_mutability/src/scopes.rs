pub fn executar() {
    println!("--- AULA: Scopes ---");

    /* scopes basicamente é a semântica do código
    ordem para que se permita e não se permita executar
    basicamente, a ordem dos fatores que altera o resultado*/

    let coffe_price: f64 = 5.99;
    println!("The price is {coffe_price}");
    {
        println!("The price is {coffe_price}");
        {
            let coffe_price: f64 = 9.99;
            println!("The price is {coffe_price}");
        }
        let cookie_price = 1.99;
        println!("{cookie_price}");
    }
    //println!("{cookie_price}"); comentado pois ocorre um erro de execução se for executado desta maneira
    println!("The price is {coffe_price}");
    println!();
}
