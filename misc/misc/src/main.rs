fn main() {
    let mut time;
    let mut output_times: Vec<_> = Vec::new();
    let i1 = 500000;


    time = std::time::Instant::now();
    for i in 0..i1 {
        println!("hello");
    }
    output_times.push(time.elapsed());

    //obtaining lock ======================
    let mut lock = stdout().lock();

    time = std::time::Instant::now();
    for i in 0..i1 {
        write!(&mut lock, "hello").unwrap();
    }
    output_times.push(time.elapsed());



    time = std::time::Instant::now();
    for i in 0..i1 {
        println!("hello");
    }
    output_times.push(time.elapsed());

    time = std::time::Instant::now();
    for i in 0..i1 {
        print!("hello \n");
    }
    output_times.push(time.elapsed());

    time = std::time::Instant::now();
    for i in 0..i1 {
        print!("hello  ");
    }
    output_times.push(time.elapsed());





    println!("\n {:?}", output_times);
}
