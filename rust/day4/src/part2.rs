use std::fs::File;
use std::io::prelude::*;

use std::iter::Iterator;
use std::iter::FromIterator;

fn main() {

    let mut f = File::open("./input.txt").expect("file not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("something went wrong!");
    
    let mut sum: usize = 0;
    for l in content.lines() {
        let mut flag : bool = false;
        for c in l.split_whitespace() {
            for c2 in l.split_whitespace() {
                let mut chars: Vec<char> = c.chars().collect();
                chars.sort_by(|a, b| b.cmp(a));
                let c = &String::from_iter(chars)[..];
                let mut chars: Vec<char> = c2.chars().collect();
                chars.sort_by(|a, b| b.cmp(a));
                let c2 = &String::from_iter(chars)[..];
                if c == c2 {
                    if flag {
                        flag = false;
                        break;
                    } else {
                        flag = true;
                    }
                }
            }
            if !flag { flag = true; break; } else { flag = false; }
        }
        if !flag { sum += 1; println!("L: {}", l);}
    }

    println!("Sum {}", sum);

}
