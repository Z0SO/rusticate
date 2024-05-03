
---

# Programando un juego de adivinanzas en Rust

En este tutorial, aprenderás a programar un juego de adivinanzas en Rust. Este juego generará un número aleatorio entre 1 y 100, luego solicitará al jugador que ingrese una suposición. El programa indicará si la suposición es demasiado baja o demasiado alta, y felicitará al jugador si la suposición es correcta.

### Configuración del proyecto

Para empezar, crea un nuevo proyecto Rust utilizando Cargo:

```bash
$ cargo new guessing_game
$ cd guessing_game
```

Esto creará un nuevo proyecto con el nombre `guessing_game`.

### Implementación

Abre el archivo `src/main.rs` y reemplaza el contenido con el siguiente código:

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

Este código solicita al jugador que ingrese una suposición y luego imprime la suposición ingresada.

### Almacenamiento de valores con variables

En Rust, puedes crear variables utilizando la palabra clave `let`. Por ejemplo:

```rust
let apples = 5; // inmutable
let mut bananas = 5; // mutable
```

En nuestro juego, creamos una variable mutable llamada `guess` para almacenar la suposición del jugador:

```rust
let mut guess = String::new();
```

### Recibir entrada del usuario

Utilizamos la función `read_line` del módulo `io` para obtener la entrada del usuario y almacenarla en la variable `guess`:

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

### Manejo de fallas potenciales con `Result`

La función `read_line` devuelve un `Result`, que puede ser `Ok` si la operación fue exitosa o `Err` si ocurrió un error. Utilizamos el método `expect` para manejar el error:

```rust
.expect("Failed to read line");
```

### Imprimir valores con `println!`

Por último, utilizamos la macro `println!` para imprimir la suposición del jugador:

```rust
println!("You guessed: {}", guess);
```

### Probando el programa

Ahora puedes compilar y ejecutar el programa utilizando Cargo:

```bash
$ cargo run
```

Esto te permitirá probar la primera parte del juego de adivinanzas.


# Generando un número secreto en Rust

En este paso, aprenderemos a generar un número secreto aleatorio que el jugador intentará adivinar. Utilizaremos la biblioteca `rand` para generar números aleatorios en el rango de 1 a 100.

### Usando una caja para obtener más funcionalidad

La biblioteca estándar de Rust no incluye funcionalidad para generar números aleatorios. Sin embargo, podemos utilizar la caja externa `rand`, que proporciona esta funcionalidad.

Para incluir la caja `rand` como una dependencia en nuestro proyecto, necesitamos agregarla al archivo `Cargo.toml`:

```toml
[dependencies]
rand = "0.8.5"
```

Luego, ejecutamos `cargo build` para descargar y compilar la caja `rand`.

### Generando un número aleatorio

Ahora que hemos incluido la caja `rand`, podemos usarla para generar un número aleatorio en nuestro juego. Aquí está el código actualizado en `src/main.rs`:

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

En este código, utilizamos `rand::thread_rng().gen_range(1..=100)` para generar un número aleatorio entre 1 y 100, inclusive.

¡Ahora puedes compilar y ejecutar el programa para probarlo! Cada vez que ejecutes el programa, obtendrás un número secreto diferente entre 1 y 100.

¡Genial! Ahora hemos agregado la funcionalidad para generar un número secreto aleatorio en nuestro juego de adivinanzas en Rust.



Aquí está tu hermoso apunte para Obsidian:

## Comparando la conjetura con el número secreto

Una parte emocionante del juego es comparar la conjetura del jugador con el número secreto. Esta comparación se realiza utilizando el método `cmp`, que devuelve una de las tres variantes de la enumeración `Ordering`: `Less`, `Greater`, o `Equal`. Vamos a ver cómo se implementa esto:

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

Este código compara la conjetura del usuario (`guess`) con el número secreto (`secret_number`) y proporciona retroalimentación en consecuencia.

## Permitir múltiples conjeturas con bucle

Para hacer el juego más interesante, permitimos que los jugadores realicen múltiples conjeturas. Esto se logra con un bucle infinito (`loop`) que solicita constantemente al usuario que ingrese una nueva conjetura:

```rust
loop {
    println!("Please input your guess.");

    // --snip--

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

Este bucle permite al usuario realizar conjeturas continuas hasta que adivine correctamente el número secreto, momento en el que el programa se detiene.

## Manejo de entradas no válidas

Es importante manejar las entradas que no son números para garantizar una experiencia de juego suave. Para esto, ignoramos cualquier conjetura que no sea un número y solicitamos otra conjetura al usuario:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

Esta parte del código convierte la conjetura del usuario de una cadena a un número (`u32`). Si la conversión tiene éxito, se procede con la comparación. De lo contrario, se ignora la conjetura no válida y se solicita otra al usuario.




---
