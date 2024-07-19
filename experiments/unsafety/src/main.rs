fn main() {
    unsafe {
        let mut a = 5usize;
        let b = &mut a as *mut usize;
        let c= b;

        *b += 1;
        *c += 1;

        println!("{:?}", a);
        println!("{:?}", b);
    }
}
