use std::any::Any;

fn main() {
    let a = a();
    let b: &A = &a.downcast().unwrap();
    println!("{:?}", b);










}

fn a() -> Box<dyn Any>{
    Box::new(A{a: 5})
}

#[derive(Debug)]
struct A {
    a: u8
}

impl A {
    fn a(&self) {
        println!("{}", self.a);
    }
}
