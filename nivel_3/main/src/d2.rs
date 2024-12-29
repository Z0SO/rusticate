pub enum Operacion {
    Suma,
    Resta,
    Multiplicacion,
    Division,
}
// se definen los nros como f64 para hacer division con decimales
// si son enteros hace division entera
pub fn d2(a: f64, b: f64, op: Operacion) -> f64 {
    // - Crea una enumeración `Operacion` con variantes como `Suma`, `Resta`, etc.
    // - Escribe una función que tome esta enumeración y dos números, devolviendo el resultado.
    let mut res: f64 = 0f64;
    match op {
        Operacion::Suma => res = a + b,
        Operacion::Resta => res = a - b,
        Operacion::Multiplicacion => res = a * b,
        Operacion::Division => {
            if b == 0f64 {
                println!("No se puede dividir por cero");
                // this is how you return infinities in Rust
                res = f64::INFINITY;
            } else {
                res = a / b;
            };
        }
    }
    res
}
