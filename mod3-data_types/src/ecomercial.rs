pub fn executar() {
    println!("--- AULA: && ---");

    let purchase_ticket = true;
    let plane_on_time = true;
    let making_event = purchase_ticket && plane_on_time;

    println!("It is {} that I will arrive as expected", making_event);

    println!()
}
