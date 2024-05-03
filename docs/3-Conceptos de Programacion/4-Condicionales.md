
---

## **Flujo de control en Rust**

En la programación, el flujo de control es crucial para ejecutar o repetir código según ciertas condiciones. En Rust, esto se logra principalmente mediante las expresiones `if` y los bucles. Veamos ejemplos de código para comprender mejor estos conceptos.

**Expresiones `if`**

La expresión `if` en Rust permite bifurcar el flujo del código según condiciones. Aquí tienes un ejemplo:

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("La condición era verdadera");
    } else {
        println!("La condición era falsa");
    }
}
```

En este código, si `number` es menor que 5, se imprime "La condición era verdadera"; de lo contrario, se imprime "La condición era falsa".

**Manejo de múltiples condiciones con `else if`**

Puedes encadenar múltiples condiciones usando `else if`. Por ejemplo:

```rust
fn main() {
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
}
```

Este programa imprime el primer resultado verdadero que encuentra, comenzando desde arriba hacia abajo.

**Usando `if` en una declaración `let`**

Como `if` **es** **una** **expresión** en Rust, puedes usarlo en la declaración `let` para asignar valores. Por ejemplo:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("El valor de number es: {number}");
}
```

Aquí, `number` se asigna a 5 si `condition` es verdadera, de lo contrario, se asigna a 6.



### **Bucles en Rust**

Rust ofrece varios tipos de bucles: `loop`, `while` y `for`. Veamos ejemplos de cada uno.

**Bucle `loop`**

El bucle `loop` ejecuta un bloque de código repetidamente hasta que se detiene explícitamente con `break`. Por ejemplo:

```rust
fn main() {
    loop {
        println!("¡De nuevo!");
    }
}
```

Este bucle se ejecutará infinitamente hasta que se detenga manualmente.

**Bucle `while`**

El bucle `while` ejecuta un bloque de código mientras una condición es verdadera. Por ejemplo:

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("¡Despegue!");
}
```

Este bucle imprime una cuenta regresiva desde 3 hasta 1, y luego imprime "¡Despegue!".

**Bucle `for`**

El bucle `for` itera sobre elementos de una colección. Por ejemplo:

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("El valor es: {element}");
    }
}
```

Este bucle imprime cada elemento de la matriz `a`.

**Resumen**

En este resumen, hemos explorado las expresiones `if` y los bucles en Rust, que son componentes esenciales para controlar el flujo de un programa. También hemos visto ejemplos de código para comprender mejor estos conceptos. Ahora estás listo para practicar y seguir adelante con el aprendizaje de Rust. ¡Adelante!

---