use std::collections::{HashSet, VecDeque};

fn main() {
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

    let mut visited: HashSet<u32> = HashSet::new();
    let mut queue: VecDeque<u32> = VecDeque::new();
    queue.push_front(start);
    let mut current_node: usize = 1;

    loop {

        match queue.pop_front() {
            Some(x) => { current_node = x as usize }
            None => panic!("something went wrong"),
        };
        if !visited.contains(&(current_node as u32)) {
            let neighbors: &Vec<u32> = &V[current_node - 1];
            if neighbors.len() == 0 {
                visited.remove(&(current_node as u32));
                queue.pop_front();
            }
            if neighbors.contains(&target) {
                println!("Found Node at {}", current_node);
                std::process::exit(0);
            }
            queue.extend(neighbors.clone());

            println!("neighbors: {:?}", neighbors);
        }
    }

}
