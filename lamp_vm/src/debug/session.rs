use super::commands;
use crate::base::vm::VM;
use log::{error, info};
use std::io;
use std::io::Write;

pub struct DebugSession {
    vm: VM,
}

impl DebugSession {
    pub fn new(vm: VM) -> Self {
        Self { vm }
    }

    pub fn start_debug_session(&mut self) {
        info!("Starting debug session.");
        loop {
            print!("\n>>> ");
            let _ = io::stdout().flush();
            let mut command_line = String::new();

            io::stdin()
                .read_line(&mut command_line)
                .map_err(|e| {
                    error!("Unable to read line: {:?}", e);
                })
                .unwrap();

            command_line = String::from(command_line.trim());
            match commands::parse_cmd(&command_line) {
                Some(cmd) =>  {
                    let res = cmd.execute(&mut self.vm, command_line.split(' ').collect());
                    info!("Command reported the {} result.", res);
                },
                None => error!("This command doesn\'t exist. type \"help\" for an exhaustive list of the available commands."),
            }
        }
    }
}
