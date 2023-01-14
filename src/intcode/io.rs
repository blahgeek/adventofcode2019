use std::{collections::VecDeque, ops::Deref};

use super::inst::Word;

pub trait Input {
    fn read(&mut self) -> Option<Word>;
}

pub trait Output {
    fn write(&mut self, val: Word);
}


pub struct BufferInput {
    inputs: VecDeque<Word>,
}

impl Input for BufferInput {
    fn read(&mut self) -> Option<Word> {
        self.inputs.pop_front()
    }
}

impl BufferInput {
    pub fn new(vals: &[Word]) -> Self {
        BufferInput { inputs: VecDeque::from(Vec::<Word>::from(vals)) }
    }
}


#[derive(Default)]
pub struct BufferOutput {
    outputs: Vec<Word>,
}

impl Output for BufferOutput {
    fn write(&mut self, val: Word) {
        self.outputs.push(val)
    }
}

impl Deref for BufferOutput {
    type Target = [Word];

    fn deref(&self) -> &Self::Target {
        self.outputs.deref()
    }
}
