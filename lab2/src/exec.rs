use crate::parse::{StdinType, StdoutMode, StdoutType};
use crate::{Cmd, Result};
use libc::{kill, pid_t, SIGINT};
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::process::{Child, Command, Stdio};

thread_local! {
    static EXEC_PIDS: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

pub fn clear_exec_pids() {
    EXEC_PIDS.with(|c| {
        for pid in c.borrow().iter() {
            unsafe {
                kill(*pid as pid_t, SIGINT);
            }
        }

        c.borrow_mut().clear();
    });
}

impl Cmd {
    pub fn execute(&self) -> Result<i32> {
        let mut piped = Option::<Child>::None;
        for sub_cmd in self.sub_cmds.iter() {
            let prog = sub_cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            let args = if sub_cmd.args.len() > 1 {
                &sub_cmd.args[1..]
            } else {
                &[]
            };

            piped = Some({
                let child = Command::new(prog)
                    .args(args)
                    .stdin(match &sub_cmd.stdin {
                        StdinType::Inherit => Stdio::inherit(),
                        StdinType::Piped => Stdio::from(piped.unwrap().stdout.unwrap()),
                        StdinType::Redirected(path) => Stdio::from(
                            File::open(path)
                                .map_err(|_| format!("Can not open file to read: {}", path))?,
                        ),
                    })
                    .stdout(match &sub_cmd.stdout {
                        StdoutType::Inherit => Stdio::inherit(),
                        StdoutType::Piped => Stdio::piped(),
                        StdoutType::Redirected(path, mode) => match mode {
                            StdoutMode::Overwrite => Stdio::from(
                                File::create(path)
                                    .map_err(|_| format!("Can not open file to write: {}", path))?,
                            ),
                            StdoutMode::Append => Stdio::from(
                                OpenOptions::new()
                                    .append(true)
                                    .create(true)
                                    .open(path)
                                    .map_err(|_| {
                                        format!("Can not open file to append: {}", path)
                                    })?,
                            ),
                        },
                    })
                    .spawn()
                    .map_err(|_| "Execution failed.")?;

                // Save pid.
                EXEC_PIDS.with(|c| {
                    c.borrow_mut().push(child.id());
                });

                child
            })
        }

        let exit_status = piped.unwrap().wait().map_err(|e| e.to_string())?;

        Ok(exit_status.code().ok_or("Terminated by signal.")?)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}