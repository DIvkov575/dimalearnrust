pub struct A {}

pub trait mytrait {
    fn a(&self);
}

impl mytrait for A {
    fn a(&self) {
        println!("a")
    }
}
