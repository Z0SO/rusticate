
---

### Funciones en Rust

Las funciones son fundamentales en Rust, y la palabra clave `fn` nos permite definirlas. Aquí hay un ejemplo básico de una función en Rust:

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

En este ejemplo, `main()` es la función de entrada y `another_function()` es una función adicional. Rust no tiene restricciones sobre dónde se definen las funciones dentro del código.

#### Parámetros

Las funciones pueden tener parámetros, que son variables especiales dentro de la firma de la función. Por ejemplo:

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

Aquí, `another_function()` toma un parámetro `x` de tipo `i32`.

#### Declaraciones y expresiones

En Rust, las funciones están compuestas de declaraciones y expresiones. Las declaraciones realizan acciones y no devuelven valores, mientras que las expresiones se evalúan como valores resultantes. Por ejemplo:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

En este caso, el bloque `{}` es una expresión que se evalúa como `4`.

#### Valores de retorno

Las funciones en Rust pueden devolver valores al código que las llama. La sintaxis `->` se utiliza para especificar el tipo de retorno de una función. Por ejemplo:

```rust
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("The value of x is: {x}");
}
```

Aquí, `five()` devuelve `5` como un valor de tipo `i32`.

#### Error común: Punto y coma

Es importante recordar que agregar un punto y coma al final de una expresión la convierte en una declaración, lo que puede causar errores si se espera un valor de retorno. Por ejemplo:

```rust
fn plus_one(x: i32) -> i32 {
    x + 1; // ¡Error! Esto devuelve `()`, no `i32`
}
```

Eliminar el punto y coma solucionaría este error.

Las funciones son una parte esencial del desarrollo en Rust, y entender cómo trabajar con parámetros, expresiones y valores de retorno es fundamental para escribir código efectivo.