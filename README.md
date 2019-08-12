# benchmark-rust

## Run
```bash
cargo run --release
```

## Results

* CAPACITY = 100, LOOPS = 1, IN_LOOPS = 5000000, ALL_LOOPS = 1
```
Starting!
Zero test: simple array
sum:24750000000 time:1.849236ms
First test: Vec loop with index access
sum:24750000000 time:69.376212ms
Second test: Vec iterator loop
sum:24750000000 time:71.716159ms
Third test: Vec for_each loop
sum:24750000000 time:73.162476ms
Fourth test: Vec fold loop
sum:24750000000 time:74.056854ms
End!

```

* CAPACITY = 240, LOOPS = 1, IN_LOOPS = 5000000, ALL_LOOPS = 1
```
Starting!
Zero test: simple array
sum:143400000000 time:166.935978ms
First test: Vec loop with index access
sum:143400000000 time:151.736587ms
Second test: Vec iterator loop
sum:143400000000 time:141.93473ms
Third test: Vec for_each loop
sum:143400000000 time:141.179929ms
Fourth test: Vec fold loop
sum:143400000000 time:141.574687ms
End!
```
