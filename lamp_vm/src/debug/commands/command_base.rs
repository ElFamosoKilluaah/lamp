use crate::base::vm::VM;
use log::error;

pub trait DebugCommand {
    fn execute(&self, vm: &mut VM, args: Vec<&str>) -> usize;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn syntax(&self) -> &str;
    fn display_error(&self) {
        error!(
            "Error: too much/not enough arguments.\nUsage: {}",
            self.syntax()
        );
    }
}
