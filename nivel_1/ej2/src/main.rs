// Declarar variables inmutables y mutables.
// Realizar operaciones aritméticas básicas.
// Imprimir el valor de las variables.

fn main() {

    let mut x = 5;
    let y = 10;

    print!("Resultado de variable x + y = {} \n", x + y);
    x = 10;
    print!("Resultado de variable x + y = {} \n", x + y);


    // --- variables explicitas

    let j: u32 = 10;    // unsigned 32 bits
    let k: f64 = 10.50;  // float 64 bits

    print!("Resultado de variable j + k = {} \n", j as f64 + k);
}
