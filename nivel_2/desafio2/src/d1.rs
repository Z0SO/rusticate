pub fn desafio1() {
    let mut x = String::from("a");
    let y: String = x;
    x = String::from("b");
    println!("{}", y);
    println!("{}", x);
}
