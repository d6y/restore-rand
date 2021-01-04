use crate::Result;
use rand::prelude::SeedableRng;
use rand_pcg::Pcg32;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{fs, path::PathBuf};
use tempfile::{self, NamedTempFile};

pub fn init(seed: u64) -> (u64, Pcg32) {
    (0, Pcg32::seed_from_u64(seed))
}

#[derive(Debug, Serialize, Deserialize)]
struct State<R: Serialize> {
    rng: R,
    iter: u64,
}

pub fn checkpoint<R: Serialize>(path: &PathBuf, iter: u64, rng: R) -> Result<()> {
    let state = State { rng, iter };
    let json = serde_json::to_string(&state)?;

    let tmp = NamedTempFile::new()?;
    fs::write(tmp.path(), json)?;
    fs::rename(tmp.path(), path)?;

    Ok(())
}

pub fn restore(path: &PathBuf) -> Result<(u64, Pcg32)> {
    let json = fs::read_to_string(path)?;
    let state: State<Pcg32> = serde_json::from_str(&json)?;
    Ok((state.iter + 1, state.rng))
}
