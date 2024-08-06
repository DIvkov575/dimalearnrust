use std::collections::{BinaryHeap, HashMap};
include!("E.rs");

fn main() {
    let mut mass_distances: [i32; 6] = [-1; 6];
    let mut queue = BinaryHeap::<E>::new();
    let input_graph = hashmap::from([
        (
            0,
            vec::from([e { node: 1, weight: 4 }, e { node: 2, weight: 3 }]),
        ),
        (
            1,
            vec::from([e { node: 2, weight: 5 }, e { node: 3, weight: 6 }]),
        ),
        (2, vec::from([e { node: 4, weight: 7 }])),
        (3, vec::from([e { node: 5, weight: 3 }])),
        (4, vec::from([])),
        (5, vec::from([])),
    ]);
    // creates starting node
    let start = 0;
    mass_distances[start] = 0;

    // beep boop
    dijkstra(start as i32, &mut mass_distances, input_graph, &mut queue);
    println!("V: {:#?}", mass_distances);
}

fn dijkstra(
    current_node: i32,
    mass_distances: &mut [i32; 6],
    input_graph: HashMap<i32, Vec<E>>,
    queue: &mut BinaryHeap<E>,
) {
    // add new nodes into queue
    for elem in input_graph.get(&current_node).unwrap().iter() {
        queue.push(*elem);
    }

    if queue.len() == 0 {
        return;
    }

    // select next node (min edge)
    let min_edge: E = queue.pop().unwrap().clone();

    // -1 signifies end node
    if min_edge.node == -1 {
        panic!("end reached");
    }
    // checks if next node is infinte distance away -> sets value
    // checks if less than existing distance -> sets value
    if mass_distances[min_edge.node as usize] == -1 {
        mass_distances[min_edge.node as usize] =
            mass_distances[current_node as usize] + min_edge.weight;
    } else {
        if mass_distances[min_edge.node as usize]
            > mass_distances[current_node as usize] + min_edge.weight
        {
            mass_distances[min_edge.node as usize] =
                mass_distances[current_node as usize] + min_edge.weight;
        }
    }

    dijkstra(min_edge.node, mass_distances, input_graph, queue);
}
