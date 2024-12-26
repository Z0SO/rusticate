
// #### **Desafío 4: Inversión de Cadenas**
// - Escribe un programa que tome una cadena como entrada y devuelva la cadena invertida.
// - Pista: Usa bucles o métodos específicos de las cadenas.

// ---
//

use std::io;

fn invertir(param: &mut String) -> String 
{
    return param.chars().rev().collect();
}

fn main()
{
    let mut input=String::new();

    println!("Ingresar Cadena");
    io::stdin().read_line(&mut input).expect("error");
    
    let res = invertir(&mut input);
    
    println!("String invertido {res}");
}
