pub fn executar() {
    println!("--- AULA: Methods ---");

    let value: i32 = -15;
    println!("{}", value.abs()); //metodo é basicamente variavel.algumacoisa() para ser executado, como lenght e etc

    let empty_space = "       my content       ";
    println!("{}", empty_space.trim()); // trim corta os espaços adicionais

    println!("{}", value.pow(2));
    println!("{}", value.pow(3));
    println!("{}", value.pow(4));

    println!()
}
