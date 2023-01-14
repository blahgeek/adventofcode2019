use std::io;
use std::collections::VecDeque;
use std::ops::Deref;
use adv2019::intcode;

fn run_amplifier(mem: Vec<i64>, phase_setting: i64, input: i64) -> i64 {
    let mut computer = intcode::computer::IntcodeComputer::new(
        mem,
        intcode::io::BufferInput{ inputs: VecDeque::from([phase_setting, input]) },
        intcode::io::BufferOutput::default()
    );
    computer.run_until_finish();
    *computer.output_ref().deref().last().unwrap()
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
