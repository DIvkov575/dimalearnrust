#![allow(non_snake_case)]

use std::fmt::{Display, Debug};

fn main() {
    let mut V: Vec<Vec<u32>> = Vec::from([
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

    if y_len >= x_len {
        for row in (&mut V).iter() {
            row.extend(vec![0; y_len - row.len()])
        }
    } else {
        for row in (&mut V).iter() {
            row.extend(vec![0; x_len - row.len()])
        }

        let new_vec: Vec<Vec<u32>> = vec![vec![0; x_len]; x_len-y_len];
        V.extend(new_vec);
    }
}

pub fn print2d<T: Display + std::fmt::Debug>(V: &Vec<Vec<T>>) {
    for row in V {
        println!("{:?}", row);
    }

}