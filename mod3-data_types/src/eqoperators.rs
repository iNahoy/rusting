pub fn executar() {
    println!("--- AULA: Equality Operators ---");

    /*  1 == 2 equal
        1 != 2 not equal
    */

    println!("{}", "Coke" == "Pepsi");
    println!("{}", "Coke" != "Pepsi");
    println!("{}", "Coke" == "coke");
    println!("{}", "Coke" == "Coke ");
    println!("{}", "Coke" == "Coke");

    println!("{}", 13 != 13);
    println!("{}", 26.14 == 26.1);
    println!("{}", 13 != 13.0 as i32);

    println!("{}", true == true);
    println!("{}", true != false);
    println!();
}
