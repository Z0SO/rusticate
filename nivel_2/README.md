### Nivel 2 - Propiedad, Referencias y Préstamos 🔑

---

#### **Desafío 1: Transferencia de Propiedad (Ownership Transfer)**

- **Objetivo:** Escribe un programa donde una variable transfiera su propiedad a otra. Intenta usar la variable original después de la transferencia.
- **Pista:** Define una función que tome una `String` como argumento, observa qué sucede si intentas usar esa `String` después de pasarla.

---

#### **Desafío 2: Evitando Clonaciones Innecesarias**

- **Objetivo:** Usa una referencia para evitar que una `String` pierda su propiedad cuando se pasa a una función que solo necesita leerla.
- **Pista:** Cambia la función para que reciba una referencia (`&String`) en lugar de tomar la propiedad completa.

---

#### **Desafío 3: Préstamos Mutables**

- **Objetivo:** Crea una función que modifique una cadena de texto a través de un préstamo mutable.
- **Pista:** Usa `&mut` para pasar una referencia mutable y agrega texto al final de la cadena.

---

#### **Desafío 4: Restricción de Préstamos**

- **Objetivo:** Intenta crear y usar tanto un préstamo mutable como uno inmutable al mismo tiempo para la misma variable. Observa qué error genera Rust.
- **Pista:** Experimenta declarando múltiples referencias con y sin mutabilidad.

---

#### **Desafío 5: División de Slices**

- **Objetivo:** Escribe una función que tome un slice de un array y devuelva un nuevo slice con solo los números pares.
- **Pista:** Usa slices (`&[i32]`) en lugar de pasar el array completo. Itera y filtra los valores.

---

#### **Desafío 6: Límite de Vida de Referencias**

- **Objetivo:** Define una estructura que contenga una referencia. Intenta usar esa referencia después de que su dueño haya sido liberado.
- **Pista:** Aprende sobre los `'static` lifetimes y cómo resolver errores de límites de vida.

---

#### **Desafío 7: Sumar Elementos Usando Slices**

- **Objetivo:** Implementa una función que reciba un slice de enteros y devuelva su suma.
- **Pista:** Usa iteradores o bucles simples. Asegúrate de usar slices (`&[i32]`) como entrada.

---

#### **Desafío 8: Mutabilidad y Ownership Combinados**

- **Objetivo:** Crea un programa que tenga una `Vec<String>`. Pasa su propiedad a una función, modifícala y devuélvela al programa principal.
- **Pista:** Usa el operador `return` explícitamente para devolver la propiedad.

---

#### **Desafío 9: Split y Referencias en Cadenas**

- **Objetivo:** Escribe una función que tome una cadena como referencia, la divida por espacios y devuelva el primer elemento como un slice.
- **Pista:** Usa el método `split` de `str` y devuelve un `&str`.

---

#### **Desafío 10: Ownership con Tipos Compuestos**

- **Objetivo:** Define una estructura que tenga dos campos: un número (`i32`) y una `String`. Escribe una función que consuma la estructura, modifique sus valores y la devuelva.
- **Pista:** Usa `struct` y `Option` para evitar perder propiedad.

---

¿Quieres que comience con alguno de estos desafíos o prepare otro enfoque para este nivel? 😊
