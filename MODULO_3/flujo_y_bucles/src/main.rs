fn main() {
    let number = 3;

    if number < 5 {
        println!("La condición era verdadera");
    } else {
        println!("La condición era falsa");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("El número es divisible por 4");
    } else if number % 3 == 0 {
        println!("El número es divisible por 3");
    } else if number % 2 == 0 {
        println!("El número es divisible por 2");
    } else {
        println!("El número no es divisible por 4, 3 ni 2");
    }

    // Como if es una expresión en Rust, puedes usarlo en la declaración let para asignar valores.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("El valor de number es: {number}");

    let mut cond_var = 0;

    loop {
        println!("¡De nuevo por {cond_var} vez!");
        cond_var += 1;

        if cond_var == 320 {
            break;
        }
    }

    // bucle while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("¡Despegue!");

    // recorrido de una matriz
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("El valor es: {element}");
    }
}
