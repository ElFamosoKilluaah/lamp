use base::vm::VM;
use debug::session::DebugSession;
use log::{error, info};
use std::path::PathBuf;
use structopt::StructOpt;

pub mod base;
pub mod debug;
#[cfg(test)]
mod tests;

#[derive(StructOpt, Debug)]
#[structopt(name = "lamp")]
struct LampApp {
    #[structopt(short)]
    bin_path: PathBuf,

    #[structopt(short, long)]
    debug: bool,
}

fn main() {
    let lamp = LampApp::from_args();
    simple_logger::init().unwrap();
    let bin = std::fs::read(&lamp.bin_path);

    match bin {
        Ok(v) => {
            if lamp.debug {
                info!("Debug session started.");
                let mut debug_session = DebugSession::new(VM::new(v));
                debug_session.start_debug_session();
                info!("Debug session ended.");
                return;
            }

            let mut lamp_vm = VM::new(v);
            let exit_status = lamp_vm.run();

            match exit_status {
                Ok(code) => info!("VM exited successfully (code {})", code),
                Err(e) => error!("VM exited with an error.\nReason: {:?}", e),
            }
        }
        Err(e) => error!("Unable to read the binary's content: {:?}", e),
    }

    info!("VM Shutdown.");
}
