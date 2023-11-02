#![allow(non_snake_case)]

#[allow(special_module_name)]
mod lib;

use lib::print_type_of;
use std::thread;
use std::error::Error;
use std::thread::{spawn, JoinHandle};
use std::sync::Arc;
use std::time::{Duration, Instant};


fn main() -> Result<(), Box<dyn Error>> {
    let mut timer;
    let input: Vec<usize> = harr!(1*(10usize).pow(7), 42);
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();


    // timer = Instant::now();
    // let output1 = q_sort(input.clone());
    // let t1 = timer.elapsed();
    // println!("Output1: {:.3?}", t1);

    // timer = Instant::now();
    // let output1 = pool.install(|| q_sort_parallel2(input));
    // let t2 = timer.elapsed();
    // println!("Output2: {:.3?}", t2);

    timer = Instant::now();
    let pivot = input[input.len()/2];
    let mut output: Vec<usize> = Vec::with_capacity(input.len());
    let mut L: Vec<usize> = Vec::with_capacity(input.len() / 2);
    let mut M: Vec<usize> = Vec::with_capacity(1);
    let mut R: Vec<usize> = Vec::with_capacity(input.len() / 2);
    for i in 0..input.len() {
        if input[i] < pivot {
            L.push(input[i]);
        } else if input[i] > pivot {
            R.push(input[i]);
        } else {
            M.push(input[i]);
        }
    }
    println!("Output2: {:.3?}", timer.elapsed());



    timer = Instant::now();
    let (_, _, _) = serialized_partition(pivot, &input);
    println!("Output2: {:.3?}", timer.elapsed());


    // println!("Dif: {:?}", t1.as_millis() as f64 /t2.as_millis() as f64 );
    Ok(())
}

fn serialized_partition(pivot: usize, input: &Vec<usize>) -> (Vec<usize>, Vec<usize>, Vec<usize>){
    let mut L: Vec<usize> = Vec::new();
    let mut M: Vec<usize> = Vec::new();
    let mut R: Vec<usize> = Vec::new();
    let split: usize = input.len()/2;
    let ( (
            mut L1,
            mut M1,
            mut R1),
        (
            mut L2,
            mut M2,
            mut R2)
    ) = rayon::join(|| {
                        let mut L: Vec<usize> = Vec::new();
                        let mut M: Vec<usize> = Vec::new();
                        let mut R: Vec<usize> = Vec::new();
                        for i in 0..split{
                            if input[i] < pivot {
                                L.push(input[i]);
                            } else if input[i] > pivot{
                                R.push(input[i]);
                            } else {
                                M.push(input[i]);
                            }
                        }
                    return (L, M, R);
                    },
                || {
                    let mut L: Vec<usize> = Vec::new();
                    let mut M: Vec<usize> = Vec::new();
                    let mut R: Vec<usize> = Vec::new();
                    for i in split..input.len(){
                        if input[i] < pivot {
                            L.push(input[i]);
                        } else if input[i] > pivot{
                            R.push(input[i]);
                        } else {
                            M.push(input[i]);
                        }
                    }
                    return (L, M, R);
                }
    );

    L.append(&mut L1);
    L.append(&mut L2);
    M.append(&mut M1);
    M.append(&mut M2);
    R.append(&mut R1);
    R.append(&mut R2);

    return (L, M, R)
}

fn q_sort_parallel2(input: Vec<usize>) -> Vec<usize> {
    let pivot = input[input.len()/2];
    let mut output: Vec<usize> = Vec::with_capacity(input.len());
    let mut L: Vec<usize> = Vec::with_capacity(input.len() / 2);
    let mut M: Vec<usize> = Vec::with_capacity(1);
    let mut R: Vec<usize> = Vec::with_capacity(input.len() / 2);

    (L, M, R) = serialized_partition(pivot, &input);

    // for i in 0..input.len() {
    //     if input[i] < pivot {
    //         L.push(input[i]);
    //     } else if input[i] > pivot {
    //         R.push(input[i]);
    //     } else {
    //         M.push(input[i]);
    //     }
    // }

    if L.len() > 1 && R.len() > 1 {
        let (mut l_handle, mut r_handle) = rayon::join(
            || q_sort_parallel2(L),
            || q_sort_parallel2(R)
        );

        output.append(&mut l_handle);
        output.append(&mut M);
        output.append(&mut r_handle);
    } else if L.len() > 1 && R.len() <= 1 {
        output.append(&mut q_sort_parallel2(L));
        output.append(&mut M);
        output.append(&mut R);
    } else if L.len() <= 1 && R.len() > 1 {
        output.append(&mut L);
        output.append(&mut M);
        output.append(&mut q_sort_parallel2(R));
    } else {
        output.append(&mut L);
        output.append(&mut M);
        output.append(&mut R);
    }

    output
}



fn q_sort_parallel1(input: Vec<usize>, pool: Arc<rayon::ThreadPool>) -> Vec<usize> {
    let pivot = input[input.len()/2];
    let mut output: Vec<usize> = Vec::with_capacity(input.len());
    let mut L: Vec<usize> = Vec::with_capacity(input.len() / 2);
    let mut M: Vec<usize> = Vec::with_capacity(1);
    let mut R: Vec<usize> = Vec::with_capacity(input.len() / 2);

    for i in 0..input.len() {
        if input[i] < pivot {
            L.push(input[i]);
        } else if input[i] > pivot {
            R.push(input[i]);
        } else {
            M.push(input[i]);
        }
    }

    if L.len() > 1 && R.len() > 1 {
        let (mut l_handle, mut r_handle) = rayon::join(
            || q_sort_parallel1(L, pool.clone()),
            || q_sort_parallel1(R, pool.clone())
        );

        output.append(&mut l_handle);
        output.append(&mut M);
        output.append(&mut r_handle);
    } else if L.len() > 1 && R.len() <= 1 {
        output.append(&mut q_sort_parallel1(L, pool));
        output.append(&mut M);
        output.append(&mut R);
    } else if L.len() <= 1 && R.len() > 1 {
        output.append(&mut L);
        output.append(&mut M);
        output.append(&mut q_sort_parallel1(R, pool));
    } else {
        output.append(&mut L);
        output.append(&mut M);
        output.append(&mut R);
    }

    output
}
fn q_sort(input: Vec<usize>) -> Vec<usize> {
    let pivot = input[input.len() / 2];
    let mut output: Vec<usize> = Vec::with_capacity(input.len());
    let mut L: Vec<usize> = Vec::with_capacity(input.len() / 2);
    let mut M: Vec<usize> = Vec::with_capacity(1);
    let mut R: Vec<usize> = Vec::with_capacity(input.len() / 2);

    for i in 0..input.len() {
        if input[i] < pivot {
            L.push(input[i]);
        } else if input[i] > pivot {
            R.push(input[i]);
        } else {
            M.push(input[i]);
        }
    }

    if L.len() > 1 {
        output.append(&mut q_sort(L));
    } else { output.append(&mut L); }

    output.append(&mut M);

    if R.len() > 1 {
        output.append(&mut q_sort(R));
    } else { output.append(&mut R); }

    output
}


