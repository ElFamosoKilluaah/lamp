use super::commands;
use crate::base::vm::{VMResult, VM};
use log::{error, info, warn};
use std::io;

pub struct DebugSession {
    vm: VM,
}

impl DebugSession {
    pub fn new(vm: VM) -> Self {
        Self { vm }
    }

    pub fn start_debug_session(&mut self) -> VMResult {
        info!("Starting debug session.");
        loop {
            let mut command_line = String::new();

            io::stdin()
                .read_line(&mut command_line)
                .map_err(error!("Unable to read line."))
                .unwrap();

            match commands::parse_cmd(&command_line) {
                Some(cmd) => cmd.execute(&mut self.vm, command_line.split(' ').collect()),
                None =>
                    error!("This command doesn\'t exist. type \"help\" for an exhaustive list of the available commands."),
            }
        }
    }
}
