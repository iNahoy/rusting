pub fn executar() {
    println!("--- AULA: Or || ---");
    /*
     *  true || false - valida um true a depender de somente um true em comparação
     */

    let user_has_paid = true;
    let user_is_admin = true;
    let user_can_see_premium = user_has_paid || user_is_admin;
    println!("{user_can_see_premium}");

    println!()
}
