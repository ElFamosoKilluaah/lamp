pub mod command_base;

// Commands modules declarations
pub mod regdump;
pub mod setpc;
pub mod step;

use command_base::DebugCommand;

pub fn get_cmds() -> Vec<Box<dyn DebugCommand>> {
    vec![
        Box::new(regdump::RegdumpCommand {}),
        Box::new(setpc::SetPcCommand {}),
        Box::new(step::StepCommand {}),
    ]
}

pub fn parse_cmd(string: &str) -> Option<Box<dyn DebugCommand>> {
    for command in get_cmds() {
        if string.contains(command.name()) {
            return Some(command);
        }
    }
    None
}
