
#[macro_export]
macro_rules! f {
    ($a:ident) => {($a-1.0)};
}

#[macro_export]
macro_rules! g {
    ($a:ident, $y:ident) => {($a-$y)};
}
#[macro_export]
macro_rules! h {
    ($x:ident) => {(2.0-$x)};
}

fn main() {
    let steps = 3usize;
    let step_size= -0.1;

    let mut x_0: f64 = 2.into();
    let mut y_0: f64 = 1.into();
    // let mut tmp;


    for i in 0..steps {
        y_0 += h!(x_0)*step_size; // 3
        // y_0 += g!(x_0, y_0)*step_size; //4
        x_0 += step_size
    }

    println!("{}", y_0);
}


