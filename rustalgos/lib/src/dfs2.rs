// attempted to save path for first path to found
// not working
// i need brian to save me
//
include!("E.rs")
include!("graph.rs")
use std::collections::HashMap;

const END: i32 = 6;
// #[derive(Debug, Clone)]
// #[allow(dead_code)]
// pub struct E {
//     weight: i32,
//     node: i32,
// }
fn main() {
    let input_graph = *G;
    let start = E { node: 0, weight: 0 };
    let mut path: Vec<E> = Vec::new();
    // dlet mut path2: Vec<E> = fs(&start, &input_graph, &mut path);
    dfs(&start, &input_graph, &mut path);
    // println!("{:#?}", path)
    for node in path.iter() {
        print!("{}", node.node);
    }
    println!("{}", path.len());
}

//1246
// fn dfs(current_node: E, input_graph: &HashMap<i32, Vec<E>>, path: &mut Vec<E>) -> &mut Vec<E> {
fn dfs(current_node: &E, input_graph: &HashMap<i32, Vec<E>>, path: &mut Vec<E>) {
    // println!("{:#?}", current_node);
    let neighbors: &Vec<E> = input_graph.get(&current_node.node).unwrap();
    path.push(current_node.clone());

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
