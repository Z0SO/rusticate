### **Desafío 1: Calculadora con Manejo de Errores**  

#### Objetivo  
- Crea una función que tome dos números y una operación (`+`, `-`, `*`, `/`) y devuelva un `Result`.  
- Maneja errores como división por cero o entrada inválida.  

#### Guía  
1. Define un enum para los posibles errores:  
   ```rust
   enum Error {
       DivisionPorCero,
       OperacionInvalida,
   }
   ```  
2. Crea una función que devuelva un `Result<f64, Error>`.  
3. Usa un `match` para manejar las operaciones y los errores.  

**Pista:** Usa `Ok(valor)` para un resultado válido y `Err(error)` para errores.  

---

### **Desafío 2: Validación de Archivos con `Option` y `Result`**  

#### Objetivo  
- Implementa una función que lea un archivo y devuelva su contenido.  
- Devuelve un `Result` en caso de error, como si el archivo no existe.  

#### Guía  
1. Usa la función `std::fs::read_to_string` para leer archivos.  
2. Maneja los errores usando un `match` o el operador `?`.  
3. Devuelve un `Result<String, std::io::Error>`.  

**Pista:** Usa la función `unwrap_or_else` para manejar valores opcionales al probar tu código.  

---

### **Desafío 3: Propagación de Errores en un Servicio de Usuario**  

#### Objetivo  
- Implementa funciones para un sistema de registro de usuarios.  
- Propaga errores cuando:  
  - El nombre de usuario ya existe.  
  - La contraseña es demasiado corta.  

#### Guía  
1. Define una estructura `Usuario`:  
   ```rust
   struct Usuario {
       nombre: String,
       contraseña: String,
   }
   ```  
2. Crea una función `registrar_usuario` que devuelva un `Result`:  
   ```rust
   fn registrar_usuario(usuarios: &mut Vec<Usuario>, nombre: String, contraseña: String) -> Result<(), String>;
   ```  
3. Usa el operador `?` para simplificar la propagación de errores.  

**Pista:** Maneja errores como `UsuarioYaExiste` o `ContraseñaInsegura` devolviendo mensajes detallados.  

---

### **Desafío 4: Función Anidada con Propagación de Errores**  

#### Objetivo  
- Implementa una función que realice múltiples operaciones (por ejemplo, sumar números desde un archivo).  
- Propaga errores en cada paso.  

#### Guía  
1. Crea una función que lea números desde un archivo.  
2. Implementa otra función que calcule la suma de los números.  
3. Propaga errores desde las funciones internas usando `?`.  

**Pista:** Diseña funciones intermedias que devuelvan un `Result` y encadénalas en la principal.  

---

### **Desafío 5: Manejador de Configuración Opcional**  

#### Objetivo  
- Implementa un sistema de configuración donde algunas opciones sean opcionales.  
- Devuelve un error si falta un campo obligatorio.  

#### Guía  
1. Define una estructura `Config`:  
   ```rust
   struct Config {
       puerto: u16,
       host: Option<String>,
   }
   ```  
2. Implementa una función que valide la configuración:  
   ```rust
   fn validar_config(config: Config) -> Result<(), String>;
   ```  
3. Devuelve un `Err` si el campo obligatorio `puerto` está fuera de rango.  

**Pista:** Usa combinadores como `map_or` o `and_then` para manejar valores opcionales.  

---

### **Desafío 6: Simulador de Banco con Resultados Variables**  

#### Objetivo  
- Simula un sistema bancario donde las transacciones pueden fallar (por ejemplo, fondos insuficientes).  
- Usa `Result` para manejar los errores.  

#### Guía  
1. Define una estructura `Cuenta`:  
   ```rust
   struct Cuenta {
       titular: String,
       balance: f64,
   }
   ```  
2. Crea funciones para:  
   - **Depositar fondos.**  
   - **Retirar fondos.**  
   - Devuelve un error si no hay suficiente dinero para la operación.  

**Pista:** Usa `Ok` para transacciones exitosas y `Err` para fallos.  

---

### **Desafío 7: Descargador con Simulación de Errores**  

#### Objetivo  
- Escribe una función que simule la descarga de datos de una URL.  
- Devuelve errores si la URL es inválida o la conexión falla.  

#### Guía  
1. Define un enum `ErrorDescarga` para los posibles errores:  
   ```rust
   enum ErrorDescarga {
       UrlInvalida,
       ConexionFallida,
   }
   ```  
2. Usa una función que tome una URL como entrada y devuelva un `Result<String, ErrorDescarga>`.  
3. Simula errores aleatorios usando la función `rand::random`.  

**Pista:** Experimenta con `Result::unwrap_or_default` para manejar los errores en pruebas.  

---

Estos desafíos te ayudarán a dominar la gestión de errores en Rust, desde `Result` y `Option` hasta la propagación y manejo avanzado de errores. ¿Te interesa alguno en particular para desarrollarlo con más detalle? 😊
