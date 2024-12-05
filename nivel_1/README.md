### Nivel 1 - Fundamentos: **Ejercicio 1 y Ejercicio 2**

---

# Ejercicio 1: Hola, Mundo 👋

## 📝 Descripción

Tu primer programa en **Rust** será un clásico: ¡mostrar "Hola, mundo!" en la terminal! Este ejercicio te permitirá configurar tu entorno y compilar tu primer programa.

---

## 🎯 Objetivo

1. Crear un proyecto de Rust usando `cargo`.
2. Imprimir un mensaje en la terminal.

---

## 🚀 Pasos

1. **Crea un nuevo proyecto de Rust:**
   ```bash
   cargo new hola_mundo
   cd hola_mundo
   ```

2. **Edita el archivo `src/main.rs`:**  
   Cambia el contenido del archivo para que sea el siguiente:
   ```rust
   fn main() {
       println!("Hola, mundo!");
   }
   ```

3. **Compila y ejecuta el programa:**
   ```bash
   cargo run
   ```

4. **Resultado esperado:**  
   En la terminal deberías ver:
   ```
   Hola, mundo!
   ```

---

## 🏆 Desafío adicional

Modifica el mensaje para que incluya tu nombre:  
```
Hola, mundo, soy [tu nombre] aprendiendo Rust.
```

---  

# Ejercicio 2: Variables y Mutabilidad 🛠️

## 📝 Descripción

Este ejercicio te ayudará a entender cómo declarar variables en **Rust**, cómo funciona la **mutabilidad** y cómo realizar operaciones básicas con ellas.

---

## 🎯 Objetivo

1. Declarar variables inmutables y mutables.
2. Realizar operaciones aritméticas básicas.
3. Imprimir el valor de las variables.

---

## 🚀 Pasos

1. **Edita el archivo `src/main.rs` en tu proyecto o crea uno nuevo:**
   ```bash
   cargo new variables
   cd variables
   ```

2. **Cambia el contenido de `src/main.rs`:**
   ```rust
   fn main() {
       // Declaración de una variable inmutable
       let x = 5;
       println!("El valor de x es: {}", x);

       // Declaración de una variable mutable
       let mut y = 10;
       println!("El valor inicial de y es: {}", y);

       // Modificar el valor de una variable mutable
       y = y + 5;
       println!("El nuevo valor de y es: {}", y);

       // Operaciones aritméticas
       let suma = x + y;
       println!("La suma de x e y es: {}", suma);
   }
   ```

3. **Compila y ejecuta el programa:**
   ```bash
   cargo run
   ```

4. **Resultado esperado:**  
   El programa debería imprimir algo similar a:
   ```
   El valor de x es: 5
   El valor inicial de y es: 10
   El nuevo valor de y es: 15
   La suma de x e y es: 20
   ```

---

## 🏆 Desafío adicional

1. Declara una variable con un tipo explícito (`i32`, `f64`, etc.).  
2. Intenta usar una variable inmutable como si fuera mutable. ¿Qué sucede?  

Ejemplo:  
```rust
let z = 3;
z = z + 1; // ¿Qué pasa aquí?
```

--- 

### Nivel 1 - Fundamentos: **Ejercicio 3 y Ejercicio 4**


# Ejercicio 3: Tipos de Datos y Conversiones 🔢

## 📝 Descripción

En Rust, cada valor tiene un tipo, y estos pueden ser explícitos o inferidos. En este ejercicio, aprenderás a trabajar con diferentes tipos de datos y cómo realizar conversiones entre ellos.

---

## 🎯 Objetivo

1. Declarar variables con tipos específicos.
2. Usar diferentes tipos de datos (`i32`, `f64`, `bool`, `char`, etc.).
3. Realizar conversiones entre tipos.

---

## 🚀 Pasos

1. **Edita el archivo `src/main.rs`:**
   ```rust
   fn main() {
       // Enteros
       let numero_entero: i32 = 42;
       println!("El número entero es: {}", numero_entero);

       // Flotantes
       let numero_flotante: f64 = 3.14;
       println!("El número flotante es: {}", numero_flotante);

       // Booleanos
       let es_verano: bool = true;
       println!("¿Es verano?: {}", es_verano);

       // Caracteres
       let letra: char = 'R';
       println!("El carácter es: {}", letra);

       // Conversión explícita
       let entero_a_flotante = numero_entero as f64;
       println!("El entero como flotante es: {}", entero_a_flotante);
   }
   ```

2. **Compila y ejecuta el programa:**
   ```bash
   cargo run
   ```

3. **Resultado esperado:**  
   El programa debería imprimir algo similar a:
   ```
   El número entero es: 42
   El número flotante es: 3.14
   ¿Es verano?: true
   El carácter es: R
   El entero como flotante es: 42.0
   ```

---

## 🏆 Desafío adicional

1. Intenta realizar operaciones entre tipos diferentes (por ejemplo, sumar un `i32` con un `f64`). ¿Qué sucede?  
2. Declara una constante usando la palabra clave `const`. Ejemplo:
   ```rust
   const PI: f64 = 3.14159;
   println!("El valor de PI es: {}", PI);
   ```

---

# Ejercicio 4: Control de Flujo 🔄

## 📝 Descripción

Rust tiene potentes herramientas para el control de flujo, incluyendo condicionales (`if`, `else`) y bucles (`for`, `while`, `loop`). Este ejercicio te ayudará a practicar estas estructuras.

---

## 🎯 Objetivo

1. Usar condicionales para tomar decisiones.
2. Escribir bucles para iterar sobre valores.
3. Comprender el uso de la palabra clave `break` para salir de un bucle.

---

## 🚀 Pasos

1. **Edita el archivo `src/main.rs`:**
   ```rust
   fn main() {
       // Condicionales
       let numero = 7;
       if numero % 2 == 0 {
           println!("El número {} es par.", numero);
       } else {
           println!("El número {} es impar.", numero);
       }

       // Bucle 'for'
       println!("Imprimiendo números del 1 al 5:");
       for i in 1..=5 {
           println!("{}", i);
       }

       // Bucle 'while'
       let mut contador = 3;
       println!("Cuenta regresiva:");
       while contador > 0 {
           println!("{}", contador);
           contador -= 1;
       }

       // Bucle infinito con 'break'
       let mut suma = 0;
       loop {
           suma += 1;
           if suma == 10 {
               println!("Suma alcanzó el valor 10.");
               break;
           }
       }
   }
   ```

2. **Compila y ejecuta el programa:**
   ```bash
   cargo run
   ```

3. **Resultado esperado:**  
   El programa debería imprimir algo como:
   ```
   El número 7 es impar.
   Imprimiendo números del 1 al 5:
   1
   2
   3
   4
   5
   Cuenta regresiva:
   3
   2
   1
   Suma alcanzó el valor 10.
   ```

---

## 🏆 Desafío adicional

1. Modifica el bucle `for` para que imprima los números pares entre 1 y 10.  
2. Usa un bucle `while` para calcular el factorial de un número dado (por ejemplo, `5! = 5 * 4 * 3 * 2 * 1`).  

---

### Nivel 2 - Desafíos Avanzados 🚀

---

#### **Desafío 1: Números Primos**
- Escribe un programa que determine si un número dado es primo. 
- Pista: Un número primo solo es divisible por 1 y por sí mismo. Usa bucles y condicionales para verificarlo.

---

#### **Desafío 2: FizzBuzz**
- Imprime los números del 1 al 100, pero:
  - Sustituye los números divisibles por 3 con "Fizz".
  - Sustituye los números divisibles por 5 con "Buzz".
  - Sustituye los números divisibles por ambos con "FizzBuzz".
- Pista: Usa operadores `%` y `if` combinados.

---

#### **Desafío 3: Fibonacci Recursivo**
- Crea una función recursiva que calcule el `n`-ésimo número de la secuencia de Fibonacci. 
- Pista: La secuencia empieza con `0, 1, 1, 2, 3, 5, ...`. La fórmula es `F(n) = F(n-1) + F(n-2)`.

---

#### **Desafío 4: Inversión de Cadenas**
- Escribe un programa que tome una cadena como entrada y devuelva la cadena invertida.
- Pista: Usa bucles o métodos específicos de las cadenas.

---

#### **Desafío 5: Palíndromo**
- Determina si una palabra o frase es un palíndromo (se lee igual de izquierda a derecha que de derecha a izquierda).
- Pista: Ignora los espacios y las mayúsculas. Usa manipulación de cadenas.

---

#### **Desafío 6: Juego del Ahorcado**
- Implementa un juego sencillo del ahorcado donde el usuario intente adivinar una palabra.
- Pista: Usa estructuras como `Vec` para almacenar las letras adivinadas y las que faltan.

---

#### **Desafío 7: Ordenamiento de un Array**
- Implementa un algoritmo que ordene un array de números enteros (por ejemplo, bubble sort o insertion sort).
- Pista: Usa bucles anidados y compara elementos adyacentes.

---

#### **Desafío 8: Calculadora RPN (Notación Polaca Inversa)**
- Escribe un programa que evalúe expresiones en notación polaca inversa (por ejemplo, `3 4 +`).
- Pista: Usa una pila para realizar las operaciones.

---

#### **Desafío 9: Generador de Contraseñas**
- Crea un generador de contraseñas que incluya letras, números y caracteres especiales.
- Pista: Usa el módulo `rand` de Rust para generar valores aleatorios.

---

#### **Desafío 10: Simulación de Juego de la Vida de Conway**
- Implementa el Juego de la Vida de Conway en una cuadrícula bidimensional.
- Pista: Cada célula tiene vecinos y sigue reglas basadas en su estado (viva/muerta).

---

¿Con cuál de estos desafíos avanzados quieres empezar? 😃
