#![deny(clippy)]
#![allow(many_single_char_names)]

extern crate bigdecimal;

use bigdecimal::BigDecimal;
use std::time::Instant;

fn main() {
    let n: u32 = std::env::args()
        .nth(1)
        .expect("provide numbers of iterations")
        .parse()
        .unwrap();

    let mut a = BigDecimal::from(1_u64);
    let mut b = BigDecimal::from(0.5).sqrt().unwrap();
    let mut t = BigDecimal::from(0.25);
    let mut p = BigDecimal::from(1_u64);

    let now = Instant::now();
    let half = BigDecimal::from(1.0/2.0);
    let two = BigDecimal::from(2_u64);

    for _ in 0..n {
        let new_a = (&a + &b) * &half;
        let new_b_square = &a * b;
        let new_b = new_b_square.sqrt().unwrap();
        let a_diff = a - &new_a;
        let new_t = t - &p * &a_diff * a_diff;
        let new_p = &two * p;

        a = new_a;
        b = new_b;
        t = new_t;
        p = new_p;
    }

    let a_plus_b = a + b;
    let result = &a_plus_b * &a_plus_b / (BigDecimal::from(4_u64) * t);
    println!(
        "Result: {}\nElapsed time: {}s {}ms.",
        result,
        now.elapsed().as_secs(),
        now.elapsed().subsec_millis()
    );
}
