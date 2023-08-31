use std::collections::HashMap;

const END: i32 = 6;
#[derive(Debug)]
#[allow(dead_code)]
pub struct E {
    weight: i32,
    node: i32,
}
fn main() {
    let input_graph: HashMap<i32, Vec<E>> = HashMap::from([
        (
            0,
            Vec::from([
                E { node: 1, weight: 4 },
                E { node: 2, weight: 3 },
                E { node: 3, weight: 2 },
            ]),
        ),
        (
            1,
            Vec::from([E { node: 2, weight: 5 }, E { node: 3, weight: 6 }]),
        ),
        (
            2,
            Vec::from([E { node: 4, weight: 7 }, E { node: 6, weight: 9 }]),
        ),
        (
            3,
            Vec::from([E { node: 5, weight: 3 }, E { node: 6, weight: 3 }]),
        ),
        (
            4,
            Vec::from([E { node: 6, weight: 5 }, E { node: 7, weight: 4 }]),
        ),
        (5, Vec::from([])),
        (6, Vec::from([])),
        (7, Vec::from([])),
    ]);

    let start = E { node: 0, weight: 0 };
    let mut path: Vec<E> = Vec::new();
    dfs(&start, &input_graph, &mut path);
}

//1246
// fn dfs(current_node: E, input_graph: &HashMap<i32, Vec<E>>, path: &mut Vec<E>) -> &mut Vec<E> {
fn dfs(current_node: &E, input_graph: &HashMap<i32, Vec<E>>, path: &mut Vec<E>) {
    // println!("{:#?}", current_node);
    let neighbors: &Vec<E> = input_graph.get(&current_node.node).unwrap();

    if current_node.node == END {
        println!("foudd node");
        std::process::exit(0x00);
    }
    for node in neighbors.iter() {
        dfs(node, input_graph, path);
    }

    // if current_node.node == END {
    //     path.push(current_node);
    // }
    // return path;
}
