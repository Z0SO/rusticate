// #### **Desafío 5: Palíndromo**
// - Determina si una palabra o frase es un palíndromo (se lee igual de izquierda a derecha que de derecha a izquierda).
//     - Pista: Ignora los espacios y las mayúsculas. Usa manipulación de cadenas.
use std::io;

fn invertir(param: String) -> bool
{
    let mut palabra = param.to_lowercase();
    palabra = palabra.replace(" ", "");
    let mut palabra_invertida = String::new();
    for letra in palabra.chars().rev()
    {
        palabra_invertida.push(letra);
    }
    // print!("palabra normal: {} -- palabra invertida {}", palabra, palabra_invertida);
    return palabra.trim() == palabra_invertida.trim();
}



fn main()
{
    let mut palabra = String::new();
    std::io::stdin()
        .read_line(&mut palabra)
        .expect("Error al leer la palabra");
    if invertir(palabra)
    {
        println!("Es un palíndromo");
    }
    else
    {
        println!("No es un palíndromo");
    }
}

