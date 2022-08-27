use std::{io, fmt, str::SplitWhitespace};
use std::error;
use std::process::Command;
use fork::{fork, Fork};
use nix::sys::wait::*;

#[derive(Debug, Clone)]
struct LshError {
    pub message: String,
}

impl LshError {
    pub fn new(message: &str) -> LshError {
        LshError { message: message.to_string() }
    }
}

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
        lsh_launch(&line);
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

fn lsh_launch(line: &str) -> Result<Status, LshError> {
    let pid = fork();
    match pid {
        Ok(Fork::Parent(child)) => {
            let wpid_result = waitpid(child, None)
                .map_err(|err| LshError::new(&format!("{}", err)));
            match wpid_result {
                Ok(WaitStatus::Exited(_, _)) => Ok(Status::Success),
                Ok(WaitStatus::Signaled(_, _, _)) => Ok(Status::Success),
                Err(err) => Err(LshError::new(&err.message)),
                _ => Ok(Status::Success),
            }
        }
        Ok(Fork::Child) => {
            let mut parts = line.split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;
            let output = Command::new(command)
                .args(args)
                .spawn()
                .map_err(|err| LshError::new(&format!("{}", err)));

            match output {
                Ok(output) => Ok(Status::Success),
                Err(err) => Err(LshError::new(&err.message)),
            }
        }
        Err(_) => Err(LshError::new("fork error"))
    }
}
