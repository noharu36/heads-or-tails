use rand::Rng;
use promptuity::prompts::Input;
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;
    p.begin()?;

    let name = p.prompt(Input::new("Who are you?").with_placeholder("yourname"))?;
    p.success(format!("Hello, {}!", name))?;

    p.log("Tossing a coin...")?;

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
        p.info(format!("Round {}: {}", i, result))?;
        
        match result {
            "Heads" => heads_count += 1,
            "Tails" => tails_count += 1,
            _ => {}
        }
    }
    p.with_outro(format!("Heads: {}, Tails: {}", heads_count, tails_count)).finish()?;


    if heads_count > tails_count {
        println!("You won!")
    } else {
        println!("You lost.")
    }

    Ok(())
}
