use std::collections::HashMap;
use anyhow::Error;

fn main() {
    trait a {
        fn a(&self) {}
    }

    impl a for anyhow::Error {
        fn a(&self) {
            print!("AAAAAAAAAAAAAAAAa");
        }
    }

    let inner = String::from("asdg").into();
    let b = anyhow::Error{inner};
    b.a();
}
