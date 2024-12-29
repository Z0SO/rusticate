pub struct Rectangulo {
    pub ancho: u32,
    pub alto: u32,
}
// Asi se define un metodo para una estructura
impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }
    fn es_cuadrado(&self) -> bool {
        self.ancho == self.alto
    }
}

pub fn d1() {
    let r = Rectangulo {
        ancho: 30,
        alto: 30,
    };
    println!("Area: {}", r.area());
    println!("Es cuadrado: {}", r.es_cuadrado());
}
