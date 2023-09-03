#![allow(non_snake_case)]

use std::fmt::{Display, Debug};

fn main() {
    let V: Vec<Vec<u32>> = Vec::from([
        Vec::from([2, 4, 5]),
        Vec::from([3]),
        Vec::from([7, 8]),
        Vec::from([7]),
        Vec::from([6, 7]),
        Vec::from([]),
        Vec::from([]),
        Vec::from([]),
    ]);

    print2d(&V);
    getMatrix(&mut V);
    print2d(&V);
}

pub fn getMatrix(V: &mut Vec<Vec<u32>>) {
    let y_len = V.len();
    let mut x_len = 0;
    V.iter().map(|x| if x.len() > x_len { x_len = x.len() } );

    for row in V {
        row.extend(vec![0; y_len - row.len()])
    }
    if x_len > y_len {
        for i in 0..(x_len - y_len) {
            let new_vec: Vec<Vec<u32>> = vec::from(vec![0; x_len])
            V.append();
        }
    }
}

pub fn print2d<T: Display>(V: &Vec<Vec<T>>) {
    for row in V {
        println!("{:?}", row);
    }

}