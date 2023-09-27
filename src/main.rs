use anyhow::{Context, Result};
use clap::Parser;
mod zktc;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use zktc::Zktc;

#[derive(Parser)]
#[clap(version = "0.1", author = "kinpoko", about = "ZKTC emulator")]
struct Args {
    /// rom file path
    rom_file_path: String,

    /// ram file path
    #[arg(long = "ram", default_value = "none")]
    ram_file_name: String,
}
fn main() -> Result<()> {
    let args = Args::parse();

    let rom_file = load_mem_file(args.rom_file_path)?;

    let ram_file = if args.ram_file_name.as_str() == "none" {
        vec![]
    } else {
        load_mem_file(args.ram_file_name)?
    };

    let mut zktc = Zktc::new(rom_file, ram_file)?;

    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("zktc-emu >> ");
        match readline {
            Ok(line) => {
                let trimed = line.trim();
                let cmd: Vec<&str> = trimed.split(' ').filter(|c| !c.is_empty()).collect();
                if !cmd.is_empty() {
                    if cmd[0] == "exit" {
                        println!("exit");
                        break;
                    }
                    zktc.do_cmd(cmd)?;
                }
            }
            Err(ReadlineError::Eof) => {
                println!("exit");
                break;
            }
            _ => {
                break;
            }
        }
    }
    Ok(())
}

fn load_mem_file(path: String) -> Result<Vec<u8>> {
    let f = std::fs::read_to_string(path.clone())
        .with_context(|| format!("could not read mem file '{}'", path))?;
    let f = f.split_whitespace().collect::<Vec<_>>();

    let mut bytes: Vec<u8> = vec![];
    for line in f {
        let mut hex =
            hex::decode(line).with_context(|| format!("could not decode '{}' to hex", line))?;
        bytes.append(&mut hex);
    }

    Ok(bytes)
}
