use std::sync::{Arc, RwLock};
use std::thread;

fn spawn_threads() -> Vec<thread::JoinHandle<usize>> {
    let global_i = Arc::new(RwLock::new(0usize));
    let mut handles = Vec::with_capacity(5);
    for _ in 0..5 {
        let global_i = Arc::clone(&global_i);
        let handle = thread::spawn(move || {
            let mut i = global_i.write().unwrap();
            *i += 1;
            let i0 = *i;
            drop(i);
            println!("thread {}", i0);
            i0
        });
        handles.push(handle);
    }
    handles
}

fn main() {
    let handles = spawn_threads();
    let nhandles = handles.len();
    let mut values = std::collections::HashSet::new();
    for h in handles {
        values.insert(h.join().unwrap());
    }
    for i in 1..=nhandles {
        assert!(values.contains(&i));
    }
}
    
