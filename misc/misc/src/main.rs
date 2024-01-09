use std::cmp::max;


fn main() {
    static mut arr: [[Option<usize>; 11]; 11] = [[None; 11]; 11];
    static w: [usiz5e; 4] = [5, 3, 10, 9];
    static mut v: [usize; 4] = [1, 4, 3, 10];

    let capacity = 10;
    let a= ks(w.len()-1, capacity);
    println!("optimal value: {}", a);

    unsafe {
        for val in &arr {
            println!("{:?}", val);
        }
    }

    fn ks(n: usize, c: usize) -> usize {
        let result: usize;
        unsafe {
            if arr[n][c] != None {
                return arr[n][c].unwrap();
            }
            if (n==0) || (c==0) {
                result = 0
            } else if w[n] > c {
                result = ks(n-1, c);
            } else {
                let tmp1 = ks(n-1, c);
                let tmp2 = ks(n-1, c);
                v[n] = ks(n-1, c);
                result = max(tmp1, tmp2);
            }
        arr[n][c] = Some(result);
        }

        return result;
    }

}

// fn ks(arr: &mut Vec<Vec<Option<usize>>>, n: usize, c: usize) -> usize {


