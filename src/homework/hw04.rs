const SIZE: usize = 6; // 

fn main() {
    let mut output = String::new();

    // 
    for y in 0..SIZE {
        for x in 0..(2 * SIZE - 1) {
            if x >= SIZE - 1 - y && x <= SIZE - 1 + y {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    // 
    for y in (0..SIZE - 1).rev() {
        for x in 0..(2 * SIZE - 1) {
            if x >= SIZE - 1 - y && x <= SIZE - 1 + y {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
