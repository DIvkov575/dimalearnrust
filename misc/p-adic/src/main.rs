use std::error::Error;
use std::path::Component::ParentDir;
use bigint::U512;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", U512::MAX);
    print_n_padic_numbers();




    Ok(())
}


fn print_n_padic_numbers() {
    let trunc = U512::from(10u32.pow(8));
    let mut a = U512::from(3u32);
    for i in 0..10 {
        println!("{}", a);
        a = a.pow(U512::from(10)) % trunc;
    }

}


// fn print_n_padic_numbers(num_digits: u32, dig_size: u32) {
//     let padding = 281 - num_digits;
//
//     for i in 0..num_digits {
//         let a = 3u32.pow(10u32).pow(i) % (10 * dig_size);
//         let char_padding=String::from_utf8(vec![b' '; padding as usize]).unwrap();
//         println!("{}{}", char_padding, a);
//     }
// }


fn print_n_padic(num_digits: usize, dig_size: usize) {
    let padding = 281 - dig_size;
    let bp = 1;

    for i in 0..num_digits {
        let q = i / bp;
        let r = i % bp;
        let mut a = U512::from(3usize);
        for j in 0..q {
            a = a.pow(U512::from(10)).pow(U512::from(bp)) % (U512::from(10).pow(U512::from(dig_size)));
        }
        a = a.pow(U512::from(10).pow(U512::from(r))) % (U512::from(10).pow(U512::from(dig_size)));

        let char_padding=String::from_utf8(vec![b' '; padding]).unwrap();
        println!("{}{}", char_padding, a);
    }
}