/*
CHAPTER 34
ownership = propiedad quien es dueño de
borrowing = pedir prestado


fn main() {
    let name = String::from("julio");
    let name_copy = name;
    println!("{}", name_copy);
    println!("{}", name);
}

*/

/*
CHAPTER 35
Lifetimes = timepo de vida de las referenicas a memoria
Lifetimes son una forma de asegurar que un pedazo de memeoria es aún valida para un referencia
borrowing = pedir prestado
*/

fn main() {

}

fn hace_algo<'a, 'b>(param: &'a i32, param_b: &'b str) -> &'a i32{
    param
}

fn hace_algo_igual(param: &i32, param_b: &str) -> i32{
    32
}

