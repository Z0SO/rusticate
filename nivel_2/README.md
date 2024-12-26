# Nivel 2

## Propiedad 🔑  
#### Ejemplo de uso  
Cuando transfieres una variable a otra, Rust cede la propiedad para garantizar la seguridad de la memoria. Esto significa que no puedes usar la variable original después de la transferencia.  
```rust
let saludo = String::from("Hola, mundo!");
let nuevo_saludo = saludo; // `saludo` transfiere su propiedad.
println!("{}", nuevo_saludo); // Funciona.
println!("{}", saludo);      // Error: `saludo` ya no es válido.
```

### Ejercicio: Rastrea la Propiedad  
### Descripción  
Crea un programa que asigne una `String` a una variable, transfiera su propiedad a otra, e intente usar la variable original. Observa el error de compilación y corrígelo.  

#### Pista  
- Usa el método `.clone()` para crear una copia si necesitas mantener la propiedad original.  

---


## Referencias Inmutables 🔑  
#### Ejemplo de uso  
Una referencia inmutable permite leer datos sin transferir propiedad.  
```rust
fn mostrar_mensaje(mensaje: &String) {
    println!("Mensaje: {}", mensaje);
}
let texto = String::from("Rust es divertido!");
mostrar_mensaje(&texto); // Pasa una referencia.
println!("{}", texto);   // Funciona porque la propiedad no se transfirió.
```

### Ejercicio: Evitar Transferencias  
### Descripción  
Escribe una función que reciba una referencia a una `String` y simplemente la imprima. La propiedad de la `String` no debe cambiar.  

#### Pista  
- Usa referencias (`&`) al pasar argumentos a funciones.  

---

## Préstamos Mutables 🔑  
#### Ejemplo de uso  
Un préstamo mutable permite modificar datos mientras garantiza que no existan otras referencias al mismo tiempo.  
```rust
fn agregar_texto(cadena: &mut String) {
    cadena.push_str(" ¡Rust Rocks!");
}

let mut frase = String::from("Hola");
agregar_texto(&mut frase); // Préstamo mutable.
println!("{}", frase);     // Esto imprime "Hola ¡Rust Rocks!"
```

### Ejercicio: Modificando Texto  
### Descripción  
Crea una función que reciba una referencia mutable a una `String` y agregue un texto adicional.  

#### Pista  
- Declara la variable original como `mut`.  
- Usa `&mut` al pasar la referencia a la función.  

---

## Restricción de Préstamos 🔑  
#### Ejemplo de uso  
Rust no permite usar préstamos mutables e inmutables simultáneamente.  
```rust
let mut numero = 42;
let referencia_inmutable = &numero;
let referencia_mutable = &mut numero; // Error: Rust no lo permite.
println!("{}", referencia_inmutable);
```

### Ejercicio: Experimenta con Restricciones  
### Descripción  
Intenta crear tanto una referencia mutable como una inmutable para la misma variable. Observa el error generado y corrígelo eliminando una de las referencias.  

---

## Slices 🔑  
#### Ejemplo de uso  
Los slices son vistas inmutables de datos más grandes.  
```rust
fn primera_palabra(oracion: &str) -> &str {
    oracion.split_whitespace().next().unwrap_or("")
}

let frase = "Rust es asombroso";
let palabra = primera_palabra(frase);
println!("Primera palabra: {}", palabra); // Esto imprime "Rust".
```

### Ejercicio: Extraer Palabras  
### Descripción  
Escribe una función que tome una cadena y devuelva su última palabra como slice (`&str`).  

#### Pista  
- Usa `split_whitespace()` y el método `last()` para obtener el último elemento.  

---

## Límites de Vida 🔑  
#### Ejemplo de uso  
Rust usa lifetimes para garantizar que las referencias sean válidas durante su uso.  
```rust
struct Contenedor<'a> {
    contenido: &'a str,
}

fn crear_contenedor<'a>(texto: &'a str) -> Contenedor<'a> {
    Contenedor { contenido: texto }
}

let texto = String::from("Ejemplo");
let contenedor = crear_contenedor(&texto);
println!("{}", contenedor.contenido);
```

### Ejercicio: Referencias Seguras  
### Descripción  
Crea una estructura que almacene una referencia a una cadena. Intenta usar la referencia después de que el dueño haya sido liberado y corrige el error usando lifetimes.  

#### Pista  
- Usa anotaciones de lifetime (`'a`) para indicar que la referencia en la estructura es válida mientras su dueño lo sea.  
