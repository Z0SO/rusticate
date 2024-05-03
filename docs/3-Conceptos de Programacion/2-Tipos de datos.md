
---

En Rust, cada valor tiene un tipo de datos específico que le indica al compilador cómo trabajar con esos datos. Hay dos subconjuntos principales de tipos de datos: escalares y compuestos.

Los tipos de datos escalares representan valores únicos y comprenden enteros, números de punto flotante, booleanos y caracteres.

### Ejemplos de tipos escalares:

#### Tipos de números enteros:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

En este ejemplo, `guess` se declara como un entero sin signo de 32 bits (`u32`). Si no se proporciona la anotación de tipo (`: u32`), el compilador mostrará un error porque necesita más información para saber qué tipo usar.

#### Tipos de punto flotante:

```rust
let x = 2.0; // f64
let y: f32 = 3.0; // f32
```

Aquí, `x` se infiere como un punto flotante de 64 bits (`f64`) debido al valor, mientras que `y` se especifica explícitamente como un punto flotante de 32 bits (`f32`).

#### Tipo booleano:

```rust
let t = true;
let f: bool = false;
```

Los valores booleanos pueden ser `true` o `false`, y se especifican con el tipo `bool`.

#### Tipo de carácter:

```rust
let c = 'z';
let z: char = 'ℤ';
let heart_eyed_cat = '😻';
```

Los caracteres en Rust se especifican con el tipo `char` y pueden representar valores Unicode, incluidos emojis y caracteres alfabéticos.

Los tipos compuestos en Rust incluyen tuplas y matrices, que pueden agrupar múltiples valores en un solo tipo.

### Ejemplos de tipos compuestos:

#### Tuplas:

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

Una tupla puede contener varios valores de diferentes tipos, y se accede a sus elementos mediante desestructuración o indexación.

#### Matrices:

```rust
let a = [1, 2, 3, 4, 5];
```

Una matriz contiene elementos del mismo tipo y tiene una longitud fija. Se accede a los elementos de una matriz mediante indexación, y Rust verifica los límites de la matriz en tiempo de ejecución para evitar errores de acceso no válido a la memoria.

---
