### **Desafío 1: Crear una CLI con `clap`**  

#### Objetivo  
- Crear una aplicación de línea de comandos (CLI) que acepte varios parámetros de entrada y ejecute diferentes funciones dependiendo de los argumentos proporcionados.

#### Guía  
1. **Instalar `clap`**: Agrega `clap` a tu `Cargo.toml`:
   ```toml
   [dependencies]
   clap = "4.0"
   ```
   
2. **Configurar los comandos**:  
   Define una aplicación que acepte al menos tres tipos de comandos. Por ejemplo:  
   - `add`: Agregar dos números.
   - `subtract`: Restar dos números.
   - `multiply`: Multiplicar dos números.
   
   Utiliza `clap` para definir los comandos y los parámetros que tomarán. Aquí tienes un ejemplo básico:
   ```rust
   use clap::{Arg, Command};
   
   fn main() {
       let matches = Command::new("Calculadora CLI")
           .version("1.0")
           .author("Tu Nombre <tu@correo.com>")
           .about("Una simple calculadora de línea de comandos")
           .subcommand(
               Command::new("add")
                   .about("Suma dos números")
                   .arg(Arg::new("x").required(true).index(1))
                   .arg(Arg::new("y").required(true).index(2)),
           )
           .subcommand(
               Command::new("subtract")
                   .about("Resta dos números")
                   .arg(Arg::new("x").required(true).index(1))
                   .arg(Arg::new("y").required(true).index(2)),
           )
           .subcommand(
               Command::new("multiply")
                   .about("Multiplica dos números")
                   .arg(Arg::new("x").required(true).index(1))
                   .arg(Arg::new("y").required(true).index(2)),
           )
           .get_matches();
       
       match matches.subcommand() {
           Some(("add", sub_matches)) => {
               let x: f64 = sub_matches.get_one::<String>("x").unwrap().parse().unwrap();
               let y: f64 = sub_matches.get_one::<String>("y").unwrap().parse().unwrap();
               println!("Resultado: {}", x + y);
           }
           Some(("subtract", sub_matches)) => {
               let x: f64 = sub_matches.get_one::<String>("x").unwrap().parse().unwrap();
               let y: f64 = sub_matches.get_one::<String>("y").unwrap().parse().unwrap();
               println!("Resultado: {}", x - y);
           }
           Some(("multiply", sub_matches)) => {
               let x: f64 = sub_matches.get_one::<String>("x").unwrap().parse().unwrap();
               let y: f64 = sub_matches.get_one::<String>("y").unwrap().parse().unwrap();
               println!("Resultado: {}", x * y);
           }
           _ => println!("Comando no válido"),
       }
   }
   ```

3. **Prueba**: Llama a tu CLI desde la terminal:
   - `cargo run add 3 4`
   - `cargo run subtract 10 4`
   - `cargo run multiply 2 5`

**Pista**: Experimenta con la validación de entradas (por ejemplo, asegurándote de que los valores proporcionados sean numéricos).

---

### **Desafío 2: Conectar a una API REST con `reqwest`**  

#### Objetivo  
- Crear una pequeña aplicación que haga peticiones a una API REST y procese la respuesta.

#### Guía  
1. **Instalar `reqwest`**:  
   Agrega `reqwest` y `tokio` en el `Cargo.toml`:
   ```toml
   [dependencies]
   reqwest = { version = "0.11", features = ["json"] }
   tokio = { version = "1", features = ["full"] }
   ```
   
2. **Configurar el Cliente HTTP**:
   Crea una función asincrónica que haga una petición GET a una API pública, como la de [JSONPlaceholder](https://jsonplaceholder.typicode.com).

   ```rust
   use reqwest::Error;
   
   #[tokio::main]
   async fn main() -> Result<(), Error> {
       let url = "https://jsonplaceholder.typicode.com/posts";
       let response = reqwest::get(url).await?;
       
       let posts: Vec<serde_json::Value> = response.json().await?;
       println!("Posts: {:#?}", posts);
   
       Ok(())
   }
   ```

3. **Probar con una API POST**:
   Crea una función que permita hacer una solicitud POST enviando un JSON con datos.

   ```rust
   use reqwest::Client;
   use serde::Serialize;
   use reqwest::Error;
   
   #[derive(Serialize)]
   struct NewPost {
       title: String,
       body: String,
       user_id: i32,
   }
   
   #[tokio::main]
   async fn main() -> Result<(), Error> {
       let client = Client::new();
       let new_post = NewPost {
           title: String::from("Nuevo Título"),
           body: String::from("Este es el cuerpo del nuevo post."),
           user_id: 1,
       };
   
       let response = client
           .post("https://jsonplaceholder.typicode.com/posts")
           .json(&new_post)
           .send()
           .await?;
   
       println!("Respuesta: {:#?}", response.text().await?);
   
       Ok(())
   }
   ```

**Pista**: Usa `serde` para deserializar la respuesta JSON y crear tipos Rust adecuados.

---

### **Desafío 3: Construir un Servidor Web Básico con `warp` o `axum`**  

#### Objetivo  
- Construir un servidor web básico que reciba peticiones HTTP y responda con diferentes datos.

#### Guía  
1. **Instalar dependencias**:  
   En tu `Cargo.toml`, agrega `warp` (o `axum` si prefieres esa opción):
   ```toml
   [dependencies]
   warp = "0.3"
   tokio = { version = "1", features = ["full"] }
   ```

2. **Configurar el Servidor Web con `warp`**:
   ```rust
   use warp::Filter;
   
   #[tokio::main]
   async fn main() {
       // Ruta básica que responde con un saludo
       let hello = warp::path("hello")
           .map(|| warp::reply::html("¡Hola, mundo!"));
   
       // Ruta que acepta parámetros y responde con ellos
       let greet = warp::path!("greet" / String)
           .map(|name| format!("¡Hola, {}!", name));
   
       // Combinamos las rutas
       let routes = hello.or(greet);
   
       // Iniciamos el servidor en el puerto 3030
       warp::serve(routes)
           .run(([127, 0, 0, 1], 3030))
           .await;
   }
   ```

3. **Probar el Servidor**:  
   - Inicia el servidor: `cargo run`.
   - Accede a `http://localhost:3030/hello` y `http://localhost:3030/greet/tu_nombre`.

**Pista**: Experimenta añadiendo más rutas y manejadores para realizar operaciones como devolver JSON o procesar formularios.

---

### **Desafío 4: Sistema de Archivos: Lectura y Escritura de Archivos**  

#### Objetivo  
- Crear un programa que lea y escriba datos en archivos de texto.

#### Guía  
1. **Leer un archivo**:
   Usa `std::fs::File` para abrir un archivo y `std::io::Read` para leer su contenido.
   ```rust
   use std::fs::File;
   use std::io::{self, Read};
   
   fn leer_archivo(path: &str) -> io::Result<String> {
       let mut file = File::open(path)?;
       let mut contenido = String::new();
       file.read_to_string(&mut contenido)?;
       Ok(contenido)
   }
   
   fn main() {
       match leer_archivo("mi_archivo.txt") {
           Ok(contenido) => println!("Contenido del archivo: {}", contenido),
           Err(e) => eprintln!("Error al leer el archivo: {}", e),
       }
   }
   ```

2. **Escribir en un archivo**:
   Usa `std::fs::File` y `std::io::Write` para escribir datos en un archivo.
   ```rust
   use std::fs::File;
   use std::io::{self, Write};
   
   fn escribir_archivo(path: &str, contenido: &str) -> io::Result<()> {
       let mut file = File::create(path)?;
       file.write_all(contenido.as_bytes())?;
       Ok(())
   }
   
   fn main() {
       match escribir_archivo("mi_archivo.txt", "Este es un texto de ejemplo.") {
           Ok(()) => println!("Archivo escrito exitosamente"),
           Err(e) => eprintln!("Error al escribir el archivo: {}", e),
       }
   }
   ``

`

3. **Manipulación avanzada**:  
   Experimenta con la lectura y escritura en archivos binarios, la creación de directorios o la verificación de la existencia de archivos.

---

### Resumen

Estos desafíos te permitirán desarrollar proyectos que cubren una gama de funcionalidades avanzadas en Rust. Al completarlos, habrás ganado experiencia trabajando con entradas y salidas, conexiones de red, servidores web, y manipulación de archivos, lo que te prepara para tareas más complejas. ¡Buena suerte! 🚀
