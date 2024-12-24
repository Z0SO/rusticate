// Desafío 7: Ordenamiento de un Array
//     Implementa un algoritmo que ordene un array de números enteros (por ejemplo, bubble sort o insertion sort).
//     Pista: Usa bucles anidados y compara elementos adyacentes.

fn bubble_sort(mut param: [i32; 10]) -> [i32; 10]
{
    let mut aux : i32=0;

    for i in 0..param.len()
    {
        println!("Iteración {}", i);
        for j in 0..param.len()-1
        {
            // si el elemento j actual es mayor al siguiente, los intercambia
            if param[j] > param[j+1]
            {
                aux = param[j];
                param[j] = param[j+1];
                param[j+1] = aux;
            }
        }
    }

    return param;
}



fn main() 
{
    let mut array : [i32; 10] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    println!("Array sin ordenar: {:?}", array);

    array = bubble_sort(array);

    println!("Array ordenado: {:?}", array);

    





}
