use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Result;

static mut COUNTER: i8 = 0;
#[derive(Debug, Copy, Clone)]
struct E {
    weight: i32,
    node: i32,
}

fn main() {
    #[allow(non_snake_case)]
    let mut V: [i32; 6] = [-1; 6];
    #[allow(non_snake_case)]
    let W = HashMap::from([
        (
            0,
            Vec::from([E { node: 1, weight: 4 }, E { node: 2, weight: 3 }]),
        ),
        (
            1,
            Vec::from([E { node: 2, weight: 5 }, E { node: 3, weight: 6 }]),
        ),
        (2, Vec::from([E { node: 4, weight: 7 }])),
        (3, Vec::from([E { node: 5, weight: 3 }])),
        (4, Vec::from([])),
        (5, Vec::from([])),
    ]);
    let start = 0;
    V[start] = 0;
    #[allow(non_snake_case)]
    let mut Q = Vec::<E>::new();

    dij(start as i32, &mut V, W, &mut Q);
    println!("V: {:#?}", V);
}

#[allow(non_snake_case)]
fn dij(start: i32, V: &mut [i32; 6], W: HashMap<i32, Vec<E>>, Q: &mut Vec<E>) {
    // finds minimum weight edge -> sets Value & index
    let vec = W.get(&start).unwrap().clone();
    Q.extend(vec);

    // ends program if queue empty (all nodes visited)
    if Q.len() == 0 {
        return;
    }

    // select next node
    let mut min_edge = Q[0];
    for (i, edge) in Q.iter().enumerate() {
        if edge.weight < min_edge.weight {
            min_edge = Q[i];
        }
    }

    // -1 signifies end node
    if min_edge.node == -1 {
        panic!("end reached");
    }
    // checks if next node is infinte distance away -> sets value
    // checks if less than existing distance -> sets value
    if V[min_edge.node as usize] == -1 {
        V[min_edge.node as usize] = V[start as usize] + min_edge.weight;
    } else {
        if V[min_edge.node as usize] > V[start as usize] + min_edge.weight {
            V[min_edge.node as usize] = V[start as usize] + min_edge.weight;
        }
    }
    // remove visited node
    Q.remove(Q.iter().position(|x| x.node == min_edge.node).unwrap());

    dij(min_edge.node, V, W, Q);
}
