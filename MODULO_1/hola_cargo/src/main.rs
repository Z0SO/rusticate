fn main() {
    let mut number: i64 = 600851475143; // Asegúrate de especificar que es de tipo i64
    let mut largest_prime_factor = 0;

    let mut factor = 2;
    while factor * factor <= number {
        if number % factor == 0 {
            largest_prime_factor = factor;
            number /= factor;
        } else {
            factor += 1;
        }
    }

    if number > largest_prime_factor {
        largest_prime_factor = number;
    }

    println!("El factor primo más grande es: {}", largest_prime_factor);
}
