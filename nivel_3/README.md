### **Desafío 1: Modelando un Rectángulo con Estructuras**

#### Objetivo

- Define una estructura `Rectangulo` con campos `ancho` y `alto`.
- Implementa un método para calcular el área del rectángulo.

#### Guía

1. Define una estructura `Rectangulo` con dos campos (`u32`).
2. Usa un bloque `impl` para asociar métodos a la estructura.
3. Crea un método `area` que calcule y devuelva el área.

**Pista:** La fórmula del área es `ancho * alto`.

---

### **Desafío 2: Una Calculadora usando Enumeraciones**

#### Objetivo

- Crea una enumeración `Operacion` con variantes como `Suma`, `Resta`, etc.
- Escribe una función que tome esta enumeración y dos números, devolviendo el resultado.

#### Guía

1. Define el enum `Operacion` con variantes básicas:
   ```rust
   enum Operacion {
       Suma,
       Resta,
       Multiplicacion,
       Division,
   }
   ```
2. Implementa una función que reciba dos números (`i32`) y la operación.
3. Usa un `match` para manejar las variantes y calcular el resultado.

**Pista:** Asegúrate de manejar errores como división por cero.

---

### **Desafío 3: Librería de Geometría Modularizada**

#### Objetivo

- Organiza el código en módulos dentro de una carpeta `geometria`.
- Implementa estructuras como `Rectangulo` y `Circulo` con métodos asociados.

#### Guía

1. Crea una carpeta `src/geometria` con un archivo `mod.rs`.
2. Define submódulos `rectangulo.rs` y `circulo.rs` para las estructuras.
3. Expón los módulos en `mod.rs` usando `pub mod`.
4. Desde el programa principal (`main.rs`), usa las funciones importadas.

**Pista:** Usa `pub use` para facilitar la importación.

---

### **Desafío 4: Uso de `Option` en una Estructura**

#### Objetivo

- Crea una estructura `Usuario` con un campo opcional `email`.
- Implementa un método que valide si el campo tiene un valor.

#### Guía

1. Define la estructura con un campo `Option<String>`:
   ```rust
   struct Usuario {
       nombre: String,
       email: Option<String>,
   }
   ```
2. Implementa un método que use `Option::is_some()` para validar.
3. Crea un usuario con y sin email para probar el comportamiento.

**Pista:** Experimenta con `Option::unwrap_or`.

---

### **Desafío 5: Sistema de Inventario**

#### Objetivo

- Usa `enum` para modelar ítems como `PocionDeVida`, `Espada`, etc.
- Crea funciones para describir cada ítem.

#### Guía

1. Define una enumeración con variantes asociadas:
   ```rust
   enum Item {
       PocionDeVida,
       Espada(u32),
       Armadura { defensa: u32, peso: f64 },
   }
   ```
2. Usa un `match` para devolver una descripción de cada ítem.
3. Prueba la función creando varios ítems diferentes.

**Pista:** Usa variantes con y sin datos asociados.

---

### **Desafío 6: Organización con Módulos**

#### Objetivo

- Reestructura un programa dividiéndolo en módulos para manejar usuarios y productos.

#### Guía

1. Crea archivos `usuario.rs` y `productos.rs` en la carpeta `src`.
2. Define estructuras y funciones en cada archivo.
3. Importa los módulos en `main.rs` usando `mod`.

**Pista:** Usa el archivo `mod.rs` si los módulos están en subcarpetas.

---

### **Desafío 7: Implementación de Traits para Estructuras**

#### Teoría

Un trait es similar a una **interfaz** en otros lenguajes como Java o TypeScript. Los traits permiten definir un comportamiento común que diferentes tipos pueden compartir, proporcionando una forma de trabajar con polimorfismo en Rust.

1. Definen comportamiento:

Un trait es como un contrato que especifica qué métodos deben implementarse para los tipos que lo adopten.

2. Polimorfismo:

Permiten escribir código genérico que funciona con cualquier tipo que implemente un determinado trait.

#### Objetivo

- Define un `trait` llamado `Describible` con un método `descripcion`.
- Implementa este trait para varias estructuras.

#### Guía

1. Define el trait:
   ```rust
   trait Describible {
       fn descripcion(&self) -> String;
   }
   ```
2. Implementa el trait para estructuras como `Rectangulo` o `Usuario`.
3. Usa el método para mostrar las descripciones.

**Pista:** Usa el método `to_string()` en los campos.

---

### **Desafío 8: Sistema de Configuración**

#### Objetivo

- Modela un sistema de configuración con un `struct Config`.
- Implementa un método para validar que los campos obligatorios están configurados.

#### Guía

1. Define una estructura con campos opcionales:
   ```rust
   struct Config {
       modo_debug: bool,
       nivel_log: Option<u32>,
       archivo_salida: Option<String>,
   }
   ```
2. Implementa un método que valide usando `Option::is_none()`.
3. Muestra mensajes de error si faltan campos.

**Pista:** Experimenta con valores predeterminados para los campos opcionales.

---

### **Desafío 9: Enumeraciones Anidadas**

#### Objetivo

- Modela un árbol binario con una enumeración.
- Escribe una función recursiva para contar los nodos.

#### Guía

1. Define la enumeración:
   ```rust
   enum Arbol {
       Nodo(Box<Arbol>, Box<Arbol>),
       Hoja,
   }
   ```
2. Implementa una función que cuente los nodos recursivamente.
3. Usa `Box` para manejar la recursión.

**Pista:** Devuelve 1 para cada nodo y 0 para las hojas.

---

### **Desafío 10: Un Blog Modularizado**

#### Objetivo

- Implementa un sistema básico de blog con usuarios, publicaciones y comentarios.

#### Guía

1. Crea módulos para `usuario`, `publicacion`, y `comentario`.
2. Define estructuras con campos básicos:
   ```rust
   struct Publicacion {
       titulo: String,
       contenido: String,
       autor: String,
   }
   ```
3. Implementa métodos para agregar, editar y eliminar.
4. Conecta las entidades mediante referencias o identificadores.

**Pista:** Usa `Vec` para almacenar múltiples publicaciones y comentarios.

---

Con estos desafíos, ¡puedes fortalecer tus conocimientos sobre composición y modularidad en Rust! ¿Te preparo algún detalle adicional o te ayudo con la estructura de un módulo? 🚀
