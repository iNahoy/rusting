pub fn executar() {
    println!("--- AULA: Generics ---");

    let month_days: std::ops::Range<u8> = 1..31;

    for day in month_days {
        println!("{}", day)
    }

    let letters: std::ops::RangeInclusive<char> = 'a'..='f';

    for letter in letters {
        println!("{letter}")
    }

    println!();
}
