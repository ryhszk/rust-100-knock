extern crate num;
use num::Complex;

// Complex is is generic structre.
// T means for all types.
struct Complex<T> {
    // Real portion of the complex number
    re: T,

    // Imaginary portion of the complex number
    im: T
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) [
    let mut z = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
]

fn main() {
    println!("Hello, world!");
}