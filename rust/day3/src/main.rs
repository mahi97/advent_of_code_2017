const size : usize = 200;
const msize : i32 = size as i32;
fn sum_up(x: i32, y: i32, m : [[u32; size*2]; size*2]) -> u32{
    let x = x + msize;
    let y = y + msize;
    let x = x as usize;
    let y = y as usize;
    m[x + 1][y] + m[x - 1][y] + m[x][y + 1] + m[x][y - 1] + m[x-1][y-1] + m[x-1][y+1] + m[x+1][y-1] + m[x+1][y+1]
}


fn main() {
    let mut cursor: (i32, i32) = (0, 0);

    
    let mut mat = [[0u32; size*2]; size*2];

    enum State {
        Right,
        Up,
        Left,
        Down,
    }
    let mut s:State = State::Right;
    let mut stepLeft = 1;
    let mut stepMax = 1;
    let target: u32 = 325489;
    let mut counter: u32 = 1;
    mat[size][size] = 1;
    while mat[(cursor.0 + msize) as usize][(cursor.1 + msize) as usize] < target {
        println!("V: {}", mat[(cursor.0 + msize) as usize][(cursor.1 + msize) as usize]);
        match s {
            State::Right => {
                if stepLeft > 0 {
                    stepLeft -= 1;
                    cursor.0 += 1;
                    counter += 1;
        println!("Cursor: {} {}", cursor.0, cursor.1);
                    mat[(cursor.0 + msize) as usize][(cursor.1 + msize) as usize] = sum_up(cursor.0, cursor.1, mat);
                } else { 
                    s = State::Up;
                    stepLeft = stepMax;
                }
            },
            State::Up => {
                if stepLeft > 0 {
                    stepLeft -= 1;
                    cursor.1 += 1;
                    counter += 1;
        println!("Cursor: {} {}", cursor.0, cursor.1);
                    mat[(cursor.0 + msize) as usize][(cursor.1 + msize) as usize] = sum_up(cursor.0, cursor.1, mat);
                } else { 
                    s = State::Left; 
                    stepMax += 1;
                    stepLeft = stepMax;
                }
            },
            State::Left => {
                if stepLeft > 0 {
                    stepLeft -= 1;
                    cursor.0 -= 1;
                    counter += 1;
        println!("Cursor: {} {}", cursor.0, cursor.1);
                    mat[(cursor.0 + msize) as usize][(cursor.1 + msize) as usize] = sum_up(cursor.0, cursor.1, mat);
                } else { 
                    s = State::Down; 
                    stepLeft = stepMax;
                }
            },
            State::Down => {
                if stepLeft > 0 {
                    stepLeft -= 1;
                    cursor.1 -= 1;
                    counter += 1;
        println!("Cursor: {} {}", cursor.0, cursor.1);
                    mat[(cursor.0 + msize) as usize][(cursor.1 + msize) as usize] = sum_up(cursor.0, cursor.1, mat);
                } else {
                    s = State::Right; 
                    stepMax += 1;
                    stepLeft = stepMax;
                }
            },
        }
    }
    println!("Dist: {}", cursor.0 + cursor.1);
    println!("ANS: {}", mat[(cursor.0 + msize) as usize][(cursor.1 + msize) as usize]);
}
