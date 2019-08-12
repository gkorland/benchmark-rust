# benchmark-rust

## Run
```bash
cargo run --release
```

## Results
```
Starting!
Zero test: simple array
sum:247500000 time:34.23µs
First test: Vec loop with index access
sum:247500000 time:644.947µs
Second test: Vec iterator loop
sum:247500000 time:631.225µs
Third test: Vec for_each loop
sum:247500000 time:690.126µs
Fourth test: Vec fold loop
sum:247500000 time:661.474µs
End!
```

