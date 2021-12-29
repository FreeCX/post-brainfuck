extern crate argparse;

pub mod command;
pub mod consts;
pub mod emulator;
pub mod error;

fn main() {
    let mut filename = String::from("-");
    let mut mem_size = consts::DEFAULT_MEM_SIZE;

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
        app.parse_args_or_exit();
    }

    let mut emulator = emulator::Emulator::new(mem_size);
    let result = if filename == "-" { emulator.from_stdin() } else { emulator.from_file(&filename) };
    match result.and_then(|_| emulator.execute()) {
        Err(err) => println!("[error] {}: {}", filename, err),
        _ => (),
    }
}
