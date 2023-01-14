use std::{io, ops::Deref};
use adv2019::intcode;

fn run_with_input(prog: &str, input: i64) -> i64 {
    let mem: Vec<i64> = prog.trim_end().split(",").map(|x| x.parse().unwrap()).collect();
    let mut computer = intcode::computer::IntcodeComputer::new(
        mem, intcode::io::BufferInput::new(&[input]), intcode::io::BufferOutput::default(),
    );
    computer.run_until_finish();

    let outputs: &[i64] = computer.output_ref().deref();
    for i in 0..(outputs.len()-1) {
        assert_eq!(outputs[i], 0);
    }
    *outputs.last().unwrap()
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    println!("{}", run_with_input(&line, 1));
    println!("{}", run_with_input(&line, 5));
}

#[test]
fn final_result() {
    assert_eq!(run_with_input(include_str!("../../input/5"), 1), 12440243);
    assert_eq!(run_with_input(include_str!("../../input/5"), 5), 15486302);
}
