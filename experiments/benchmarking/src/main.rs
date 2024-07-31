fn main() {
    a();
    b();

}



fn a() {
    let mut a = true;
    if a {
        for i in 0..100 {
            std::hint::black_box(&mut a);
            println!("{}", i);
            let b = 2 ^ i;
            std::hint::black_box(&mut a);
        }
    } else {
        for i in 0..100 {
            std::hint::black_box(&mut a);
            let b = 2 ^ i;
            std::hint::black_box(&mut a);
        }
    }


}
fn b() {
    let mut a = true;
    for i in 0..100 {
        std::hint::black_box(&mut a);
        if a {
            println!("{}", i)
        }

        let b = 2^i;
        std::hint::black_box(&mut a);

    }


}


fn c() {
    let b = 3*5;
    println!("do stuffig");
}