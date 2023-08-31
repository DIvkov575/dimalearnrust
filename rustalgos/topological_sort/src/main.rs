use lib::E;

static mut COUNTER: i8 = 0;
static mut V: [i8; 8] = [127; 8];

fn main() {
    #[allow(non_snake_case)]
    let G = [
        Vec::from([
            E { node: 1, weight: 4 },
            E { node: 2, weight: 3 },
            E { node: 3, weight: 2 },
        ]),
        Vec::from([E { node: 2, weight: 5 }, E { node: 3, weight: 6 }]),
        Vec::from([E { node: 4, weight: 7 }, E { node: 6, weight: 9 }]),
        Vec::from([E { node: 5, weight: 3 }, E { node: 6, weight: 3 }]),
        Vec::from([E { node: 6, weight: 5 }, E { node: 7, weight: 4 }]),
        Vec::from([]),
        Vec::from([]),
        Vec::from([]),
    ];

    let current_node = E { node: 0, weight: 0 };
    let mut queue: Vec<E> = Vec::from([current_node]);
    unsafe { V[current_node.node as usize] = COUNTER }

    bfs(&current_node, &G, &mut queue);

    unsafe {
        print!("{:?}", V);
    }
}

fn bfs(current_node: &E, input_graph: &[Vec<E>; 8], queue: &mut Vec<E>) {
    queue.remove(0);
    for node in &input_graph[current_node.node as usize] {
        unsafe {
            if V[node.node as usize] == 127 {
                COUNTER += 1;
                V[node.node as usize] = COUNTER;
                queue.push(*node);
            }
        }
    }
    if queue.len() == 0 {
        return;
    }
    bfs(&queue[0], input_graph, &mut queue.clone());
}
