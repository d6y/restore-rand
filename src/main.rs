use rand::prelude::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use serde_json;
use std::fs;
use structopt::StructOpt;

// Thank you:
// https://users.rust-lang.org/t/saving-and-restoring-the-state-of-a-seedablerng/52642/2?u=d6y

#[derive(Debug, StructOpt)]
struct Args {
    /// Initial seed for random number generator
    #[structopt(short, long)]
    pub seed: u64,

    /// Random state to restore
    #[structopt(short, long)]
    pub restore_checkpoint: Option<String>,
}

fn main() {
    let args = Args::from_args();
    println!("{:?}", args);

    match args.restore_checkpoint {
        // Not restoring, so run step 1, save state, run step 2
        None => {
            let mut rng = Pcg32::seed_from_u64(args.seed);

            println!("Step 1: {}", step(&mut rng));

            let checkpoint = serde_json::to_string(&rng).expect("serialize random state");
            let checkpoint_filename = format!("checkpoint-seed-{}-step-1.json", args.seed);
            fs::write(checkpoint_filename, checkpoint).expect("Unable to write file");

            println!("Step 2: {}", step(&mut rng));
        }

        // Restoring, and then just run step 2
        Some(checkpoint_filename) => {
            let json = fs::read_to_string(checkpoint_filename).expect("reading json");
            let mut rng: Pcg32 = serde_json::from_str(&json).expect("deserialize random state");
            println!("Step 2: {}", step(&mut rng));
        }
    }
}

fn step<R: Rng>(rng: &mut R) -> u64 {
    (0..100).map(|_| rng.gen_range(0, 100)).sum()
}
