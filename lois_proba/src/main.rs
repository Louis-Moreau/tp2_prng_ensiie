mod analysis;

use rand_xoshiro::rand_core::{SeedableRng,RngCore};
use rand_xoshiro::Xoshiro256PlusPlus;
use statrs::function::erf::*;
use crate::analysis::*;
use std::time::{Instant, Duration};

const BENCHMARK_SIZE : usize = 1000_0000;

const SEED : u64 = 555 ;

fn main() {
    println!("Hello, world!");

    let mut xoshiro = Xoshiro256PlusPlus::seed_from_u64(SEED);

/* 
    let mut vec_loi = vec![0f64;BENCHMARK_SIZE];
    for i in 0..BENCHMARK_SIZE {
        vec_loi[i] =  f64::sqrt(2f64)* erf_inv(2f64*normalize(xoshiro.next_u32())- 1f64);
    }
    analyse(&vec_loi,0.2f64,"Loi Inverse");*/


    let mut vec_centrale_limite = vec![0f64;BENCHMARK_SIZE];
    let n = 1000;
    let now = Instant::now();
    for i in 0..BENCHMARK_SIZE {
        let mut somme = 0f64;
        for _j in 0..n {
            somme += normalize(xoshiro.next_u32());
        }
        vec_centrale_limite[i] =  (somme - n as f64/2f64) / f64::sqrt(n as f64/12f64)
    }
    print!("temps : {:?}",now.elapsed());
    analyse(&vec_centrale_limite,0.2f64,"Loi Centrale Limite");




}

fn normalize(input : u32) -> f64 {
    return input as f64 / u32::MAX as f64;
}