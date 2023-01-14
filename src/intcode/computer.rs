use super::inst::*;
use super::io::*;

pub struct IntcodeComputer<IN, OUT> {
    mem: Vec<Word>,
    pc: usize,

    input: IN,
    output: OUT,
}

impl<IN, OUT> IntcodeComputer<IN, OUT>
where IN: Input, OUT: Output {

    pub fn new(initial_mem: Vec<Word>, input: IN, output: OUT) -> Self {
        IntcodeComputer {
            mem: initial_mem,
            pc: 0,
            input, output
        }
    }

    pub fn input_ref(&self) -> &IN {
        &self.input
    }

    pub fn output_ref(&self) -> &OUT {
        &&self.output
    }

    fn get_mem(&self, param: Parameter) -> Word {
        match param {
            Parameter::Position(i) => self.mem[i],
            Parameter::Immediate(v) => v,
        }
    }

    fn set_mem(&mut self, param: Parameter, val: Word) {
        match param {
            Parameter::Immediate(_) => panic!("Cannot set with immediate param"),
            Parameter::Position(i) => self.mem[i] = val,
        }
    }

    fn execute_one_instruction(&mut self, inst: Instruction) {
        let mut new_pc: Option<usize> = None;
        match inst.op {
            Operation::Add =>
                self.set_mem(inst.params[2], self.get_mem(inst.params[0]) + self.get_mem(inst.params[1])),
            Operation::Multiply =>
                self.set_mem(inst.params[2], self.get_mem(inst.params[0]) * self.get_mem(inst.params[1])),
            Operation::LessThan =>
                self.set_mem(inst.params[2], if self.get_mem(inst.params[0]) < self.get_mem(inst.params[1]) { 1 } else { 0 }),
            Operation::Equals =>
                self.set_mem(inst.params[2], if self.get_mem(inst.params[0]) == self.get_mem(inst.params[1]) { 1 } else { 0 }),
            Operation::Input => {
                let v = self.input.read().unwrap();
                self.set_mem(inst.params[0], v);
            },
            Operation::Output =>
                self.output.write(self.get_mem(inst.params[0])),
            Operation::JumpIfTrue => {
                if self.get_mem(inst.params[0]) != 0 {
                    new_pc = Some(self.get_mem(inst.params[1]) as usize);
                }
            },
            Operation::JumpIfFalse => {
                if self.get_mem(inst.params[0]) == 0 {
                    new_pc = Some(self.get_mem(inst.params[1]) as usize);
                }
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
        let inst = self.mem.get(self.pc)?;
        let op = Operation::from(inst % 100);
        let mut params = Vec::<Parameter>::new();
        for i in 0..(op.instruction_len()-1) {
            let v = *self.mem.get(self.pc+i+1)?;
            let param = match (inst / 100 / (10i64.pow(i as u32))) % 10 { // mode
                1 => Parameter::Immediate(v),
                0 => Parameter::Position(v as usize),
                _ => return None,
            };
            params.push(param);
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
