use super::inst::*;
use super::io::*;

use std::collections::BTreeMap;

pub struct IntcodeComputer<IN, OUT> {
    mem: BTreeMap<usize, Word>,
    pc: usize,
    relative_base: usize,

    input: IN,
    output: OUT,
}

impl<IN, OUT> IntcodeComputer<IN, OUT>
where IN: Input, OUT: Output {

    pub fn new(initial_mem: Vec<Word>, input: IN, output: OUT) -> Self {
        let mem : BTreeMap<usize, Word> =
            BTreeMap::from_iter(initial_mem.into_iter().enumerate());
        IntcodeComputer {
            mem,
            pc: 0,
            relative_base: 0,
            input, output
        }
    }

    pub fn input_ref(&self) -> &IN {
        &self.input
    }

    pub fn output_ref(&self) -> &OUT {
        &&self.output
    }

    fn read_param(&self, param: Parameter) -> Word {
        match param {
            Parameter::AbsPosition(i) => self.mem.get(&i).copied().unwrap_or(0),
            Parameter::RelPosition(i) =>
                self.mem.get(&((self.relative_base as i64 + i) as usize)).copied().unwrap_or(0),
            Parameter::Immediate(v) => v,
        }
    }

    fn write_param(&mut self, param: Parameter, val: Word) {
        match param {
            Parameter::Immediate(_) => panic!("Cannot set with immediate param"),
            Parameter::AbsPosition(i) => { self.mem.insert(i, val); },
            Parameter::RelPosition(i) => {
                self.mem.insert((self.relative_base as i64 + i) as usize, val);
            },
        }
    }

    fn execute_one_instruction(&mut self, inst: Instruction) {
        let mut new_pc: Option<usize> = None;
        match inst.op {
            Operation::Add =>
                self.write_param(inst.params[2], self.read_param(inst.params[0]) + self.read_param(inst.params[1])),
            Operation::Multiply =>
                self.write_param(inst.params[2], self.read_param(inst.params[0]) * self.read_param(inst.params[1])),
            Operation::LessThan =>
                self.write_param(inst.params[2], if self.read_param(inst.params[0]) < self.read_param(inst.params[1]) { 1 } else { 0 }),
            Operation::Equals =>
                self.write_param(inst.params[2], if self.read_param(inst.params[0]) == self.read_param(inst.params[1]) { 1 } else { 0 }),
            Operation::Input => {
                let v = self.input.read().unwrap();
                self.write_param(inst.params[0], v);
            },
            Operation::Output =>
                self.output.write(self.read_param(inst.params[0])),
            Operation::JumpIfTrue => {
                if self.read_param(inst.params[0]) != 0 {
                    new_pc = Some(self.read_param(inst.params[1]) as usize);
                }
            },
            Operation::JumpIfFalse => {
                if self.read_param(inst.params[0]) == 0 {
                    new_pc = Some(self.read_param(inst.params[1]) as usize);
                }
            },
            Operation::AdjustRelativeBase => {
                let delta = self.read_param(inst.params[0]);
                self.relative_base = (self.relative_base as i64 + delta) as usize;
            },
            Operation::Halt => panic!("should not arrive here"),
        }
        if let Some(new_pc) = new_pc {
            self.pc = new_pc;
        } else {
            self.pc += inst.op.instruction_len()
        }
    }

    fn parse_next_instruction(&self) -> Option<Instruction> {
        let inst = self.read_param(Parameter::AbsPosition(self.pc));
        let op = Operation::from(inst % 100);
        let mut params = Vec::<Parameter>::new();
        for i in 0..(op.instruction_len()-1) {
            let v = self.read_param(Parameter::AbsPosition(self.pc+i+1));
            let mode = (inst / 100 / (10i64.pow(i as u32))) % 10;
            params.push(Parameter::new(mode as i8, v));
        }
        Some(Instruction { op, params })
    }

    pub fn run_until_finish(&mut self) {
        while let Some(inst) = self.parse_next_instruction() {
            // dbg!(&inst);
            if inst.op == Operation::Halt {
                break;
            } else {
                self.execute_one_instruction(inst);
            }
        }
    }
}
