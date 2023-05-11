# HW3: evaluating two different AMQs

## Language
-Rust
## Overall Structure
- Code in `amq/src` directory
- `main.rs` will run bloom filter, minimal perfect hash filters, and mphf with fingerprint array experiments with a set of keys of various sizes. Output for size, false positive rate, and timing will be printed to stdout
  - Experiments are implemented for 
    - `K/K'` set sizes of 1000,5000,10000, and 50000 (key length of 31 characters)
    - expected false positive rate of 1/2^7, 1/2^8, 1/2^10
    - Mix of old/new keys of 10%, 25%, and 50%
    - Fingerprint stored with last 7,8, or 10 bits 
- `bloom_wrap.rs` is used to generate and query the bloom filter
- `lib.rs` has the function to generate `K` and `K'`
- `mphf.rs` is used to generate and query mphfs and the fingerprint arrays in `main.rs` 
- `Cargo.toml` contains all dependencies

## Instructions
1. within the `amq` folder: run `cargo build --release`
2. Run `cargo run --release` to run full experiment
