use super::command_base::DebugCommand;
use crate::base::vm::VM;

pub struct StepCommand;

impl DebugCommand for StepCommand {
    #[allow(unused_variables)]
    fn execute(&self, vm: &mut VM, args: Vec<&str>) -> usize {
        match vm.cycle() {
            Ok(_) => 0,
            Err(_) => 1,
        }
    }
    fn name(&self) -> &str {
        "step"
    }
    fn description(&self) -> &str {
        "Makes 1 VM's cycle."
    }
    fn syntax(&self) -> &str {
        "step"
    }
}
