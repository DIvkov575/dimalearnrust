use std::collections::HashMap;
// use std::collections::Queue;
#[allow(dead_code)]
struct E {
    weight: usize,
    node: usize,
}

fn main() {
    #[allow(non_snake_case)]
    let mut v: [i32; 5] = [-1; 5];
    #[allow(non_snake_case)]
    let W = HashMap::from([
        (0, Vec::from([(1, 4), (2, 3)])),
        (1, Vec::from([(2, 5), (3, 6)])),
        (2, Vec::from([(4, 7)])),
        (3, Vec::from([(5, 3)])),
        (4, Vec::from([(-1, 0)])),
        (5, Vec::from([(-1, 0)])),
    ]);
    let start = 0;
    v[start] = 0;

    func(start as u32, &v, W, Vec::new());
}

fn func(start: u32, V: &[i32; 5], W: HashMap<u32, Vec<(i32, i32)>>, Q: Vec<i32>) {
    #[allow(non_snake_case)]
    let tmp: Vec<i32> = W.get(&start).unwrap().iter().map(|x| x.0).collect();
    let weights: Vec<i32> = W.get(&start).unwrap().iter().map(|x| x.1).collect();

    // finds minimum weight edge -> sets Value & index
    let mut index = 0;
    let mut min_weight = weights[0];
    let mut counter = 0;
    for x in weights.iter() {
        if x < &min_weight {
            min_weight = *x;
            index = counter;
        }
        counter += 1;
    }

    let weight = weights[index];
    let next_node = Q[index];
    if next_node == -1 {
        panic!("end reached");
    }

    // checks if next node is infinte distance away -> sets
    // checks if less than existing distance -> sets
    if V[next_node as usize] == -1 {
        V[next_node as usize] = V[start as usize] + weight as i32;
    } else {
        if V[next_node as usize] > V[start as usize] + weight as i32 {
            V[next_node as usize] = V[start as usize] + weight as i32;
        }
    }
    Q.remove(Q.iter().position(|x| x == &next_node).unwrap());
    println!("V: {:#?}", V);
    println!("Q: {:#?}", Q);

    func(next_node as u32, V, W, Q);
}

// fn min_edge(input: &Vec<(i32,u32)>) -> usize {
//     let mut counter:u32 = 0;
//     let mut min_edge:u32 = input[0].1;
//     let mut min_node_index:u32 = 0;
//     for edge in input {
//        if edge.1 < min_edge {
//            min_edge = edge.1;
//            min_node_index = counter;
//        }
//         counter += 1;
//     }
//     min_node_index as usize

// }
// #[allow(non_snake_case)]
// let W = std::collections::HashMap::from([
//     (0,Vec::from([1,2])),
//     (1,Vec::from([2,3])),
//     (2,Vec::from([4])),
//     (3,Vec::from([5])),
//     (4,Vec::from([-1])),
//     (5,Vec::from([-1]))
// ]);
