

fn main() {
    let mut steps = 3usize;
    let mut step_size= 0.1;

    let mut x_0 = 1f64;
    let mut y_0 = 2f64;
    // let mut tmp;


    for i in 0..steps {
        y_0 += g!(y_0, x_0)*step_size;
        x_0 += step_size
    }

    println!("{}", y_0);
}


#[macro_export]
macro_rules! f {
    ($a:ident) => {($a-1.0)};
}

#[macro_export]
macro_rules! g {
    ($a:ident, $y:ident) => {($a-$y)};
}