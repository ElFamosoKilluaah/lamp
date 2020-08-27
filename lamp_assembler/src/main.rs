use crate::compiler::Compiler;
use std::fs::File;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

mod compiler;

#[derive(StructOpt, Debug)]
#[structopt(name = "lamp_asm")]
struct LampAsm {
    #[structopt(short)]
    source: PathBuf,
    #[structopt(short)]
    output: PathBuf,
}

fn main() {
    let args = LampAsm::from_args();

    let mut file = File::open(&args.source);

    match file {
        Ok(ref mut file) => {
            let mut buffer = String::new();
            match file.read_to_string(&mut buffer) {
                Ok(k) => {
                    println!("Source file size is {} bytes.", k);
                    let lines = buffer.split('\n').map(|s| s.to_string()).collect();

                    let mut compiler = Compiler::new(lines);
                    match compiler.compile() {
                        Ok(bin_size) => {
                            println!("Output's size is {} bytes.", bin_size);
                            match write_output(&args.output, compiler.result_buffer) {
                                Ok(_) => println!("Compilation successfully ended."),
                                Err(e) => println!(
                                    "Compilation failed: Cannot write the output. Error: {:?}",
                                    e
                                ),
                            }
                        }
                        Err(errors) => {
                            eprintln!("Can\'t compile the source file: {} problems found. But don\'t worry ! Here are the listed mistakes: ", errors.len());
                            for error in errors {
                                eprintln!("{}", error);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: cannot read the file.\nDetails: {:?}", e);
                }
            }
        }
        Err(e) => eprintln!("Error: cannot open {:?}: {:?}", args.source, e),
    }

    fn write_output(path: &PathBuf, bin: Vec<u8>) -> Result<usize, std::io::Error> {
        let buff = bin.as_slice();
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write(&buff)
    }
}
