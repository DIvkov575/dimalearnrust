// bfs
include! {"lib.rs"}
static mut VISITED: [bool; 6] = [false; 6];
static EP: u32 = 5;

fn main() {
    let input_graph: [Vec<E>; 6] = [
        Vec::from([E { node: 1, weight: 4 }, E { node: 2, weight: 3 }]),
        Vec::from([E { node: 2, weight: 5 }, E { node: 3, weight: 6 }]),
        Vec::from([E { node: 4, weight: 7 }]),
        Vec::from([E { node: 5, weight: 3 }]),
        Vec::from([]),
        Vec::from([]),
    ];
    let starting_node = E { node: 0, weight: 0 };
    let mut queue: Vec<E> = Vec::from([starting_node]);
    bfs(&starting_node, &input_graph, &mut queue);
}

fn bfs(current_node: &E, input_graph: &[Vec<E>; 6], queue: &mut Vec<E>) {
    println!("{:?}", queue);
    queue.remove(0);
    for node in &input_graph[current_node.node as usize] {
        unsafe {
            if node.node == EP {
                println!("found node");
                std::process::exit(0);
            }
            if VISITED[node.node as usize] == false {
                VISITED[node.node as usize] = true;
                queue.push(*node);
            }
        }
    }
    bfs(&queue[0], input_graph, &mut queue.clone());
}
