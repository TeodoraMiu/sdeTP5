use nix::unistd::{fork, ForkResult, getpid, getppid, execvp};
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use std::{thread, time};
use std::ffi::CString;
use nix::unistd::dup2;
use std::fs::File;
use std::os::unix::io::AsRawFd;

fn main(){
    match unsafe{fork()} {
        Ok(ForkResult::Parent { child }) => {
            println!("child pid in parent: {}", child);
            println!("parent pid in parent: {}", getpid());
            println!("waiting for child process to end");
            let options: Option<WaitPidFlag> = None;
            let status = match waitpid(child, options){
                Ok(WaitStatus::Exited(pid, status)) => status,
                _ => 0,
            };
            println!("child is done with exit code {}", status);
        }
        Ok(ForkResult::Child) => {
            println!("child pid in child: {}", getpid());
            println!("parents pid in child: {}", getppid());
            let seconds = time::Duration::from_millis(2000);
            thread::sleep(seconds);
            let mut file = File::create("output.txt").unwrap();
            dup2(file.as_raw_fd(), 1);
            println!("time has passed");
            println!("child pid in child: {}", getpid());
            println!("parents pid in child: {}", getppid());
            execvp(&CString::new("ls").unwrap(), &[CString::new("ls").unwrap(), CString::new("-l").unwrap()]);
            std::process::exit(1);
        }
        Err(_) => println!("Fork failed"),
     }
}
