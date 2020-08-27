pub mod command_base;
use command_base::DebugCommand;

pub fn parse_cmd(string: &String) -> Option<Box<dyn DebugCommand>> {
    todo!();
}
