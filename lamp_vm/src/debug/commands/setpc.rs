use super::command_base::DebugCommand;
use crate::base::vm::VM;

pub struct SetPcCommand;

impl DebugCommand for SetPcCommand {
    fn execute(&self, vm: &mut VM, args: Vec<&str>) -> usize {
        match args.get(1) {
            Some(arg) => match arg.parse::<usize>() {
                Ok(new_pc) => {
                    vm.set_pc(new_pc);
                    0
                }
                Err(_) => {
                    self.display_error();
                    1
                }
            },
            None => {
                self.display_error();
                1
            }
        }
    }

    fn name(&self) -> &str {
        "setpc"
    }
    fn description(&self) -> &str {
        "Sets the program counter to the given index."
    }
    fn syntax(&self) -> &str {
        "setpc <unsigned value>"
    }
}
