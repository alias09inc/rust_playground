use std::io;

fn read_buffer() -> i32 {
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(_) => buffer.trim().parse().expect("Failed to perse."),
        Err(e) => panic!("Failed to read line: {}", e)
    }
}

fn read_buffers() -> Vec<i32> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line.");
    buffer.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse."))
        .collect()
}


fn main() {
    let n = read_buffer();
    let pp = read_buffers();
    let mut sum = 0;
    for i in pp.iter() {
        sum += i;
    }

    sum *= -1;

    println!("{}", sum)
}
