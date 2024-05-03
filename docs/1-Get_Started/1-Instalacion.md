
---

Para instalar Rust, utilizaremos rustup, una herramienta de línea de comandos que gestiona las versiones de Rust y sus herramientas asociadas. Puedes instalarla mediante el siguiente comando en Linux o macOS:

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Si estás en Windows, visita [este enlace](https://www.rust-lang.org/tools/install) para seguir las instrucciones de instalación.

Después de la instalación, es recomendable verificar si Rust se instaló correctamente. Puedes hacerlo ejecutando:

```bash
$ rustc --version
```

Si ves el número de versión, la confirmación y la fecha de la última versión estable, ¡la instalación fue exitosa!

Para actualizar Rust, utiliza:

```bash
$ rustup update
```

Y para desinstalar, ejecuta:

```bash
$ rustup self uninstall
```

Además, con la instalación se incluye una copia local de la documentación de Rust. Puedes acceder a ella sin conexión ejecutando `rustup doc` en tu terminal.


---


