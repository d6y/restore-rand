# Test of restoring random number state

By default, this code runs 10,000 steps of random number generation.
With a fixed seed (default: 1), the result will always be the same.

After each step, the code saves state to disk.
Interrupt the programme (ctrl-c), and run it again: the code should pick up where it left off, producing the same result as if it were not interrupted.

Example:

```
% cargo run | tail -2                                                                            [master]
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/restore-rand`
iteration 9999, 461
Run complete; cleaning up
```

The 461 number is the result of some random calculation.

Re-run again, but interrupt. The state will be saved on disk and reloaded:

```
% cargo run
(much output)
iteration 8604, 469
iteration 8605, 425
^C

% cargo run 
Settings { seed: 1, num_iter: 10000 }
Running from step 8606
(much output)
iteration 9999, 461
Run complete; cleaning up
```

Thank you to mbrubeck for pointing me at the pcg crate for serializing random number state, on [users.rust-lang.org]( https://users.rust-lang.org/t/saving-and-restoring-the-state-of-a-seedablerng/52642/2?u=d6y)
