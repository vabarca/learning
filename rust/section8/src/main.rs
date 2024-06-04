
/*
CHAPTER 30



#[allow(unused_variables)]
fn main(){

    let mut v: Vec<i32> = Vec::new();
    let tomate: i32 = 'hola: {
        let mut v = vec![1,2,3];
        v.push(5);

        break 'hola 0
    };

    println!("{}", tomate);

    v.push(100);
    v.push(101);
    v.push(102);
    v.push(103);

    let value = v.get(0);

    if value.is_some()
    {
        println!("{}", value.unwrap());
    }

    match value{
        Some(valor)=> println!("{}", valor),
        _ => ()
    }

    let value2 = v[2];
    println!("{}", value2);

    for i in &v{
        println!("{}", i);
    };

    for i in &mut v{
        println!("{}", *i);
    };

    for i in v{
        println!("{}", i);
    };

    enum Mensaje {
        TEXTO(String),
        Error(i32)
    }

    let msg = vec![Mensaje::TEXTO("KK".to_string()), Mensaje::Error(0)];

    for i in msg{
        match i{
            Mensaje::Error(value)=> println!("{}", value),
            Mensaje::TEXTO(value)=> println!("{}", value),
        }
    }
    
}

*/

/*
CHAPTER 31


#[allow(unused_variables)]
fn main(){

    //Strings
    //Strings vs string slice
    // slice: referencia a una contigua secuencia de elementos de un collection
    //Vec<u8>

    //Stringd y string slice son una colección de caracteres, específicamente u8's UTF-8

    //String se guarda en el heap 
    //String slide se guarda en el stack (referencia al heap o string literal en el codigo binario)
    
    let nombre_string = String::from("julio!!"); //Esto está en el heap
    let literal = "julio!!!"; //Esto está en el binario
    let mut nombre2 = literal.to_string();
    nombre2.push('a');

    let nombre3 = &nombre_string[..3];
    let nombre4 = &nombre_string[..nombre_string.len()];


    println!("{}", nombre4);

}
*/

/*
CHAPTER 32



use std::collections::HashSet;

#[allow(unused_variables)]
fn main(){

    //hashset garantiza que no hay elementos duplicados
    let mut user_id: HashSet<i32> = HashSet::new();
    let id: i32 = 150;
    user_id.insert(100);
    user_id.insert(100);
    user_id.insert(100);
    user_id.insert(100);
    user_id.insert(id);
    user_id.remove(&100);
    user_id.insert(13);
    user_id.insert(55);

    for id in user_id.iter(){
        println!("{}", id);
    }
    


    let mut user_id_backup: HashSet<i32> = HashSet::new();
    user_id_backup.insert(100);
    user_id_backup.insert(13);
    user_id_backup.insert(88);
    user_id_backup.insert(9);

    println!("-------Difference--------");

    for id in user_id.difference(&user_id_backup){

        println!("{}", id);
    }

    println!("-------Simetric difference--------");

    for id in user_id.symmetric_difference(&user_id_backup){

        println!("{}", id);
    }

    println!("------Intersection--------");

    for id in user_id.intersection(&user_id_backup){

        println!("{}", id);
    }

    println!("------Union--------");

    for id in user_id.union(&user_id_backup){

        println!("{}", id);
    }


    // union: obtener los elementos unicos entre 2 sets
    // difference: obtener los elementos que están en el primer set y no en el otro
    // intersection: obtener los elementos que estan en ambos sets
    // simetric_differenced: obtener todos los elementos que estan en un set, o en el otro, pero no en ambos
}

*/

/*
CHAPTER 33
*/

use std::collections::HashMap;

#[allow(unused_variables)]
fn main(){

    //hashset garantiza que no hay elementos duplicados
    let mut puntuaciones: HashMap<String, i32> = HashMap::new();
    puntuaciones.insert(String::from("Azul"), 20);
    puntuaciones.insert(String::from("Rojo"), 100);

    for id in puntuaciones.iter(){
        println!("{}, {}", id.0, id.1);
    }

    for (key, value) in puntuaciones.iter(){
        println!("{}, {}", key, value);
    }


    let ptos = puntuaciones.get("Azul");
    let ptos_no_exist = puntuaciones.get("Marron").get_or_insert(&25);
    puntuaciones.entry(String::from("manolo")).or_insert(55);

    match puntuaciones.get("Azul") {
        Some(value) => println!("{}", value),
        _ => (),
    }

    match puntuaciones.get("manolo") {
        Some(value) => println!("{}", value),
        _ => (),
    }


    let a = 20;
    let b = String::from("macarron");

    puntuaciones.insert(b, a);

    

}

