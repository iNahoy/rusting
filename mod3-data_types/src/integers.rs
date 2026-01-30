#[allow(unused_variables)]

pub fn executar() {
    println!("--- AULA: Integers ---");

    let eight_bit: i8 = -112; //se fosse -210 estaria fora de range pelo tamanho do interger
    let eight_bit_unsigned: u8 = 255; // -15 estaria fora de range pois o minimio de um unsigned eh 0 e o maximo 255

    let sixteeen_bit: i16 = -32500;
    let sixteen_bit_unsigned: u16 = 64000;

    let thirty_two_bit_signed: i32 = -2147483648;
    let thirty_two_bit_unsigned: u32 = 4294967295;

    let some_value = 20i8; /*  passa a tratar o valor apartir somente do que vem depois do valor
    aqui a variavel passa a ser interger 8bits*/
}
