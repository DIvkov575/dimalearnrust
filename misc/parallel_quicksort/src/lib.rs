#![allow(dead_code)]

use std::alloc::{alloc, handle_alloc_error, Layout};
// use std::alloc::{dealloc};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher};
use rand::Rng;


#[macro_export]
macro_rules! harr {
    ($n: expr) => {{
        lib::hashed_gen($n, 0)
    }};
    ($n: expr, $offset: expr) => {{
        lib::hashed_gen($n, $offset)
    }};
}

#[macro_export]
macro_rules! get_arr {
    ($n: expr) => {{
        lib::hashed_gen($n, 0)
    }};
    ($n: expr, $gen: expr) => {{
        if $gen == "malloc" {
            lib::malloc_gen($n)
        } else if $gen == "random" {
            lib::random_gen($n)
        } else {
            lib::hashed_gen($n, 0)
        }
    }};
    ($n: expr, "hashed", $offset: expr) => {{
        lib::hashed_gen($n, $offset)
    }}
}
pub fn malloc_gen(n: usize) -> Vec<usize> {
    unsafe {
        let layout = Layout::array::<usize>(n).unwrap();
        let ptr: *mut usize = alloc(layout).cast();
        if ptr.is_null() { handle_alloc_error(layout); }

        let output: Vec<usize> = Vec::from_raw_parts(ptr, n, n);

        output
    }
    // dealloc(ptr.cast(), layout);
}

pub fn random_gen(n: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let output: Vec<usize> = vec![0; n].iter().map(|_| rng.gen::<usize>() % 100000).collect();
    output
}

pub fn hashed_gen(n: usize, offset: usize) -> Vec<usize> {
    let mut s = DefaultHasher::new();
    let output: Vec<usize> = vec![0usize; n]
        .iter()
        .enumerate()
        .map(|(x,_)| {
            s.write_usize(x + offset);
            (s.finish() % 100000) as usize
        })
        .collect();
    output
}


pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
