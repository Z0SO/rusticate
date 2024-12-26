fn saludar(mut a: String) -> String {
    println!("valor x original: {}", a);
    a = String::from("pasaje por propiedad");
    a // return implicito
}

fn saludar2(a: &mut String) {
    println!("valor x original: {}", a);
    *a = String::from("pasaje por referencia");
}

pub fn desafio2() {
    let mut x = String::from("pasaje");

    // Pasaje por propiedad
    // no se puede hacer saludar(x) porque despues la variable x no es accesible en desafio2

    let result = saludar(x.clone()); // Clonamos `x` para que no pierda su valor original.
    println!("Pasaje por propiedad: {}", result);

    // Pasaje por referencia mutable
    saludar2(&mut x);
    println!("Pasaje por referencia: {}", x);
}
