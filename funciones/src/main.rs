// fn another_function() {
//     println!("Another function.");
// }

// Declaraciones y expresiones
fn main() {
    let y = {
        let x = 3; // tiene ; asi que esto es una declaracion
        x + 1 // esto no tiene ; asi que esto es una expresion y es valor de retorno
    };

    println!("The value of y is: {y}");

    // parametros
    another_function(5);

    fn another_function(x: i32) {
        println!("The value of x is: {x}");
    }

    // valores de retorno en las funciones
    fn five() -> i32 {
        5
    }

    let x = five();
    println!("The value of x is: {x}");
}


fn plus_one(x: i32) -> i32 {
    x + 1; // ¡Error! Esto devuelve `()`, no `i32`
}