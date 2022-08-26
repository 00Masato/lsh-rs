use std::{io, fmt, str::SplitWhitespace};
use std::error;
use fork::{fork, Fork};
use nix::sys::wait::*;

// type Result<T> = std::result::Result<T, LshError>;

#[derive(Debug, Clone)]
struct LshError;

impl fmt::Display for LshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for LshError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub enum Status {
    Success,
    Exit
}

fn main() -> io::Result<()> {
    lsh_loop();
    Ok(())
}

fn lsh_loop() {
    let mut line = String::new(); let status: i32;
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

fn lsh_launch(args: SplitWhitespace) -> Result<Status, LshError> {
    let mut pid = fork();
    match pid {
        Ok(Fork::Parent(child)) => {
            let wpid_result = waitpid(child, None);
            match wpid_result {
                Ok(WaitStatus::Exited(_, _)) => Ok(Status::Success),
                Ok(WaitStatus::Signaled(_, _, _)) => Ok(Status::Success),
                Err(err) => LshError
            }
        }
        Ok(Fork::Child) => {
            println!("child");
        }
        Err(err) => LshError
    }
}
