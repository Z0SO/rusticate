mod d1;
mod d2;
mod d7;
pub use crate::d1::d1;
pub use crate::d2::d2;
pub use crate::d7::d7;

fn main() {
    // let a = d2(1f64, 3f64, d2::Operacion::Division);
    // println!("{}", a);
    // Output: 0.3333333333333333 
    // Output: input: 1, 0, division. No se puede dividir por cero inf 
    d7();
    // Output: Rectangulo: 30x30 Cancion: Temardo de https://open.spotify.com/track/1DXiYIhcbQZaxE2reM0aOW?si=04c92f03653a497c 
}
