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

En Rust, **punteros y referencias no son exactamente lo mismo**, aunque están relacionados. Aquí tienes una introducción clara:

## Introducción a Punteros y Referencias en Rust 🔑  

En Rust, un **puntero** es una dirección de memoria que apunta a un dato. Hay diferentes tipos de punteros en Rust, cada uno diseñado para ser seguro y evitar errores comunes como punteros nulos o uso indebido de memoria.  

Una **referencia** es un tipo especial de puntero que permite acceder a un valor sin transferir su propiedad. Rust asegura que las referencias sean válidas y seguras, previniendo condiciones como **uso después de liberar** (use after free).  

### Tipos Clave:
1. **Referencias (`&T` y `&mut T`)**  
   - Inmutables: Permiten leer un valor sin modificarlo.  
   - Mutables: Permiten leer y escribir un valor, pero con restricciones estrictas.  

2. **Punteros en el Heap (`Box<T>`)**  
   - Un puntero inteligente que almacena valores en el heap.  

3. **Opciones de punteros (`Option<&T>`)**  
   - Representa punteros opcionales, evitando errores de punteros nulos.  

### Diferencias:  
- **Referencia (`&T`)**: Vida corta, gestionada automáticamente.  
- **Puntero (`Box<T>`)**: Vida más larga, control explícito del heap.  

---

## Fundamento: Referencias  
#### Ejemplo de Uso Básico  
```rust
fn imprimir_numero(numero: &i32) {
    println!("El número es: {}", numero);
}

let valor = 10;
imprimir_numero(&valor); // Pasar una referencia inmutable.
```
---

## Introducción a Referencias  
#### Ejemplo de uso  
```rust
fn imprimir_numero(numero: &i32) {
    println!("El número es: {}", numero);
}

let valor = 10;
imprimir_numero(&valor); // Pasar una referencia inmutable.
```

### Ejercicio: Usa Referencias Inmutables  
### Descripción  
Escribe una función que reciba una referencia a un número (`i32`) y devuelva el doble del valor.  

#### Pista  
- Usa referencias inmutables (`&`) en la función.  

---

## Referencias Mutables  
#### Ejemplo de uso  
```rust
fn incrementar(valor: &mut i32) {
    *valor += 1; // Desreferencia para modificar el valor.
}

let mut numero = 5;
incrementar(&mut numero); // Pasar referencia mutable.
println!("Valor incrementado: {}", numero); // Esto imprime "6".
```

### Ejercicio: Incremento Seguro  
### Descripción  
Crea una función que reciba una referencia mutable a un número y lo incremente en 10.  

#### Pista  
- Usa `*` para desreferenciar y modificar el valor.  
- Declara la variable original como `mut`.  

---

## Trabajando con `Box`  
#### Ejemplo de uso  
Los `Box` son punteros inteligentes que almacenan datos en el heap en lugar de en el stack.  
```rust
fn duplicar(box_valor: &Box<i32>) -> i32 {
    **box_valor * 2 // Desreferencia doble para acceder al valor.
}

let valor = Box::new(7);
println!("El doble es: {}", duplicar(&valor));
```

### Ejercicio: Usa un `Box`  
### Descripción  
Crea una función que tome un `Box<i32>` como entrada y devuelva el triple del valor almacenado.  

#### Pista  
- Usa `*` para acceder al valor dentro del `Box`.  

---

## Referencias y Slices  
#### Ejemplo de uso  
```rust
fn suma(slice: &[i32]) -> i32 {
    slice.iter().sum() // Itera y suma los valores.
}

let numeros = vec![1, 2, 3, 4];
println!("La suma es: {}", suma(&numeros));
```

### Ejercicio: Sumar Rangos  
### Descripción  
Escribe una función que tome un slice de números y devuelva la suma de los valores entre dos índices dados.  

#### Pista  
- Usa slices (`&[i32]`) y el método `iter()` para iterar.  
- Asegúrate de manejar índices fuera de rango correctamente.  

---

## Opciones de Punteros (`Option<&T>`)  
#### Ejemplo de uso  
Rust utiliza `Option` para indicar valores opcionales o punteros nulos.  
```rust
fn buscar_mayor(numeros: &[i32]) -> Option<&i32> {
    numeros.iter().max()
}

let valores = vec![10, 20, 15];
if let Some(maximo) = buscar_mayor(&valores) {
    println!("El valor máximo es: {}", maximo);
}
```

### Ejercicio: Encuentra el Menor  
### Descripción  
Crea una función que reciba un slice de enteros y devuelva un `Option<&i32>` con el valor más pequeño.  

#### Pista  
- Usa el método `.min()` de los iteradores para obtener el mínimo.  

---

## Punteros y Seguridad  
#### Ejemplo de uso  
Rust evita el acceso a punteros nulos al requerir el uso explícito de `Option`.  
```rust
fn referencia_segura(texto: Option<&str>) {
    if let Some(valor) = texto {
        println!("Texto: {}", valor);
    } else {
        println!("No hay texto.");
    }
}

let mensaje = Some("Hola, Rust!");
referencia_segura(mensaje);
referencia_segura(None);
```

### Ejercicio: Referencia Nula Segura  
### Descripción  
Escribe una función que reciba un `Option<&i32>` y devuelva el doble del valor si existe, o `None` si no hay un valor.  

#### Pista  
- Usa `if let` o el método `.map()` de `Option` para manejar el caso.  

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
