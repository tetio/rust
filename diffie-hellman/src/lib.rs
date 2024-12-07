use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow(g,a) % p
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow(b_pub,a) % p
}


fn pow(base: u64, exp: u64) -> u64{
    match exp {
        0 => 1u64,
        _ => (1..=exp).into_iter().fold(1, |acc, _| {acc.checked_mul(base).unwrap()})
    }
}