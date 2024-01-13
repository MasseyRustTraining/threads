use std::sync::RwLock;
use std::thread;

static I: RwLock<usize> = RwLock::new(0usize);

fn spawn_threads() -> Vec<thread::JoinHandle<usize>> {
    let mut handles = Vec::with_capacity(5);
    for _ in 0..5 {
        let handle = thread::spawn(|| {
            let mut i = I.write().unwrap();
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
    for h in handles {
        assert!((1..=5).contains(&h.join().unwrap()));
    }
}
    
