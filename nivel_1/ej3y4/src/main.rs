
// Control de flujo -> dado un numero ingresado por el usuario verificar si es par o impar
// Rust tiene potentes herramientas para el control de flujo, incluyendo condicionales (if, else) y bucles (for, while, loop). Este ejercicio te ayudará a practicar estas estructuras.
// 🎯 Objetivo
//     Usar condicionales para tomar decisiones.
//     Escribir bucles para iterar sobre valores.
//     Comprender el uso de la palabra clave break para salir de un bucle.
use std::io;

fn verif_par_impar(param_num:i32)
{
    // para saber el resto de una división se usa el operador %.
    if param_num % 2 == 0
    {
        println!("El numero {param_num} es par");
    }else{
        println!("El numero {param_num} es impar");
    }
}



fn numeros_primos(param_num:i32)
{
// #### **Desafío 1: Números Primos**
//     - Escribe un programa que determine si un número dado es primo. 
//         - Pista: Un número primo solo es divisible por 1 y por sí mismo. Usa bucles y condicionales para verificarlo.


    let mut band = true;
    for i in 2..param_num
    {
        if param_num % i == 0
        {
            band = false;
            println!("El numero {param_num} no es primo");
            break;
        }
    }
    if band
    {
        println!("El numero {param_num} es primo");
    }


}




fn main() 
{
    let mut un_numero = String::new(); // variable mutable de tipo string
    
    println!("Ingrese un número: ");
    io::stdin()
        .read_line(&mut un_numero)
        .expect("Error al leer la entrada");
    
    // Convertir el string a un número entero
    let un_numero:i32 = un_numero.trim()
        .parse()
        .expect("Por favor ingrese un número entero");

    verif_par_impar(un_numero);


    // Condicionales
    let numero = 7;
    if numero % 2 == 0 {
        println!("El número {} es par.", numero);
    } else {
        println!("El número {} es impar.", numero);
    }

    // Bucle 'for'
    println!("Imprimiendo números del 1 al 5:");
    for i in 1..=5 {
        println!("{}", i);
    }

    // Bucle 'while'
    let mut contador = 3;
    println!("Cuenta regresiva:");
    while contador > 0 {
        println!("{}", contador);
        contador -= 1;
    }

    // Bucle infinito con 'break'
    // let mut suma = 0;
    // loop {
    //     println!("Suma: {}", suma);
    //     suma += 1;
    //     if suma == -1 {
    //         println!("Suma alcanzó el valor {}.", suma);
    //         break;
    //     }
    // }

    println!("verificando si el numero es primo");
    numeros_primos(un_numero);
}

