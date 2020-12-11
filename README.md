# Test of restoring random number state

Thank you to mbrubeck for solving this for me on [users.rust-lang.org]( https://users.rust-lang.org/t/saving-and-restoring-the-state-of-a-seedablerng/52642/2?u=d6y)

When run with just a seed:

```
cargo run -- --seed 1
```

...will run two random calculations, 
saving the state after the first step (as `checkpoint-seed-1-step-1.json`).

Run again, but restoring state:

```
cargo run -- --seed 9999 --restore-checkpoint checkpoint-seed-1-step-1.json
```

...and this will restore the random state and just run the second step.

The output for Step 2 should be the same in both cases.

