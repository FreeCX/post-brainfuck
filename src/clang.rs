use std::io::{self, Write};

use crate::command::Command;
use crate::emulator::Emulator;
use crate::transpiler::TranspilerC;

impl TranspilerC for Emulator {
    fn into_code<W: Write>(self, writer: &mut W) -> Result<(), io::Error> {
        let mut code = String::new();

        for token in self.iter_command() {
            match token {
                Command::Next => code.push_str("\tif (mp + 1 > MAX_MEM_SIZE) out_of_memory();\n\tmp += 1;\n"),
                Command::Previous => code.push_str("\tif (mp == 0) negative_memory();\n\tmp -= 1;\n"),
                Command::Increment => code.push_str("\tmem[mp] += 1;\n"),
                Command::Decrement => code.push_str("\tmem[mp] -= 1;\n"),
                Command::Put => code.push_str("\tputchar(mem[mp]);\n"),
                Command::Read => code.push_str("\tmem[mp] = getchar();\n"),
                Command::LoopBegin(_) => code.push_str("\twhile (mem[mp] != 0) {\n"),
                Command::LoopEnd(_) => code.push_str("\tif (mem[mp] == 0) break;\n\t}\n"),
            }
        }

        write!(writer, include_str!("../templates/c-template.txt"), mem_size = self.mem_size(), code = code)
    }
}
