use std::io;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
enum Parameter {
    Position(usize),
    Immediate(i64),
}

#[derive(PartialEq, Debug)]
enum Operation {
    Add, Multiply, Input, Output,
    JumpIfTrue, JumpIfFalse, LessThan, Equals,
    Halt,
}

impl Operation {
    fn required_param_count(&self) -> usize {
        match self {
            Self::Add | Self::Multiply | Self::LessThan | Self::Equals => 3,
            Self::JumpIfTrue | Self::JumpIfFalse => 2,
            Self::Input | Self::Output => 1,
            Self::Halt => 0,
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
            _ => Self::Halt,
        }
    }
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    params: Vec<Parameter>,
}

struct IntcodeComputer {
    mem: Vec<i64>,
    pc: usize,

    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
}

impl IntcodeComputer {

    fn get_mem(&self, param: Parameter) -> i64 {
        match param {
            Parameter::Position(i) => self.mem[i],
            Parameter::Immediate(v) => v,
        }
    }

    fn set_mem(&mut self, param: Parameter, val: i64) {
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
                let v = self.inputs.pop_front().unwrap();
                self.set_mem(inst.params[0], v);
            },
            Operation::Output =>
                self.outputs.push(self.get_mem(inst.params[0])),
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
            self.pc += inst.op.required_param_count() + 1;
        }
    }

    fn parse_next_instruction(&self) -> Option<Instruction> {
        let inst = self.mem.get(self.pc)?;
        let op = Operation::from(inst % 100);
        let mut params = Vec::<Parameter>::new();
        for i in 0..op.required_param_count() {
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

    fn run_all(&mut self) {
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

fn run_amplifier(mem: Vec<i64>, phase_setting: i64, input: i64) -> i64 {
    let mut computer = IntcodeComputer {
        mem,
        pc: 0,
        inputs: VecDeque::from([phase_setting, input]),
        outputs: Vec::new(),
    };
    computer.run_all();
    *computer.outputs.last().unwrap()
}

fn run_amplifier_chain(mem: Vec<i64>, phase_settings: &[usize; 5]) -> i64 {
    let mut last_output = 0;
    for i in 0..5 {
        last_output = run_amplifier(mem.clone(), phase_settings[i] as i64, last_output);
    }
    last_output
}

fn generate_permutations<const N: usize>() -> Vec<[usize; N]> {
    fn _helper_recursive<const N: usize>(idx: usize, mask: u32, current: &mut [usize; N], result: &mut Vec<[usize; N]>) {
        if idx == N {
            result.push(current.clone());
            return;
        }
        for i in 0..N {
            if (mask & (1 << i)) != 0 {
                current[idx] = i;
                _helper_recursive(idx+1, mask & !(1<<i), current, result);
            }
        }
    }
    let mut result: Vec<[usize; N]> = Vec::new();
    _helper_recursive::<N>(0, 0xffffffff, &mut [0; N], &mut result);
    result
}

#[test]
fn test_permutations() {
    assert_eq!(generate_permutations::<5>().len(), 120);
}


fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mem: Vec<i64> = line.trim_end().split(",").map(|x| x.parse().unwrap()).collect();

    let ans = generate_permutations::<5>().iter()
        .map(|phase_settings| run_amplifier_chain(mem.clone(), &phase_settings))
        .max();
    println!("{}", ans.unwrap());
}
