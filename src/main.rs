extern crate argparse;

use std::fs::File;
use std::io::BufWriter;

use transpiler::{TranspilerC, TranspilerLLVM};

pub mod clang;
pub mod command;
pub mod consts;
pub mod emulator;
pub mod error;
pub mod llvm;
pub mod transpiler;

fn main() {
    let mut filename = String::from("-");
    let mut output = String::new();
    let mut mem_size = consts::DEFAULT_MEM_SIZE;
    let mut format = String::from("llvm");

    {
        let mut app = argparse::ArgumentParser::new();
        app.set_description("Brainfuck emulator in Rust");
        app.refer(&mut filename).add_option(
            &["-i", "--input"],
            argparse::Store,
            "Specify input file or use stdin [default: '-']",
        );
        app.refer(&mut mem_size).add_option(
            &["-m", "--memory"],
            argparse::Store,
            "Specify memory size [default: 30000]",
        );
        app.refer(&mut format).add_option(&["-f", "--format"], argparse::Store, "Specify output format (c|llvm)");
        app.refer(&mut output).add_option(&["-o", "--output"], argparse::Store, "Specify output file");
        app.parse_args_or_exit();
    }

    let mut emulator = emulator::Emulator::new(mem_size);
    let result = if filename == "-" { emulator.from_stdin() } else { emulator.from_file(&filename) };
    if let Err(err) = result {
        panic!("[error] {}: {}", filename, err)
    }

    if !output.is_empty() {
        match &format[..] {
            "c" => {
                let file = File::create(output).unwrap();
                let mut writer = BufWriter::new(file);
                let _ = TranspilerC::into_code(emulator, &mut writer);
            }
            "llvm" => {
                let file = File::create(output).unwrap();
                let mut writer = BufWriter::new(file);
                let _ = TranspilerLLVM::into_code(emulator, &mut writer);
            }
            other => println!("error: format `{other}` does not support!"),
        }
    } else if let Err(err) = result.and_then(|_| emulator.execute()) {
        println!("[error] {}: {}", filename, err)
    }
}
