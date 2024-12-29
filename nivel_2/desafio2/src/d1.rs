pub fn desafio1() {
    /// Exploración de Propiedad    
    /// Ejercicio: Implementa una función que tome dos Strings y las combine,
    /// devolviendo el nuevo String sin invalidar las originales

    fn combinar(a: &str, b: &str) -> String {
        /// la concatenacion de strings requiere un 'owned String' a la izquierda
        return a.to_owned() + b;
    }
    let a = "Hello";
    let b = " World";
    let c = combinar(a, b);
    println!("{c}");
}
