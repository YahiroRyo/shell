use nix::{
    sys::wait::waitpid,
    unistd::{execvp, fork, ForkResult}
};
use std::{
    ffi::CString,
    io::{stdin, stdout, Write}
};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();
        
        let mut input_line = String::new();
        stdin().read_line(&mut input_line)
            .expect("[ERROR] Faild to read a line.");
        let command: Vec<&str> = input_line.split(' ').collect();
        let args = command
            .into_iter()
            .map(|c| CString::new(c).unwrap())
            .collect::<Vec<_>>();
        
        match unsafe { fork() } {
            Ok(ForkResult::Parent{ child, .. }) => {
                waitpid(child, None).unwrap();
            }
            Ok(ForkResult::Child) => {
                match execvp(&args[0], &args) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("[ERROR] {}", e);
                    }
                }
            }
            Err (e) => {
                println!("[FORK ERROR] {}", e);
            }
        }
    }
}