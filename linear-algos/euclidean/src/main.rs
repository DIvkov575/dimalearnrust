#![allow(non_snake_case)]

fn main() {
    println!("{}", F(2166,6099));
}

fn F(mi: usize, ni: usize) -> usize{
    let mut r: usize = 0;
    let mut m: usize;
    let mut n: usize;

    // if mi < ni {
    //     m = ni;
    //     n = mi;
    // } else {
        m = mi;
        n = ni;
    // };

    loop {
        println!("{} {} {}", r,m,n);

        r = m%n;
        if r == 0 {
            return n;
        }
        m = n;
        n = r;
    }
}
fn E(mi: usize, ni: usize) -> usize{
    let mut r: usize = 0;
    let mut m: usize;
    let mut n: usize;

    if mi < ni {
        m = ni;
        n = mi;
    } else {
        m = mi;
        n = ni;
    };
    loop {
        println!("{} {} {}", r,m,n);

        r = m%n;
        if r == 0 {
            return n;
        }
        m = n;
        n = r;
    }
}
