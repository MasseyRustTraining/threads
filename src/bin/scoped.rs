use std::thread;

fn spawn_threads() {
    for i in 0..3 {
        thread::scope(|s| {
            for _ in 0..5 {
                s.spawn(|| {
                    println!("thread {}", i);
                    i
                });
            }
        });
    }
}

fn main() {
    spawn_threads();
}
