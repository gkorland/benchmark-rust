use std::time::{Instant};

const CAPACITY : usize = 10000;
const LOOPS : usize = 10;
const IN_LOOPS : usize = 50000;


fn main() {
    println!("Hello, world!");

    let mut arr = [0; CAPACITY];
    for i in 0..CAPACITY {
        arr[i] = i;
    }
    let vec = arr.to_vec();


    for _ in 1..3 {
        println!("ZERO LOOP");
        for _ in 1..LOOPS {
            let mut sum = 0;
            let now = Instant::now();
            for _ in 1..IN_LOOPS {
                let mut s = 0;
                for i in 0..arr.len() {
                    s += arr[i];
                }
                sum += s;
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
        }


        println!("FIRST LOOP");
        for _ in 1..LOOPS {
            let mut sum = 0;
            let now = Instant::now();
            for _ in 1..IN_LOOPS {
                let mut s = 0;
                for i in 0..vec.len() {
                    s += vec[i];
                }
                sum += s;
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
        }

        println!("SECOND LOOP");
        for _ in 1..LOOPS {
            let mut sum = 0;
            let now = Instant::now();
            for _ in 1..IN_LOOPS {
                let mut s = 0;
                for element in vec.iter() {
                    s += element;
                }
                sum += s;
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
        }

        println!("THIRD LOOP");
        for _ in 1..LOOPS {
            let mut sum = 0;
            let now = Instant::now();
            for _ in 1..IN_LOOPS {
                let mut s = 0;
                vec.iter().for_each(|element| s += element);
                sum += s;
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
        }

        println!("FOURTH LOOP");
        for _ in 1..LOOPS {
            let mut sum = 0;
            let now = Instant::now();
            for _ in 1..IN_LOOPS {
                sum += vec.iter().fold(0, |acc, x| acc + x);
            }
            println!("sum:{} time:{:?}", sum, now.elapsed());
        }
    }
}
