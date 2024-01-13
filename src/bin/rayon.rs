use rayon::prelude::*;

fn main() {
    let mut a = Box::new([0usize; 1_000_000_000]);
    a.par_iter_mut().for_each(|v| {
        *v += 1;
    });
    println!("{}", a[1000]);
}
