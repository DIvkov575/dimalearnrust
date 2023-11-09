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
    let mut output: Vec<(f64, Vec<(usize, f64)>, Vec<(usize, f64)>)> = Vec::from([
        (0f64, Vec::new(), Vec::new()),
        (0f64, Vec::new(), Vec::new()),
        (0f64, Vec::new(), Vec::new()),
        (0f64, Vec::new(), Vec::new()),
        (0f64, Vec::new(), Vec::new()),
        (0f64, Vec::new(), Vec::new()),
        (0f64, Vec::new(), Vec::new()),
        (0f64, Vec::new(), Vec::new()),
        (0f64, Vec::new(), Vec::new()),
        (0f64, Vec::new(), Vec::new()),
    ]);
    let target = 0.5;
    let mut numbers1: Vec<(usize, f64)> = raw_numbers
        .iter()
        .enumerate()
        .filter(|(x, y)| y <= &&target)
        .filter(|(x, y)| y != &&0f64)
        .sorted_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(x, y)| (x, *y))
        .collect();
    let mut numbers2: Vec<(usize, f64)> = raw_numbers
        .iter()
        .enumerate()
        .filter(|(x, y)| y > &&target)
        .filter(|(x, y)| y != &&1f64)
        .sorted_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(x, y)| (x, *y))
        .collect();

    println!("numbers1{:?}", numbers1);
    println!("numbers2{:?}", numbers2);

    for i in (0..10usize).rev() {
        if numbers2.len() <= 1 { break; }
        let mut buf: Vec<(usize, f64)> = Vec::from([numbers2[numbers2.len() - 1]]);
        let mut prod = numbers2[numbers2.len() - 1].1;

        for k in 0..(numbers2.len() - 1) {
            prod *= numbers2[k].1;
            buf.push(numbers2[k]);
            if (output[i].0 + prod) <= 0.5 {
                output[i].0 += prod;
                output[i].2.append(&mut buf.clone());
                // println!("buf: {:?}", buf);
                for elem in buf {
                    let index = numbers2.iter().position(|x| x == &elem).unwrap();
                    // println!("index: {index}, elem: {:?}", elem);
                    numbers2.remove(index);
                }
                break;
            }
        }
    }

    for _ in 0..numbers1.len() {
        for i in 0..10usize {
            if numbers1.len() <= 0 { break }
            if (numbers1[numbers1.len() - 1].1 + output[i].0) <= target { // P(current digit) + P(selected)
                output[i].1.push(numbers1[numbers1.len() - 1]); // push to output
                output[i].0 += numbers1[numbers1.len() - 1].1; // increment digit P(total)
                numbers1.remove(numbers1.len() - 1); // remove from options
            }
        }
    }

    if numbers1.len() > 1 {
        for i in (0..10usize).rev() {
            if numbers1.len() <= 1 { break; }
            let mut buf: Vec<(usize, f64)> = Vec::from([numbers1[numbers1.len() - 1]]);
            let mut prod = numbers1[numbers1.len() - 1].1;

            for k in 0..(numbers1.len() - 1) {
                prod *= numbers1[k].1;
                buf.push(numbers1[k]);
                if (output[i].0 + prod) <= 0.5 {
                    output[i].0 += prod;
                    output[i].2.append(&mut buf.clone());
                    // println!("buf: {:?}", buf);
                    for elem in buf {
                        let index = numbers1.iter().position(|x| x == &elem).unwrap();
                        // println!("index: {index}, elem: {:?}", elem);
                        numbers1.remove(index);
                    }
                    break;
                }
            }
        }
    }

    println!("numbers1 remaining: {:?}", numbers1);
    println!("numbers2 remaining: {:?}", numbers2);

    output
}

fn main() {
    let mut dist = create_dist(10);
    let mut numbers: Vec<f64> = Vec::new();
    for mut line in dist { numbers.append(&mut line); }

    let mut score = 0f64;
    let output = sums(numbers);
    for line in &output {
        score += (0.5 - line.0);
        println!("output-line: {:?}", line);
    }
    println!("score {:.1}", score);
}
