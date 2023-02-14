// created echo
// fn main() {
//     let args: Vec<String> = std::env::args().collect();
//     let mut ctr: u16 = 0;

//     for i in args{
//         if ctr > 0 {
//             print!("{} ", i);
//         }
//         ctr+=1;
//     }
// }

#![feature(option_result_contains)]
fn main() {
   //cat
    let args: Vec<String> = std::env::args().collect();
    // let mut ctr: u16 = 0;

    let mut iter = args.iter();
        iter.next();
        loop {
            let elem = iter.next();
            if elem.contains(&"/") {
                println!("inner");
                // let path = std::path::Path::new(&elem);
                if let Some(path) = std::path::Path::new(&elem)  {
                    let file_content: String = std::fs::read_to_string(&path).unwrap();
                    println!("{}", &file_content)
                }

            }
            // println!("{}", elem.unwrap());
        }
}
