use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    gen_padic_numbers(25, 50);

    Ok(())
}



fn gen_padic_numbers(num_digits: u32, dig_size: u32) {
    let padding = 331 - num_digits;
    for i in 0..num_digits {
        let a = 3u32.pow(10u32).pow(i) % (10 * dig_size);
        let char_padding=String::from_utf8(vec![b' '; padding as usize]).unwrap();

        println!("{}{}", char_padding, a);
    }

}