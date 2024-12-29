// Ejercicio: Crea una función que reciba una referencia mutable a un vector
// y duplique todos sus valores
//
pub fn d2_2() {
    fn duplicate(nbrs: &mut Vec<i32>) {
        for nbr in nbrs {

            *nbr *= 2; //*nbr hace una desreferencia a nbr, i.e., accede al valor que guarda nbr
                       //porque cada uno de los nbr es una ref a los vals del array original
        }
    }

    let mut v = vec![1, 2, 3,4,5];
    duplicate(&mut v);
    println!("{:?}", v);
}
