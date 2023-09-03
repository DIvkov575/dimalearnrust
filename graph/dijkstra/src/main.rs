#![allow(non_snake_case)]

use std::convert::TryInto;

fn main() {
    let W: Vec<Vec<u32>> = Vec::from([
        Vec::from([2, 4, 5, 0, 0, 0, 0, 0,]),
        Vec::from([3, 0, 0, 0, 0, 0, 0, 0,]),
        Vec::from([7, 8, 0, 0, 0, 0, 0, 0,]),
        Vec::from([7, 0, 0, 0, 0, 0, 0, 0,]),
        Vec::from([6, 7, 0, 0, 0, 0, 0, 0,]),
        Vec::from([0, 0, 0, 0, 0, 0, 0, 0,]),
        Vec::from([0, 0, 0, 0, 0, 0, 0, 0,]),
        Vec::from([0, 0, 0, 0, 0, 0, 0, 0,]),
    ]);
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
    let start = 1;
    iterative_dijkstra(V, W, start);
}

fn iterative_dijkstra(V: Vec<Vec<u32>>, W: Vec<Vec<u32>>, start: u32) {
    let mut distances: Vec<u32> = vec![u32::MAX; V.len()];
    let mut stack: Vec<u32> = Vec::from([start]);
    let mut current_node: usize = 1;
    let mut ctr = 0;


    loop {
        current_node = stack.pop().unwrap().try_into().unwrap();
        if ctr < distances[current_node - 1] {
            distances[current_node - 1] = ctr;
            let neighbors: &Vec<u32> = &V[current_node - 1];

            if neighbors.is_empty() {
                println!("go crazy {ctr}");
                stack.pop();
                ctr = 0;
            } else {
                stack.splice(stack.len()..stack.len(), neighbors.clone());
                println!("{:?}", W[current_node - 1][(stack[stack.len() - 1] - 1) as usize]);
                ctr += W[current_node - 1][(stack[stack.len() - 1] - 1) as usize];
            }
        }

         if stack.is_empty() {
             print!("{:?}", distances);
             std::process::exit(0);
         }
    }



}
