pub fn executar() {
    println!("--- AULA: Array ---");

    let number: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];
    let apples = ["Granny Smith", "McIntosh", "Red Delicious"];

    println!("{:#?}", number);
    println!("Length {}", apples.len());

    let mut seasons = ["Spring", "Summer", "Fall", "Winter"];

    let first = seasons[0];
    let second = seasons[1];
    let third = seasons[2];
    let fourth = seasons[3];

    println!("1 - {} 2 - {} 3 - {} 4 - {}", first, second, third, fourth);
    println!("{}", seasons[3]);

    seasons[2] = "Autumn";
    println!("{:?}", seasons);
    println!();

    println!("--- AULA: DBG! ---");

    dbg!(2 + 2);
    dbg!(seasons);

    println!()
}
