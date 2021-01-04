use git_version::git_version;
use rand::prelude::Rng;
use std::path::PathBuf;
use structopt::StructOpt;

mod state;

const VERSION: &str = git_version!();

// Thank you:
// https://users.rust-lang.org/t/saving-and-restoring-the-state-of-a-seedablerng/52642/2?u=d6y

#[derive(Debug, StructOpt)]
struct Settings {
    /// Initial seed for random number generator
    #[structopt(short, long, default_value = "1")]
    seed: u64,

    /// How many iterations to run
    #[structopt(short, long, default_value = "10000")]
    num_iter: u64,
}

// Our result is some value of type T, or some error implementation
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let settings = Settings::from_args();
    println!("{:?}", settings);
    let id = experiment_id(&settings, VERSION);

    // Where we save state for this run:
    let checkpoint_filename = format!("{}.checkpoint", id);
    let checkpoint_path = PathBuf::from(&checkpoint_filename);

    // Restore or initialize:
    let (start_iter, mut rng) = if checkpoint_path.exists() {
        state::restore(&checkpoint_path)?
    } else {
        state::init(settings.seed)
    };

    println!("Running from step {}", start_iter);

    for i in start_iter..settings.num_iter {
        // Pretend step is some meaningful but slow process:
        let number = step(&mut rng);
        println!("iteration {}, {}", i, number);

        state::checkpoint(&checkpoint_path, i, &rng)?;
    }

    println!("Run complete; cleaning up");
    std::fs::remove_file(checkpoint_path)?;

    Ok(())
}

fn step<R: Rng>(rng: &mut R) -> u64 {
    (0..100).map(|_| rng.gen_range(0..10)).sum()
}

fn experiment_id(settings: &Settings, version: &str) -> String {
    let settings_text = format!("{:?}", &settings);
    format!("{}_{}", version, hash(settings_text))
}

fn hash(s: String) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    let mut h = DefaultHasher::new();
    h.write(s.as_bytes());
    h.finish()
}
