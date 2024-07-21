/* 45

use std::thread;
use std::time::Duration;

fn main() {
    let nombre = String::from("Vicente");
    println!("Hello {}", nombre);
    let nombre_clone = nombre.clone();

    let hndl = thread::spawn( move || {
        thread::sleep(Duration::from_millis(1000));
        println!("Hello from inside the thread {}", nombre);
    });
    println!("{}, just only one thread have finished!", nombre_clone);

    hndl.join().unwrap();

    println!("{}, both threads have already finished!", nombre_clone);
}

*/

/* 46 

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::char;

fn main() {

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn( move || {
        for cnt in 0..10 {
            let mut message = String::from("Thread1 cnt:");
            message.push(char::from_digit(cnt, 10).unwrap());
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    thread::spawn( move || {
        thread::sleep(Duration::from_millis(1000));
        tx2.send(String::from("Thread2")).unwrap();
    });

    for msg in rx {
        println!("{}", msg);
    }
}


*/

/* 47 */

use std::thread;
use std::sync::{Arc, Mutex};

fn main() {

    let mutex = Mutex::new(99);
    let safe_mutex = Arc::new(mutex);
    let mut handles = vec![];

    for _ in 0..3{
        let safe_mutex_copy = Arc::clone(&safe_mutex);
        let handle = thread::spawn(move || {
            let mut number = safe_mutex_copy.lock().unwrap();
            *number += 1;
        });
        handles.push(handle);
    }

    for hndl in handles{
        hndl.join().unwrap();
    }

    println!("Everything is finished! Result: {}", safe_mutex.lock().unwrap());
}


