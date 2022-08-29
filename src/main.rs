use std::{io, fmt, str::SplitWhitespace};
use std::error;
use std::path::Path;
use std::process::Command;
use fork::{fork, Fork};
use nix::sys::wait::*;
use nix::unistd::*;

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
        lsh_execute(&line);
    }
}

fn lsh_read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn lsh_launch(command: &str, args: Vec<String>) -> Result<Status, LshError> {
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

fn lsh_cd(args: &str) -> Result<Status, LshError> {
    if args.is_empty() {
        Err(LshError::new("error"))
    } else {
        chdir(Path::new(&args))
            .map(|_| Status::Success)
            .map_err(|err| LshError::new(&err.to_string()))
    }
}

fn lsh_help() -> Result<Status, LshError> {
    println!("LSH-rs\n");
    println!("Type program names and arguments, and hit enter\n");
    println!("The following are built in:\n");
    println!("Use the man command for information on other programs.\n");
    Ok(Status::Success)
}

fn lsh_exit() -> Result<Status, LshError> {
    Ok(Status::Exit)
}

fn lsh_execute(line: &str) -> Result<Status, LshError> {
    let mut parts = line.split_whitespace();
    let command = parts.next().unwrap();
    let args: Vec<String> = parts.map(|token: &str| String::from(token)).collect();
    if command.is_empty() {
        Ok(Status::Success)
    } else {
        match command {
            "cd" => lsh_cd(&args[0]),
            "exit" => lsh_exit(),
            "help" => lsh_help(),
            _ => lsh_launch(&command, args)
        }
    }

}
