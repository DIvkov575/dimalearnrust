#![allow(non_snake_case)]

use std::error::Error;
use std::time::Instant;

use itertools::Itertools;
use rayon::prelude::*;

#[allow(special_module_name)]
mod lib;


fn main() -> Result<(), Box<dyn Error>> {
    let mut timer;
    let input_a: Vec<usize> = harr!(1*(10usize).pow(5), 42);
    let input_b = input_a.clone();
    let input_c= input_a.clone();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(8).build().unwrap();

    // comparison of various sorting algos
    // entirely serial
    timer = Instant::now();
    let _output1 = q_sort(input_a);
    let t1 = timer.elapsed();
    // serial partition
    timer = Instant::now();
    let _output1 = pool.install(|| q_sort_parallel1(input_b));
    let t2 = timer.elapsed();
    // sequential partition
    timer = Instant::now();
    let _output2 = pool.install(|| q_sort_parallel2(input_c));
    let t3 = timer.elapsed();


    println!("Output1: {:.3?}", t1);
    println!("Output2: {:.3?}", t2);
    println!("Output3: {:.3?}", t3);



    // comparison of serial vs parallel partition

    // timer = Instant::now();
    // let pivot = input[input.len()/2];
    // let mut output: Vec<usize> = Vec::with_capacity(input.len());
    // let mut L: Vec<usize> = Vec::with_capacity(input.len() / 2);
    // let mut M: Vec<usize> = Vec::with_capacity(1);
    // let mut R: Vec<usize> = Vec::with_capacity(input.len() / 2);
    // for i in 0..input.len() {
    //     if input[i] < pivot { L.push(input[i]);
    //     } else if input[i] > pivot { R.push(input[i]);
    //     } else { M.push(input[i]); }
    // }
    // println!("Output2: {:.3?}", timer.elapsed());

    // timer = Instant::now();
    // let (_, _, _) = serialized_partition(pivot, &input);
    // println!("Output2: {:.3?}", timer.elapsed());


    Ok(())
}

fn parallel_partition(pivot: usize, input: &Vec<usize>) -> (Vec<usize>, Vec<usize>, Vec<usize>){
    let L: Vec<usize> = Vec::new();
    let M: Vec<usize> = Vec::new();
    let R: Vec<usize> = Vec::new();
    let split_num = 4;
    let split_size = input.len()/split_num;

    let chunks: Vec<Vec<usize>> = input.chunks(split_size).map(|x| x.to_vec()).collect();
    let _ = chunks.par_iter().map( |chunk| {
        let mut L: Vec<usize> = Vec::new();
        let mut M: Vec<usize> = Vec::new();
        let mut R: Vec<usize> = Vec::new();
        for i in 0..chunk.len() {
            if chunk[i] < pivot {
                L.push(chunk[i]);
            } else if chunk[i] > pivot {
                R.push(chunk[i]);
            } else {
                M.push(chunk[i]);
            }
        }
        return (L, M, R);
    } );

    return (L, M, R)
}

fn q_sort_parallel2(input: Vec<usize>) -> Vec<usize> {
    let pivot = input[input.len()/2];
    let mut output: Vec<usize> = Vec::with_capacity(input.len());
    let mut L: Vec<usize> = Vec::with_capacity(input.len() / 2);
    let mut M: Vec<usize> = Vec::with_capacity(1);
    let mut R: Vec<usize> = Vec::with_capacity(input.len() / 2);

    (L, M, R) = parallel_partition(pivot, &input);

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



fn q_sort_parallel1(input: Vec<usize>) -> Vec<usize> {
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
            || q_sort_parallel1(L),
            || q_sort_parallel1(R)
        );

        output.append(&mut l_handle);
        output.append(&mut M);
        output.append(&mut r_handle);
    } else if L.len() > 1 && R.len() <= 1 {
        output.append(&mut q_sort_parallel1(L));
        output.append(&mut M);
        output.append(&mut R);
    } else if L.len() <= 1 && R.len() > 1 {
        output.append(&mut L);
        output.append(&mut M);
        output.append(&mut q_sort_parallel1(R));
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


