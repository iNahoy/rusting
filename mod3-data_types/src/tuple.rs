pub fn executar() {
    println!("--- AULA: Tuple ---");

    let employee = ("Molly", 32, "Marketing");

    /*let name = employee.0;
    let age = employee.1;
    let department = employee.2;

    println!("{} {} {}", name, age, department);
    println!("{:#?}", employee);
    */

    let (name, age, department) = employee;
    println!("{} {} {}", name, age, department);
    println!("{:#?}", employee);
    println!();
}
