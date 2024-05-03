
---
¡Genial! Hemos instalado Rust y ahora estamos listos para escribir nuestro primer programa: el clásico "¡Hola, mundo!". Aquí están los pasos:

1. **Creación de un directorio de proyectos**: Empezamos creando un directorio para nuestros proyectos Rust. Esto es opcional, pero para este ejemplo, crearemos un directorio llamado `hello_world` dentro de un directorio llamado `projects`.

```bash
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

2. **Escribir y ejecutar un programa Rust**: Creamos un nuevo archivo llamado `main.rs` dentro de nuestro directorio `hello_world`. Este archivo contendrá nuestro código Rust.

```rust
fn main() {
    println!("¡Hola, mundo!");
}
```

Guardamos el archivo y en la terminal, compilamos y ejecutamos el programa:

```bash
$ rustc main.rs
$ ./main
¡Hola, mundo!
```

¡Y listo! Hemos escrito y ejecutado nuestro primer programa Rust.

Este programa consiste en una función especial llamada `main`, que es el punto de entrada de cualquier programa Rust. Dentro de esta función, utilizamos la macro `println!` para imprimir "¡Hola, mundo!" en la consola.

Espero que te haya gustado este primer vistazo a Rust. ¿Listo para más?


---