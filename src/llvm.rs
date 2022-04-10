use std::io::{self, Write};

use crate::command::Command;
use crate::emulator::Emulator;
use crate::transpiler::TranspilerLLVM;

impl TranspilerLLVM for Emulator {
    fn into_code<W: Write>(self, writer: &mut W) -> Result<(), io::Error> {
        use Command::*;

        let mut code = String::new();
        for (index, cmd) in self.iter_command().enumerate() {
            match &cmd {
                Next => {
                    code.push_str(&format!(
                        include_str!("../templates/llvm/next.txt"),
                        index = index,
                        max_size = self.mem_size()
                    ));
                }
                Previous => code.push_str(&format!(include_str!("../templates/llvm/prev.txt"), index = index)),
                Increment => {
                    code.push_str(&format!(
                        include_str!("../templates/llvm/inc-dec.txt"),
                        cmd = "add",
                        amount = 1,
                        index = index
                    ));
                }
                Decrement => {
                    code.push_str(&format!(
                        include_str!("../templates/llvm/inc-dec.txt"),
                        cmd = "sub",
                        amount = 1,
                        index = index
                    ));
                }
                Put => code.push_str(&format!(include_str!("../templates/llvm/put.txt"), index = index)),
                Read => code.push_str(&format!(include_str!("../templates/llvm/read.txt"), index = index)),
                LoopBegin(end) => {
                    code.push_str(&format!(include_str!("../templates/llvm/loop-begin.txt"), start = index, end = end))
                }
                LoopEnd(start) => {
                    code.push_str(&format!(include_str!("../templates/llvm/loop-end.txt"), start = start, end = index));
                }
            }
        }

        write!(writer, include_str!("../templates/llvm/template.txt"), mem_size = self.mem_size(), code = code)
    }
}
