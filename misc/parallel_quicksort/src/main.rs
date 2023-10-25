#![allow(non_snake_case)]
#[allow(special_module_name)]
mod lib;
use std::thread;
use std::error::Error;
use std::thread::{spawn, JoinHandle};
use std::time::{Duration, Instant};


fn main() -> Result<(), Box<dyn Error>> {
    let input: Vec<usize> = harr!(10, 42);
    println!("Input: {:?}", input);

    let mut timer = Instant::now();
    let output1 = q_sort(input.clone());
    println!("q_sort1: {:#?}", timer.elapsed());
    println!("Output1: {:?}", output1);

    timer = Instant::now();
    let output1 = q_sort_parallel(input);
    println!("q_sort2: {:#?}", timer.elapsed());
    println!("Output2: {:?}", output1);



    Ok(())
}



fn q_sort_parallel(input: Vec<usize>) -> Vec<usize> {
    let mut output: Vec<usize> = Vec::new();
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

    if L.len() > 1 && R.len() > 1 {
        let l_handle = spawn(|| q_sort_parallel(L));
        let r_handle = spawn(|| q_sort_parallel(R));

        output.append(&mut l_handle.join().unwrap());
        output.append(&mut M);
        output.append(&mut r_handle.join().unwrap());
    } else if L.len() > 1  && R.len() <= 1{
        output.append(&mut q_sort_parallel(L));
        output.append(&mut M);
        output.append(&mut R);
    } else if L.len() <= 1  && R.len() > 1{
        output.append(&mut L);
        output.append(&mut M);
        output.append(&mut q_sort_parallel(R));
    } else {
        output.append(&mut L);
        output.append(&mut M);
        output.append(&mut R);
    }

    output
}



fn q_sort(input: Vec<usize>) -> Vec<usize> {
    let len = input.len();
    let pivot = input[len-1];

    let mut L: Vec<usize>  = Vec::with_capacity(len/2);
    let mut M: Vec<usize>  = Vec::with_capacity(1);
    let mut R: Vec<usize>  = Vec::with_capacity(len/2);

    for i in 0..len {
        if input[i]  < pivot {
            L.push(input[i]);
        } else if input[i] > pivot {
            R.push(input[i]);
        } else {
            M.push(input[i]);
        }
    }

    // println!("pivot: {:?}", pivot);
    // println!("Left: {:?}", L);
    // println!("Middle: {:?}", M);
    // println!("Right: {:?}", R);

    let mut output: Vec<usize> = Vec::new();
    if L.len() > 1 {
        output.append(&mut q_sort(L));
    } else {
        output.append(&mut L);
    }

    output.append(&mut M);

    if R.len() > 1 {
        output.append(&mut q_sort(R));
    } else {
        output.append(&mut R);
    }

    // return output;


    // output.append(&mut L);
    // output.append(&mut M);
    // output.append(&mut R);
    output

}


