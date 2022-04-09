use std::io::{self, Write};

pub trait TranspilerC {
    fn into_code<W: Write>(self, writer: &mut W) -> Result<(), io::Error>;
}

pub trait TranspilerLLVM {
    fn into_code<W: Write>(self, writer: &mut W) -> Result<(), io::Error>;
}
