extern crate num_rational;
extern crate num_traits;
extern crate num_bigint;

use num_rational::Ratio;
use num_bigint::BigInt;
use std::time::Instant;

type BigRat = Ratio<BigInt>;

fn main() {
    let n: u32 = std::env::args().skip(1).next().expect("provide numbers of iterations").parse().unwrap();

    let mut a = BigRat::from_integer(1_u64.into());
    let mut b = BigRat::new(470_832_u64.into(), 665_857_u64.into());
    let mut t = BigRat::new(1_u64.into(), 4_u64.into());
    let mut p = BigRat::from_integer(1_u64.into());

    let now = Instant::now();
    let half = BigRat::new(1_u64.into(), 2_u64.into());
    let two = BigRat::new(2_u64.into(), 1_u64.into());

    for _ in 0..n {
        let new_a = (a.clone() + b.clone())*half.clone();
        let new_b_square = a.clone()*b;
        let new_b = BigRat::new(new_b_square.numer().sqrt(),new_b_square.denom().sqrt());
        let a_diff = a - new_a.clone();
        let new_t = t - p.clone()*a_diff.clone()*a_diff;
        let new_p = two.clone()*p;

        a = new_a;
        b = new_b;
        t = new_t;
        p = new_p;
    }

    let a_plus_b = a+b;
    let result = a_plus_b.clone()*a_plus_b/(BigRat::from_integer(4_u64.into())*t);
    println!("Elapsed time: {}{}ms. Result = {}", now.elapsed().as_secs(), now.elapsed().subsec_millis(), result);
}
