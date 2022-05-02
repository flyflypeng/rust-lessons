// fn main() {
//     let arr = vec![1];

//     let handler = std::thread::spawn(move || {
//         println!("{:?}", arr);
//     });

//     handler.join().unwrap();
// }
use std::sync::{Arc, Mutex};

fn main() {
    let hello_str = "hello".to_string();

    let hello_str_mutex = Arc::new(Mutex::new(hello_str));
    let hello_str_mutex_clone = Arc::clone(&hello_str_mutex);

    let handler = std::thread::spawn(move || {
        let mut value = hello_str_mutex.lock().unwrap();
        println!("{:?}", value);
        *value = "hello worlld".to_string();
    });

    handler.join().unwrap();

    println!(
        "main, print hello_str: {:?}",
        hello_str_mutex_clone.lock().unwrap()
    );
}
