use std::collections::HashSet;
fn main() {
    let mut map: HashSet<i32> = HashSet::with_capacity(1000);

    for i in 1..=1 {
        for j in 1..50 {
            let a = num::complex::Complex::new(i, j);
            let b = a.conj();
            let c = (a * b).re;

            println!("{} * {} = {}", a, b, c);

            if map.contains(&c) {
                println!("collision");
            } else {
                // println!("insertion");
                map.insert(c);
            }
        }
    }

    // let a = num::

}

