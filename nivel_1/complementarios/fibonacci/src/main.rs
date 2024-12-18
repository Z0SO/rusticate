// Desafío 3: Fibonacci Recursivo
//     Crea una función recursiva que calcule el n-ésimo número de la secuencia de Fibonacci.
//     Pista: La secuencia empieza con 0, 1, 1, 2, 3, 5, .... La fórmula es F(n) = F(n-1) + F(n-2).

fn fibonacci(param: i64) -> i64 
{
    if param == 1  
    {
        return 1
    }
    else if param ==  0
    {
        return 0
    }else
    {
        fibonacci(param-1)+fibonacci(param-2)
    }
}



fn main() 
{
   println!("{}",fibonacci(221)); 
}
