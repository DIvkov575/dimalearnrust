// use std::collections::Queue;
#[allow(dead_code)]
struct E {
    weight: usize,
    node: usize
}

fn main() {
    #[allow(non_snake_case)]
    let mut V: [i8; 5] = [-1; 5];
    #[allow(non_snake_case)]
    let W = std::collections::HashMap::from([
        (0,Vec::from([(1,4),(2,3)])),
        (1,Vec::from([(2,5),(3,6)])),
        (2,Vec::from([(4, 7)])),
        (3,Vec::from([(5, 3)])),
        (4,Vec::from([(-1,0)])),
        (5,Vec::from([(-1,0)]))
    ]);


    let start = 0;
    // let end = 5;
    V[start] = 0;

    #[allow(non_snake_case)]
    let Q: Vec<i8> = W.get(&start).unwrap().iter().map(|x| x.0).collect();
    let index = min_edge(W.get(&start).unwrap());
    let weight = W.get(&start).unwrap()[index].1;
    let next_node: isize = W.get(&start).unwrap()[index].0.into();
    if next_node == -1 {panic!("end reached");
    }

    if V[next_node as usize] == -1 {
        V[next_node as usize] = V[start] + weight as i8;
    }


    println!("V: {:#?}", V);
    println!("Q: {:#?}", Q);

}

fn min_edge(input: &Vec<(i8,u8)>) -> usize {
    let mut counter:u8 = 0;
    let mut min_edge:u8 = input[0].1;
    let mut min_node_index:u8 = 0;
    for edge in input {
       if edge.1 < min_edge {
           min_edge = edge.1;
           min_node_index = counter;
       }
        counter += 1;
    }
    min_node_index as usize

}
    // #[allow(non_snake_case)]
    // let W = std::collections::HashMap::from([
    //     (0,Vec::from([1,2])),
    //     (1,Vec::from([2,3])),
    //     (2,Vec::from([4])),
    //     (3,Vec::from([5])),
    //     (4,Vec::from([-1])),
    //     (5,Vec::from([-1]))
    // ]);
