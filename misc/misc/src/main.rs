use std::thread::AccessError;

fn main() {
    // let mut vec1 = vec![1,2,3,4,5];
    // b(&mut vec1);

    // for i in 5..0 {
    //     println!("{:?}", i);
    // }

    println!("{}", 10usize.pow(2));
}


fn b(b: &mut Vec<i32>) {
    b.remove(2);
}