mod analysis;

use crate::analysis::*;
use rand_xoshiro::rand_core::{RngCore, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use statrs::function::erf::*;
use std::f64::consts::PI;
use std::time::{Duration, Instant};

const BENCHMARK_SIZE: usize = 1_000_000;

const SEED: u64 = 555;

fn main() {
    println!("Hello, world!");

    let mut xoshiro = Xoshiro256PlusPlus::seed_from_u64(SEED);

    let mut vec_loi = vec![0f64; BENCHMARK_SIZE];
    let now = Instant::now();
    for i in 0..BENCHMARK_SIZE {
        vec_loi[i] = f64::sqrt(2f64) * erf_inv(2f64 * normalize(xoshiro.next_u64()) - 1f64);
    }
    print_info(&vec_loi,now.elapsed(),"Methode Inverse");
    analyse(&vec_loi, 0.2f64, "Methode Inverse");


    let mut vec_centrale_limite = vec![0f64; BENCHMARK_SIZE];
    let n = 100;
    let now = Instant::now();
    for i in 0..BENCHMARK_SIZE {
        let mut somme = 0f64;
        for _j in 0..n {
            somme += normalize(xoshiro.next_u64());
        }
        vec_centrale_limite[i] = (somme - n as f64 / 2f64) / f64::sqrt(n as f64 / 12f64)
    }
    print_info(&vec_centrale_limite,now.elapsed(),"Loi Centrale Limite");
    analyse(&vec_centrale_limite, 0.2f64, "Loi Centrale Limite");

    let mut vec_box_muller = vec![0f64; BENCHMARK_SIZE];
    let now = Instant::now();
    for i in 0..(BENCHMARK_SIZE / 2) {
        let x = normalize(xoshiro.next_u64());
        let y = normalize(xoshiro.next_u64());

        vec_box_muller[2 * i] = f64::sqrt(-2f64 * f64::ln(x)) * f64::cos(2f64 * PI * y);
        vec_box_muller[2 * i + 1] = f64::sqrt(-2f64 * f64::ln(x)) * f64::sin(2f64 * PI * y);
    }
    print_info(&vec_box_muller,now.elapsed(),"Methode de Box et Muller");
    analyse(&vec_box_muller, 0.2f64, "Methode de Box et Muller");

    let mut vec_marsaglia = vec![0f64; BENCHMARK_SIZE];
    let now = Instant::now();
    for i in 0..(BENCHMARK_SIZE / 2) {
        let mut x: f64;
        let mut y: f64;
        let mut s: f64;
        loop {
            x = (normalize(xoshiro.next_u64()) - 0.5f64) * 2f64;
            y = (normalize(xoshiro.next_u64()) - 0.5f64) * 2f64;
            s = x * x + y * y;
            if s > 0f64 && s < 1f64 {
                break;
            }
        }
        vec_marsaglia[2 * i] = x * (f64::sqrt((-2f64 * f64::ln(s)) / s));
        vec_marsaglia[2 * i + 1] = y * (f64::sqrt((-2f64 * f64::ln(s)) / s));
    }
    print_info(&vec_marsaglia,now.elapsed(),"Methode de Marsaglia");
    analyse(&vec_marsaglia, 0.2f64, "Methode de Marsaglia");


    let mut vec_laplace = vec![1f64; BENCHMARK_SIZE];
    let now = Instant::now();
    for i in 0..BENCHMARK_SIZE {
        loop {
            let a = normalize(xoshiro.next_u64());
            let u = normalize(xoshiro.next_u64());
            let y = f64::signum(a - 0.5f64)*f64::ln(1f64-2f64*f64::abs(a- 0.5f64));
            let f = f64::exp(-(y*y)/2f64)/ f64::sqrt(2f64 * PI);
            let g = 0.5f64* f64::exp(- f64::abs(y));
            let c = 2f64*f64::sqrt(f64::exp(1f64)/(2f64 * PI));

            if c * g * u <= f {
                if(y == 0f64) {
                    println!("zero");
                }
                vec_laplace[i] = y;
                break;
            }
        }
    }
    print_info(&vec_laplace,now.elapsed(),"Methode du rejet - Laplace");
    analyse(&vec_laplace, 0.2f64, "Methode du rejet - Laplace");

}

fn normalize(input: u64) -> f64 {
    return input as f64 / u64::MAX as f64;
}

fn mean(data: &Vec<f64>) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation(data: &Vec<f64>) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}

fn print_info(vec : &Vec<f64>,dur :Duration,nom : &str) {
    let dev = std_deviation(vec).unwrap();
    let mean = mean(vec).unwrap();
    println!("{} : Esperance: {} , Ecart-type: {} , Temps de calcul: {:?} : {:?} par iter ",nom,mean,dev,dur,dur/BENCHMARK_SIZE as u32)
}