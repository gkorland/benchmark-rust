use std::time::{Instant};

const CAPACITY : usize = 100;
const LOOPS : usize = 1;
const IN_LOOPS : usize = 50000;
const ALL_LOOPS: usize = 1;

fn main() {
    println!("Starting!");

    let mut arr = [0; CAPACITY];
    for i in 0..CAPACITY {
        arr[i] = i;
    }
    let vec = arr.to_vec();


    for _ in 0..ALL_LOOPS {
        println!("Zero test: simple array");
        for _ in 0..LOOPS {
            let mut sum = 0;
            let now = Instant::now();
            for _ in 0..IN_LOOPS {
                let mut s = 0;
                for i in 0..arr.len() {
                    s += arr[i];
                }
                sum += s;
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
        }


        println!("First test: Vec loop with index access");
        for _ in 0..LOOPS {
            let mut sum = 0;
            let now = Instant::now();
            for _ in 0..IN_LOOPS {
                let mut s = 0;
                for i in 0..vec.len() {
                    s += vec[i];
                }
                sum += s;
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
        }

        println!("Second test: Vec iterator loop");
        for _ in 0..LOOPS {
            let mut sum = 0;
            let now = Instant::now();
            for _ in 0..IN_LOOPS {
                let mut s = 0;
                for element in vec.iter() {
                    s += element;
                }
                sum += s;
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
        }

        println!("Third test: Vec for_each loop");
        for _ in 0..LOOPS {
            let mut sum = 0;
            let now = Instant::now();
            for _ in 0..IN_LOOPS {
                let mut s = 0;
                vec.iter().for_each(|element| s += element);
                sum += s;
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
        }

        println!("Fourth test: Vec fold loop");
        for _ in 0..LOOPS {
            let mut sum = 0;
            let now = Instant::now();
            for _ in 0..IN_LOOPS {
                sum += vec.iter().fold(0, |acc, x| acc + x);
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
        }
    }

    println!("End!");
}
