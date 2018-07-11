extern crate num_rational;
extern crate num_traits;

use num_rational::BigRational;
use std::time::Instant;

fn main() {
    let n: u32 = std::env::args()
        .skip(1)
        .next()
        .expect("provide numbers of iterations")
        .parse()
        .unwrap();

    let mut a = BigRational::from_integer(1_u64.into());
    let mut b = BigRational::new(470_832_u64.into(), 665_857_u64.into());
    let mut t = BigRational::new(1_u64.into(), 4_u64.into());
    let mut p = BigRational::from_integer(1_u64.into());

    let now = Instant::now();
    let half = BigRational::new(1_u64.into(), 2_u64.into());
    let two = BigRational::new(2_u64.into(), 1_u64.into());

    for _ in 0..n {
        let new_a = (&a + &b) * &half;
        let new_b_square = &a * b;
        let new_b = BigRational::new(new_b_square.numer().sqrt(), new_b_square.denom().sqrt());
        let a_diff = a - &new_a;
        let new_t = t - &p * &a_diff * a_diff;
        let new_p = &two * p;

        a = new_a;
        b = new_b;
        t = new_t;
        p = new_p;
    }

    let a_plus_b = a + b;
    let result = &a_plus_b * &a_plus_b / (BigRational::from_integer(4_u64.into()) * t);
    println!(
        "Result: {}\nElapsed time: {}s {}ms.",
        result,
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}
