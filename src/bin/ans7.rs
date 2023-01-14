use std::cell::RefCell;
use std::io;
use std::ops::Deref;
use std::sync::mpsc;
use adv2019::intcode;

struct ChannelInput {
    receiver: mpsc::Receiver<i64>,
}

impl intcode::io::Input for ChannelInput {
    fn read(&mut self) -> Option<i64> {
        self.receiver.recv().ok()
    }
}

struct ChannelOutput {
    sender: mpsc::Sender<i64>,
    extra_sender: Option<mpsc::Sender<i64>>,
}

impl intcode::io::Output for ChannelOutput {
    fn write(&mut self, val: i64) {
        let _ = self.sender.send(val); // it's ok to fail here
        if let Some(extra_sender) = &self.extra_sender {
            extra_sender.send(val).unwrap();
        }
    }
}

fn run_amplifier_chain(mem: Vec<i64>, phase_settings: &[usize; 5]) -> i64 {
    let mut last_output = 0;
    for i in 0..5 {
        let mut computer = intcode::computer::IntcodeComputer::new(
            mem.clone(),
            intcode::io::BufferInput::new(&[phase_settings[i] as i64, last_output]),
            intcode::io::BufferOutput::default()
        );
        computer.run_until_finish();
        last_output = *computer.output_ref().deref().last().unwrap();
    }
    last_output
}

fn run_amplifier_loop(mem: Vec<i64>, phase_settings: &[usize; 5]) -> i64 {
    // the last one is for output only
    let channels: [(RefCell<Option<mpsc::Sender<i64>>>, RefCell<Option<mpsc::Receiver<i64>>>); 6] =
        core::array::from_fn(|_| {
            let (sender, receiver) = mpsc::channel();
            (RefCell::new(Some(sender)), RefCell::new(Some(receiver)))
        });

    for i in 0..5 {
        channels[i].0.borrow().as_ref().unwrap().send(phase_settings[i] as i64).unwrap();
    }
    channels[0].0.borrow().as_ref().unwrap().send(0).unwrap();

    let computers: [intcode::computer::IntcodeComputer<ChannelInput, ChannelOutput>; 5] =
        core::array::from_fn(|i| {
            intcode::computer::IntcodeComputer::new(
                mem.clone(),
                ChannelInput { receiver: channels[i].1.take().unwrap(), },
                ChannelOutput {
                    sender: channels[(i+1)%5].0.take().unwrap(),
                    extra_sender: if i == 4 { Some(channels[5].0.take().unwrap()) } else { None }
                },
            )
        });

    let threads: Vec<std::thread::JoinHandle<_>> = computers.into_iter()
        .map(|mut computer| { std::thread::spawn(move || computer.run_until_finish()) })
        .collect();
    for th in threads {
        th.join().unwrap();
    }

    let final_receiver: mpsc::Receiver<i64> = channels[5].1.take().unwrap();
    let mut result = 0_i64;
    while let Ok(val) = final_receiver.recv() {
        result = val;
    }
    return result;
}

fn generate_permutations<const N: usize>(nums: &[usize; N]) -> Vec<[usize; N]> {
    fn _helper_recursive<const N: usize>(idx: usize, mask: u32,
                                         nums: &[usize; N], current: &mut [usize; N],
                                         result: &mut Vec<[usize; N]>) {
        if idx == N {
            result.push(current.clone());
            return;
        }
        for i in 0..N {
            if (mask & (1 << i)) != 0 {
                current[idx] = nums[i];
                _helper_recursive(idx+1, mask & !(1<<i), nums, current, result);
            }
        }
    }
    let mut result: Vec<[usize; N]> = Vec::new();
    _helper_recursive::<N>(0, 0xffffffff, nums, &mut [0; N], &mut result);
    result
}

#[test]
fn test_permutations() {
    assert_eq!(generate_permutations::<5>(&[0,1,2,3,4]).len(), 120);
}

fn solve(line: &str) -> (i64, i64) {
    let mem: Vec<i64> = line.trim_end().split(",").map(|x| x.parse().unwrap()).collect();
    let ans_0 = generate_permutations::<5>(&[0,1,2,3,4]).iter()
        .map(|phase_settings| run_amplifier_chain(mem.clone(), &phase_settings))
        .max();
    let ans_1 = generate_permutations::<5>(&[5,6,7,8,9]).iter()
        .map(|phase_settings| run_amplifier_loop(mem.clone(), &phase_settings))
        .max();

    (ans_0.unwrap(), ans_1.unwrap())
}


fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    println!("{:?}", solve(&line));
}

#[test]
fn test_final() {
    assert_eq!(solve(include_str!("../../input/7")), (30940, 76211147));
}
