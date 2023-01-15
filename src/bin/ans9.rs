use std::ops::Deref;
use std::io;

use adv2019::intcode;

fn run(prog: &str, inputs: &[i64]) -> Vec<i64> {
    let mem: Vec<i64> = prog.trim_end().split(",").map(|x| x.parse().unwrap()).collect();
    let mut computer = intcode::computer::IntcodeComputer::new(
        mem,
        intcode::io::BufferInput::new(inputs),
        intcode::io::BufferOutput::default()
    );
    computer.run_until_finish();
    computer.output_ref().deref().into()
}

#[test]
fn test_example() {
    assert_eq!(run("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99", &[]),
               vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
    assert_eq!(run("1102,34915192,34915192,7,4,7,99,0", &[]).first().unwrap().to_string().len(), 16);
    assert_eq!(run("104,1125899906842624,99", &[]), vec![1125899906842624]);
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    println!("{:?}", run(&line, &[1_i64]));
    println!("{:?}", run(&line, &[2_i64]));
}

#[test]
fn test_final() {
    assert_eq!(run(include_str!("../../input/9"), &[1_i64]), vec![3241900951]);
    assert_eq!(run(include_str!("../../input/9"), &[2_i64]), vec![83089]);
}
