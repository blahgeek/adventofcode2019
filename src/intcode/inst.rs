pub type Word = i64;

#[derive(Clone, Copy, Debug)]
pub enum Parameter {
    AbsPosition(usize),
    RelPosition(Word),
    Immediate(Word),
}

impl Parameter {
    pub fn new(mode: i8, val: Word) -> Self {
        match mode {
            1 => Parameter::Immediate(val),
            0 => Parameter::AbsPosition(val as usize),
            2 => Parameter::RelPosition(val),
            _ => unimplemented!("unsupported mode"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Operation {
    Add, Multiply, Input, Output,
    JumpIfTrue, JumpIfFalse, LessThan, Equals,
    AdjustRelativeBase,
    Halt,
}

impl Operation {
    pub fn instruction_len(&self) -> usize {
        match self {
            Self::Add | Self::Multiply | Self::LessThan | Self::Equals => 4,
            Self::JumpIfTrue | Self::JumpIfFalse => 3,
            Self::Input | Self::Output => 2,
            Self::AdjustRelativeBase => 2,
            Self::Halt => 1,
        }
    }
}

impl From<i64> for Operation {
    fn from(opcode: i64) -> Self {
        match opcode {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            9 => Self::AdjustRelativeBase,
            _ => Self::Halt,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub op: Operation,
    pub params: Vec<Parameter>,
}
