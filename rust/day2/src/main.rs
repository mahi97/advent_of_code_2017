use std::fs::File;
use std::io::prelude::*;

fn main() {

    let mut f = File::open("./input.txt").expect("File Not Found!");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong");
    let lines = contents.lines();
    let mut sum : u32 = 0;
    let mut sum2: u32 = 0;
    for l in lines {
        let a = l.split_whitespace();
        let mut min : u32 = 1000_000_000;
        let mut max : u32 = 0;
        let mut l: [u32; 10_000] = [0; 10_000];
        for (i, num) in a.enumerate() {
            let num = num.to_string().parse::<u32>().unwrap();
            if num < min { min = num; }
            if num > max { max = num; }
            println!("{}", num);
            l[i] = num;
            for (j, v) in l.iter().enumerate() {
                if j < i {
                    if *v >  num {
                        if *v % num == 0 {
                            sum2 += v / num;
                        }
                    } else {
                        if num % v == 0 {
                            sum2 += num / v;
                        }
                    }
                }
            }
            

        }
        sum += max - min;
    }
    println!("SUM: {}", sum);
    println!("SUM2: {}", sum2);

}
