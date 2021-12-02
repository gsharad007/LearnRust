use core::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let c1 = Rc::new(RefCell::new(0));
    println!("c1 = {:?}", c1);
    let c2 = c1.clone();
    println!("c2 = {:?}", c2);

    for _ in 0..10 {
        let mut m = c1.borrow_mut();
        *m += 1;
    }
    println!("c1 = {:?}", c1);
    println!("c2 = {:?}", c2);
    for _ in 0..10 {
        let mut m = c2.borrow_mut();
        *m += 1;
    }
    println!("c1 = {:?}", c1);
    println!("c2 = {:?}", c2);

    let tsafe = Arc::new(Mutex::new(0));
    let t2 = tsafe.clone();
    let handle = std::thread::spawn(move || {
        for x in 0..10 {
            {
                let mut m = t2.lock().unwrap();
                *m += 1;
                println!("{} thread modding, x = {}", *m, x);
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    });

    for x in 0..10 {
        {
            let mut m = tsafe.lock().unwrap();
            *m += 1;
            println!("{} mainTh modding, x = {}", *m, x);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    handle.join().unwrap();
}
