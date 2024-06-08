/*
CHAPTER 36

samrt pointers 
Tienen reference counter

Smart pointers son usualmente implementados usando structs,
pero implementando los traists Deref y Drop
Deref permite a las instancias de smart pointer comportarse como referenicas
para que el mismo codigo que funciona con referenicas, funcione con smart pointes
Drop trait permite definir lógicas que se ejecutan un vez que el smart
pointer sale del scope

Box<T> mueve datos del stack al heap

*/

use std::ops::Deref;
use std::ops::Drop;

fn main() {

    let x: i32 = 5;
    let y: Micaja<i32> = Micaja::new(2);

    println!("{}", x);
}


struct Micaja<T>(T);

impl<T> Micaja<T>{
    fn new(x:T)->Micaja<T>{
        Micaja(x)
    }
}

impl<T> Deref for Micaja<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for Micaja<T>{
    fn drop(&mut self) {
        println!("Adios")
    }
}


