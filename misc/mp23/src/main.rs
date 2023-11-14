use itertools::Itertools;
use rand::Rng;

fn create_dist(n: usize) -> Vec<Vec<f64>> {
    let mut b = rand::thread_rng();
    let mut output: Vec<Vec<f64>> = Vec::new();

    for _ in 0..n {
        let mut sum = 10;
        let mut row: Vec<f64> = Vec::new();
        for _ in 0..10 {
            let a: f64 = Rng::gen_range(&mut b, 0..=sum) as f64 / 10f64;
            sum -= (a * 10f64) as i32;
            row.push(a);
        }
        println!("{:?}", row);
        output.push(row);
    }

    output
}

fn sums(raw_numbers: Vec<f64>) -> Vec<(f64, Vec<(usize, f64)>, Vec<(usize, f64)>)> {
    let target = 0.5;
    let mut output: Vec<(f64, Vec<(usize, f64)>, Vec<(usize, f64)>)> = vec![(0f64, Vec::new(), Vec::new()); 10];
    let mut numbers: Vec<(usize, f64)> = raw_numbers
        .iter()
        .enumerate()
        .filter(|(x, y)| y <= &&target)
        .filter(|(x, y)| y != &&0f64)
        .sorted_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(x, y)| (x, *y))
        .collect();

    for _ in 0..numbers.len() {
        for i in 0..10usize {
            if numbers.len() <= 0 { break }
            if (numbers[numbers.len() - 1].1 + output[i].0) <= target { // P(current digit) + P(selected)
                output[i].1.push(numbers[numbers.len() - 1]); // push to output
                output[i].0 += numbers[numbers.len() - 1].1; // increment digit P(total)
                numbers.remove(numbers.len() - 1); // remove from options
            }
        }
    }


    output
}

fn main() {
    let mut dist = create_dist(10);
    let mut numbers: Vec<f64> = Vec::new();
    for mut line in dist { numbers.append(&mut line); }

    let mut score = 0f64;
    let output = sums(numbers);
    for line in &output {
        score += 0.5 - line.0;
        println!("output-line: {:?}", line);
    }
    println!("score {:.1}", score);
}