// use day_21::part2::*;

pub fn main() {
    let mut i: u64 = 196;
    let mut x: u64 = 33695;
    let mut v: u64 = 29917;
    let a: u64 = 29826;

    for _ in 0..202299 {
        v += a;
        x += v;
        i += 131;
        println!("x: {}, v: {}, a: {}, i: {}", x, v, a, i);
    }
}
