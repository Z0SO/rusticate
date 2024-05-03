### Variables y Mutabilidad

En Rust, las variables son inmutables por defecto, lo que significa que una vez que asignas un valor a una variable, no puedes cambiarlo. Esto es parte de la estrategia de Rust para garantizar la seguridad y facilitar la concurrencia en el código. Sin embargo, tienes la opción de hacer que tus variables sean mutables, lo que te permite cambiar su valor. Esto se logra agregando la palabra clave `mut` delante del nombre de la variable.

```rust
fn main() {
    let mut x = 5;
    println!("El valor de x es: {x}");
    x = 6;
    println!("El valor de x es: {x}");
}
```

Cuando ejecutas este programa, puedes ver que puedes cambiar el valor de `x` después de haberlo inicializado como mutable.

### Constantes

Las constantes son similares a las variables inmutables, pero con algunas diferencias clave. En Rust, las constantes son siempre inmutables y se declaran utilizando la palabra clave `const`. Se debe especificar el tipo de valor para las constantes y solo se pueden asignar con una expresión constante, es decir, su valor debe ser conocido en tiempo de compilación.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Las constantes son útiles para valores que son conocidos en tiempo de compilación y que se necesitan en todo el programa. Proporcionan un medio para nombrar valores que no deben cambiar y para transmitir el significado de esos valores a otros desarrolladores que lean el código.

### Sombreado

En Rust, puedes declarar una nueva variable con el mismo nombre que una variable anterior, lo que se conoce como sombreado. Esto significa que la nueva variable "eclipsa" a la anterior en el alcance en el que se declara. Sin embargo, esto es diferente de marcar una variable como mutable, ya que una vez que una variable está sombreada, no se puede cambiar su valor sin volver a declararla.

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("El valor de x en el alcance interno es: {x}");
    }

    println!("El valor de x es: {x}");
}
```

En este ejemplo, la variable `x` se sombrea dos veces: primero, se incrementa en 1, y luego se multiplica por 2 dentro de un alcance interno. Esto ilustra cómo se puede reutilizar el mismo nombre de variable para diferentes propósitos dentro del mismo ámbito.

Estos conceptos son fundamentales para entender cómo funcionan las variables en Rust y cómo puedes utilizarlas de manera efectiva en tus programas.