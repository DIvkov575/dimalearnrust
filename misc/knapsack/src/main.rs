
enum Output {
    Table(Vec<Vec<usize>>),
    Items(Vec<usize>),
}


fn main() {
    let w = vec![1usize, 2, 3];
    let v = vec![1usize, 2, 5];
    let capacity = 4;

    match knapsack(capacity, w, v) {
        Output::Items(a) => println!("{:?}", a),
        Output::Table(a) => for row in a {println!("{:?}", row)},
    }
}


pub fn knapsack(capacity: usize, w: Vec<usize>, v:Vec<usize>) -> Output {
    let n = w.len();
    let mut output: Vec<Vec<usize>> = vec![vec![0; n+1]; capacity+1usize];

    for i in 1..=n {
        let w = w[i-1];
        let v = v[i-1];

        for sz in 1..=n {
            output[i][sz] = output[i-1][sz];

            if (sz >= w) && (output[i-1][sz-w] + v > output[i][sz]) {
                output[i][sz] = output[i-1][sz-w] + v;
            }
        }
    }

    // let mut sz = capacity;
    // let mut items_selected: Vec<usize> = Vec::new();
    //
    // for i in (1..=n).rev() {
    //     if output[i][sz] != output[i-1][sz] {
    //         items_selected.push(i-1);
    //         sz -= w[i-1];
    //     }
    // }

    Output::Table(output)
}