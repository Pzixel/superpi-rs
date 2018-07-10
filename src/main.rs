extern crate num_rational;
extern crate num_traits;
use num_rational::BigRational;
use num_traits::cast::FromPrimitive;
use std::time::Instant;

fn main() {
    let n: u32 = std::env::args().skip(1).next().expect("provide numbers of iterations").parse().unwrap();

    let mut a = BigRational::from_i32(1).unwrap();
    let mut b = BigRational::from_f64(std::f64::consts::FRAC_1_SQRT_2).unwrap();
    let mut t = BigRational::new(1.into(), 4.into());
    let mut p = BigRational::from_i32(1).unwrap();
    let two = BigRational::from_i32(2).unwrap();

    let now = Instant::now();

    for _ in 0..n {
        let new_a = (a + b)/two;
        let new_b = (a*b).sqrt();
        let a_diff = a - new_a;
        let new_t = t - p*a_diff*a_diff;
        let new_p = two*p;

        a = new_a;
        b = new_b;
        t = new_t;
        p = new_p;
    }

    println!("Elapsed time: {}{}ms", now.elapsed().as_secs(), now.elapsed().subsec_millis());
}
