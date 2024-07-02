use rand::Rng;

fn main() {
    println!("Tossing a coin...");

    let mut heads_count = 0;
    let mut tails_count = 0;

    for i in 1..=3 {
        let mut heads_in_round = 0;
        let mut tails_in_round = 0;

        for _ in 0..5 {
            let flip = rand::thread_rng().gen_range(0..2);
            if flip == 0 {
                heads_in_round += 1;
            } else {
                tails_in_round += 1;
            }
        }

        let result = if heads_in_round > tails_in_round { "Heads" } else { "Tails" };
        println!("Round {}: {}", i, result);
        
        match result {
            "Heads" => heads_count += 1,
            "Tails" => tails_count += 1,
            _ => {}
        }
    }
    println!("Heads: {}, Tails: {}", heads_count, tails_count);
}
