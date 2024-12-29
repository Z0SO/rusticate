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

    
    //En Rust, el scope de una variable está definido por el bloque de código ({ ... }) en el que se declara.
    // Cuando algo "sale de alcance" (goes out of scope):
        // Se destruye automáticamente: En Rust, el sistema libera los recursos asociados al valor (como la memoria) gracias al mecanismo de ownership y a los destructores que se llaman automáticamente.
        // No puede ser usado más: Una vez que una variable está fuera de alcance, intentar usarla dará un error de compilación.
        
    // let result = saludar(x); 
    // esto hace que cuando termine el bloque de codigo de la funcion, x se vaya de alcance, 
    // por lo que se llama su metodo destructor y ya no existe

// Cualquier cosa delimitada por { ... } puede ser un bloque, y define su propio alcance. Esto incluye:
//
// Funciones
// Bloques explícitos
// Condicionales (if, else)
// Bucles (for, while, loop)
// Match expressions
// Clausuras
// Expresiones anidadas

    let result = saludar(x.clone()); // Clonamos `x` para que no pierda su valor original.
    println!("Pasaje por propiedad: {}", result);

    // Pasaje por referencia mutable
    saludar2(&mut x);
    println!("Pasaje por referencia: {}", x);
}
