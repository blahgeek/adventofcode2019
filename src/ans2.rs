use std::io;

struct Machine {
    mem: Vec<i64>,
    cursor: usize,
}

impl Machine {
    fn _value_ref(&mut self, offset: i32) -> &mut i64 {
        let pos = self.mem[self.cursor + offset as usize];
        &mut self.mem[pos as usize]
    }
    // return true if should stop
    fn run_one_step(&mut self) -> bool {
        let skips = match self.mem[self.cursor] {
            1 => {
                *self._value_ref(3) = *self._value_ref(1) + *self._value_ref(2);
                4
            },
            2 => {
                *self._value_ref(3) = *self._value_ref(1) * *self._value_ref(2);
                4
            },
            _ => 0,
        };
        self.cursor += skips;
        return skips == 0;
    }
    pub fn run(&mut self) -> i64 {
        while !self.run_one_step() {}
        self.mem[0]
    }
}

#[test]
fn test_machine() {
    let mut machine = Machine {
        cursor: 0,
        mem: vec![1,9,10,3,2,3,11,0,99,30,40,50],
    };
    let res = machine.run();
    assert_eq!(res, 3500);
}

fn run_machine_with_noun_verb(orig_mem: &Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut mem = orig_mem.clone();
    mem[1] = noun;
    mem[2] = verb;
    let mut machine = Machine {
        cursor: 0,
        mem,
    };
    machine.run()
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mem: Vec<i64> = line.trim_end().split(",").map(|x| x.parse().unwrap()).collect();

    println!("1: {}", run_machine_with_noun_verb(&mem, 12, 2));

    for noun in 0i64..100 {
        for verb in 0i64..100 {
            if run_machine_with_noun_verb(&mem, noun, verb) == 19690720 {
                println!("2: {}, {}: {}", noun, verb, noun * 100 + verb);
                break;
            }
        }
    }
}
