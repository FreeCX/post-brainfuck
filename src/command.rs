#[derive(Debug, Clone, Copy)]
pub enum Command {
    Next,
    Previous,
    Increment,
    Decrement,
    Put,
    Read,
    LoopBegin(usize),
    LoopEnd(usize),
}

impl From<char> for Command {
    fn from(token: char) -> Self {
        match token {
            '>' => Command::Next,
            '<' => Command::Previous,
            '+' => Command::Increment,
            '-' => Command::Decrement,
            '.' => Command::Put,
            ',' => Command::Read,
            '[' => Command::LoopBegin(0),
            ']' => Command::LoopEnd(0),
            token => panic!("unknown token `{}`", token),
        }
    }
}

impl Command {
    pub fn is_valid_token(t: char) -> bool {
        t == '>' || t == '<' || t == '+' || t == '-' || t == '.' || t == ',' || t == '[' || t == ']'
    }
}
