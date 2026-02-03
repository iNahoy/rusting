pub fn executar() {
    println!("--- AULA: Boolean Inversion ---");

    println!("{}", !true);
    println!("{}", !false);

    let age = 13;
    let can_see_rated_r_movie = age >= 18;
    println!("Can see? {}", can_see_rated_r_movie);
    let cant_see_rated_r_movie = !can_see_rated_r_movie;
    println!("Can't see? {}", cant_see_rated_r_movie);

    println!()
}
