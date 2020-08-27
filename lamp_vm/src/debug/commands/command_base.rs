use crate::base::vm::VM;

pub trait DebugCommand {
    fn execute(&self, vm: &mut VM, args: Vec<&str>) -> usize;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn syntax(&self) -> &str;
}
