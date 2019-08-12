# benchmark-rust

## Run
```bash
cargo run --release
```

## Results
```
Starting!
Zero test: simple array
sum:247500000 time:38.205µs
First test: Vec loop with index access
sum:247500000 time:670.146µs
Second test: Vec iterator loop
sum:247500000 time:636.078µs
Third test: Vec for_each loop
sum:247500000 time:630.724µs
Fourth test: Vec fold loop
sum:247500000 time:622.129µs
End!
```

