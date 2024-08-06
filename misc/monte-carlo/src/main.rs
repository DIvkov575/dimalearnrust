use rand::seq::SliceRandom; // 0.7.2


fn main() {
    let mut agg: f32 = 0.0;
    for i in 0..1000g0{
        agg += sim();
    }
     println!("{:?}", agg / 10000f32)
}


fn sim() -> f32 {

    let hearts : Vec<f32> = vec![0.0, 0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0, 1.0, 2.0, 3.0, 4.0];
    let diamonds = hearts.clone();
    let spades= hearts.clone();
    let clubs = hearts.clone();

    let heart_sum: f32 = hearts.choose_multiple(&mut rand::thread_rng(), 7).sum();
    let diamond_sum: f32 = diamonds.choose_multiple(&mut rand::thread_rng(), 2).sum();
    let spade_sum: f32 = spades.choose_multiple(&mut rand::thread_rng(), 2).sum();
    let club_sum: f32 = clubs.choose_multiple(&mut rand::thread_rng(), 2).sum();
    //
    // println!("{:?} {}", heart_collect, heart_collect.iter().map(|x| x.to_owned()).sum::<f32>());
    // println!("{:?} {}", diamond_collect, diamond_collect.iter().map(|x| x.to_owned()).sum::<f32>());
    // println!("{:?} {}", spade_collect, spade_collect.iter().map(|x| x.to_owned()).sum::<f32>());
    // println!("{:?} {}", club_collect, club_collect.iter().map(|x| x.to_owned()).sum::<f32>());

    heart_sum + diamond_sum + spade_sum + club_sum
}
