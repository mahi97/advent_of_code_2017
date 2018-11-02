use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut f = File::open("./input.txt").expect("file not found");
    let mut content = String::new();
    f.read_to_string(&mut content).expect("something went wrong");
    
    println!("file: {}", content);
    let first = content.chars().next().unwrap();
    let mut sum : u32 = 0;
    for (i, ch) in content.chars().enumerate() {
        let mut next = if i == content.len() - 2 { first } 
        else { 
            match content.chars().nth(i+1){
                None => continue,
                Some(num) => num,
            }
        };
        if ch == next {
            let v : u32 = ch.to_digit(10).unwrap();
            sum = sum + v;
        }
    }
    println!("Sum is : {}", sum);

}
