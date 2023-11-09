use rand::Rng;
fn main() {
    let mut b = rand::thread_rng();
    let mut output: Vec<Vec<f64>> = Vec::new();

    for _ in 0..6{
        let mut sum = 10;
        let mut row: Vec<f64> = Vec::new();
        for _ in 0..10 {
            let a: f64= Rng::gen_range(&mut b, 0..=sum) as f64 /10f64 ;
            sum -= (a*10f64) as i32;
            row.push(a);
        }
        println!("{:?}", row);
        output.push(row);
    }


}
