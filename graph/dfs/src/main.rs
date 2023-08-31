use std::collections::{HashMap, HashSet};

fn main() {

    bfs();



}


fn bfs() {
    let start = 1;
    let target = 8;

    #[allow(non_snake_case)]
    let V: Vec<Vec<u32>> = Vec::from([
        Vec::from([2,4,5]),
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
            Some(x) => {current_node = x as usize}
            None => panic!("something went wrong"),
        };
        if visited.contains(&(current_node as u32)) {
            let neighbors: &Vec<u32> = &V[current_node-1];
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
