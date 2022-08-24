use std::{io, str::SplitWhitespace};

fn main() -> io::Result<()> {
    lsh_loop();
    Ok(())
}

fn lsh_loop() {
    let mut line = String::new();
    let status: i32;
    loop {
        println!("> ");
        line = lsh_read_line();
        let mut args = lsh_split_line(&line);
        // status = lsh_execute(args);
        println!("{} {}", line, args.next().unwrap());
    }
}

fn lsh_read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn lsh_split_line(line: &str) -> SplitWhitespace {
    line.split_whitespace()
}