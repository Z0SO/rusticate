// - Define un `trait` llamado `Describible` con un método `descripcion`.
// - Implementa este trait para varias estructuras.

pub use crate::d1::Rectangulo;

trait Describible {
    fn descripcion(&self) -> String;
}

struct Cancion {
    titulo: String,
    link: String, // no hace falta que sean publicos para que se usen, pero para el rectangulo
                  // si
}

impl Describible for Rectangulo {
    fn descripcion(&self) -> String {
        format!("Rectangulo: {}x{}", self.ancho, self.alto)
    }
}

impl Describible for Cancion {
    fn descripcion(&self) -> String {
        format!("Cancion: {} de {}", self.titulo, self.link)
    }
}

pub fn d7() {
    let r = Rectangulo {
        ancho: 30,
        alto: 30,
    };
    let c = Cancion {
        titulo: String::from("Temardo"),
        link: String::from(
            "https://open.spotify.com/track/1DXiYIhcbQZaxE2reM0aOW?si=04c92f03653a497c",
        ),
    };
    println!("{}", r.descripcion());
    println!("{}", c.descripcion());
}
