use std::error::Error;
use bigint::U512;

fn main() -> Result<(), Box<dyn Error>> {
    print_n_padic(50, 100);

    // let a =String::from_utf8(vec![b'1'; 281).unwrap();
    // println!("{}", a);


    Ok(())
}



fn print_n_padic_numbers(num_digits: u32, dig_size: u32) {
    let padding = 281 - num_digits;

    for i in 0..num_digits {
        let a = 3u32.pow(10u32).pow(i) % (10 * dig_size);
        let char_padding=String::from_utf8(vec![b' '; padding as usize]).unwrap();

        println!("{}{}", char_padding, a);
    }
}


fn print_n_padic(num_digits: usize, dig_size: usize) {
    let padding = 281 - dig_size;

    for i in 0..num_digits {
        let a = U512::from(3).pow(U512::from(10)).pow(U512::from(i)) % (U512::from(10).pow(U512::from(dig_size)));
        // let padding_size = 281 - a.
        let char_padding=String::from_utf8(vec![b' '; padding]).unwrap();
        println!("{}{}", char_padding, a);
    }
}