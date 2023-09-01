use std::collections::{HashMap, HashSet};
use lib::{END, E};


fn main() {
    iterative_dfs();
    recursive_dfs_wrapper();
}


fn iterative_dfs() {
    let start = 1;
    let target = 8;

    #[allow(non_snake_case)]
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
    let a: [u32; 8] = (1..=8).collect::<Vec<_>>().try_into().unwrap();
    let mut visited: HashSet<u32> = HashSet::from(a);
    let mut stack: Vec<u32> = Vec::new();
    stack.push(start);
    let mut current_node: usize = 1;


    loop {
        match stack.pop() {
            Some(x) => { current_node = x as usize }
            None => panic!("something went wrong"),
        };
        if visited.contains(&(current_node as u32)) {
            let neighbors: &Vec<u32> = &V[current_node - 1];
            if neighbors.len() == 0 {
                visited.remove(&(current_node as u32));
                stack.pop();
            }
            if neighbors.contains(&target) {
                println!("Found Node at {}", current_node);
                std::process::exit(0);
            }
            stack.splice(stack.len()..stack.len(), neighbors.clone());
            println!("neighbors: {:?}", neighbors);
        }
        println!("current: {}", current_node);
        println!("stack: {:?}", stack);
    }
}

fn recursive_dfs_wrapper() {
    fn recursive_dfs(current_node: &E, input_graph: &HashMap<i32, Vec<E>>, path: &mut Vec<E>) {
        // println!("{:#?}", current_node);
        let neighbors: &Vec<E> = input_graph.get(&current_node.node).unwrap();

        if current_node.node == END {
            println!("foudd node");
            std::process::exit(0x00);
        }
        for node in neighbors.iter() {
            recursive_dfs(node, input_graph, path);
        }

        // if current_node.node == END {
        //     path.push(current_node);
        // }
        // return path;
    }
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
    recursive_dfs(&start, &input_graph, &mut path);
}

