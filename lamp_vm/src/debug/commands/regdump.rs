use super::command_base::DebugCommand;
use crate::base::vm::VM;
use log::{error, info};

pub struct RegdumpCommand;

impl DebugCommand for RegdumpCommand {
    fn execute(&self, vm: &mut VM, args: Vec<&str>) -> usize {
        match args.get(1) {
            Some(arg) => {
                if arg == &"all" {
                    for i in 0..32 {
                        print!("|| {}: {}", i, vm.get_register(i));
                    }
                    0
                } else {
                    match arg.parse::<u8>() {
                        Ok(num) => {
                            if num > 31 {
                                error!("Error: wrong arg 2.\nUsage: {}", self.syntax());
                                return 1;
                            }
                            info!("Register {} = {}", num, vm.get_register(num));
                            0
                        }
                        Err(_) => {
                            error!("Error: wrong arg 2: \'{}\'\nUsage: {}", arg, self.syntax());
                            1
                        }
                    }
                }
            }
            None => {
                self.display_error();
                1
            }
        }
    }

    fn name(&self) -> &str {
        "regdump"
    }

    fn description(&self) -> &str {
        "Prints the value of the specified register, or all if specified"
    }

    fn syntax(&self) -> &str {
        "regdump <0..31 | all>"
    }
}
