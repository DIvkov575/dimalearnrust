use std::cell::RefCell;
use std::cmp::max;

fn main() {
    let value: Vec<Vec<isize>> = vec![vec![-1; 14]; 14];
    let w = vec![5, 2, 3, 9, 3];
    let v = vec![5, 6, 3, 4, 2];
    let mut arr: Arrs = Arrs {value, w, v};

    ks(&mut arr, 4, 6);

    for row in arr.value { println!("{:?}", row); }
}

struct Arrs {
    value: Vec<Vec<isize>>,
    w: Vec<usize>,
    v: Vec<usize>,
}

fn ks(arrs: &mut Arrs, i: usize, j: usize) {
    if (i == 0) || (j <= 0) {
        arrs.value[i][j] = 0;
        return
    }
    if arrs.value[i-1][j] == -1 {
        ks(arrs, i-1, j);
    }
    if arrs.w[i]>j {
        arrs.value[i][j] = arrs.value[i-1][j];
    } else if (arrs.value[i-1][j - arrs.w[i]] == -1) {
        ks(arrs, i-1, j-arrs.w[i]);
        arrs.value[i][j] = max(arrs.value[i-1][j], arrs.value[i-1][j-arrs.w[i] + arrs.v[i]]);
    }
}


// Define value[n, W]
//
// Initialize all value[i, j] = -1
//
// Define m:=(i,j)         // Define function m so that it represents the maximum value we can get under the condition: use first i items, total weight limit is j
// {
// if i == 0 or j <= 0 then:
// value[i, j] = 0
// return
//
// if (value[i-1, j] == -1) then:     // m[i-1, j] has not been calculated, we have to call function m
// m(i-1, j)
//
// if w[i] > j then:                      // item cannot fit in the bag
// value[i, j] = value[i-1, j]
// else:
// if (value[i-1, j-w[i]] == -1) then:     // m[i-1,j-w[i]] has not been calculated, we have to call function m
// m(i-1, j-w[i])
// value[i, j] = max(value[i-1,j], value[i-1, j-w[i]] + v[i])
// }
//
// Run m(n, W)