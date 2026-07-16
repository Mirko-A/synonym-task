# synonym-task

This is a small Rust demo for orchestrating concurrent async work.

## Run it

```sh
cargo run -- --jobs 4 --fail-rate 0
```

That runs four jobs and lets them all complete.

To see the failure path:

```sh
cargo run -- --jobs 4 --fail-rate 0.25
```

`--jobs` must be greater than zero. `--fail-rate` must be between `0.0` and `1.0`; it controls how many jobs are set up to fail.
