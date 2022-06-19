use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    let a = 10;
    let b = Box::new(20); // <--- Integer on heap also known as boxed integer
    let c = Rc::new(Box::new(30)); // <--- Boxed integer wrapped within a reference counter
    let d = Arc::new(Mutex::new(40)); // <-- Integer wrapped in an atomic tef counter and
                                      // protected by a mutual exclusion lock
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?} ", a, b, c, d);
}
