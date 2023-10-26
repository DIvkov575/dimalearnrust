#![allow(non_snake_case)]
#[allow(special_module_name)]
mod lib;
use lib::print_type_of;
use std::thread;
use std::error::Error;
use std::thread::{spawn, JoinHandle};
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::time::{Duration, Instant};
// use rayon::rayon_core::thread_pool::ThreadPool;

// lazy_static! {
    // static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
    // static ref POOL: Mutex<rayon_core::thread_pool::ThreadPool> = Mutex::new(rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap());
// }


fn main() -> Result<(), Box<dyn Error>> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    // print_type_of(&pool);

    let input: Vec<usize> = harr!((10usize).pow(6), 42);
    // println!("Input: {:?}", input);

    let mut timer = Instant::now();
    let output1 = q_sort(input.clone());
    let t1 = timer.elapsed();
    println!("Output1: {:.3?}", t1);


    timer = Instant::now();
    // let output1 = POOL.lock().install(|| q_sort_parallel(input));
    let output1 = pool.install(|| q_sort_parallel(input, &pool));
    let t2 = timer.elapsed();
    println!("Output2: {:.3?}", t2);

    println!("diff: {:.3?}%", 100.0 - (t1.as_millis() as f64)/(t2.as_millis() as f64)*100.0);
    Ok(())
}


fn q_sort_parallel(input: Vec<usize>, pool: &rayon::ThreadPool) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::with_capacity(input.len());
    let mut L: Vec<usize> = Vec::with_capacity(input.len() /2);
    let mut M: Vec<usize> = Vec::with_capacity(1);
    let mut R: Vec<usize> = Vec::with_capacity(input.len() /2);
    // let mut L: Vec<usize> = Vec::new();
    // let mut M: Vec<usize> = Vec::new();
    // let mut R: Vec<usize> = Vec::new();

    for i in 0..input.len() {
        if input[i]  < input[input.len() -1] {
            L.push(input[i]);
        } else if input[i] > input[input.len() -1] {
            R.push(input[i]);
        } else {
            M.push(input[i]);
        }
    }

    if L.len() > 1 && R.len() > 1 {
        // let (mut l_handle, mut r_handle) = rayon::join(|| q_sort_parallel(L), || q_sort_parallel(R));
        let mut l_handle = pool.install(|| q_sort_parallel(L, &pool));
        let mut R = q_sort_parallel(R, &pool);

        output.append(&mut l_handle);
        output.append(&mut M);
        output.append(&mut R);
    } else if L.len() > 1  && R.len() <= 1{
        output.append(&mut q_sort_parallel(L, &pool));
        output.append(&mut M);
        output.append(&mut R);
    } else if L.len() <= 1  && R.len() > 1{
        output.append(&mut L);
        output.append(&mut M);
        output.append(&mut q_sort_parallel(R, &pool));
    } else {
        output.append(&mut L);
        output.append(&mut M);
        output.append(&mut R);
    }

    output
}


fn q_sort(input: Vec<usize>) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::with_capacity(input.len());
    let mut L: Vec<usize>  = Vec::with_capacity(input.len() /2);
    let mut M: Vec<usize>  = Vec::with_capacity(1);
    let mut R: Vec<usize>  = Vec::with_capacity(input.len() /2);

    for i in 0..input.len() {
        if input[i]  < input[input.len() -1] {
            L.push(input[i]);
        } else if input[i] > input[input.len() -1] {
            R.push(input[i]);
        } else {
            M.push(input[i]);
        }
    }

    if L.len() > 1 { output.append(&mut q_sort(L));
    } else { output.append(&mut L); }
    output.append(&mut M);
    if R.len() > 1 { output.append(&mut q_sort(R));
    } else { output.append(&mut R); }

    output
}


