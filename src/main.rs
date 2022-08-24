use std::io;

fn main() -> io::Result<()> {
    lsh_loop();
    Ok(())
}

fn lsh_loop() {
    let mut line = String::new();
    let mut args = String::new();
    let status: i32;
    loop {
        println!("> ");
        line = lsh_read_line();
        // args = lsh_split_line(line);
        // status = lsh_execute(args);
        println!("{}", line);
    }
}

fn lsh_read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input
}
