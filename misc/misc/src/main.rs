use spinoff::{Color, Spinner, spinners};

fn main() {
    trait a {
        fn a(&self) {}
    }

    impl a for Spinner{
        fn a(&self) {
            print!("AAAAAAAAAAAAAAAAa");
        }
    }

    let b= Spinner::new(spinners::Dots, "Initializing Terraform...", Color::Blue);
    b.a();
}
