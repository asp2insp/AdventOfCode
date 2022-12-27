use std::collections::VecDeque;

pub fn parse_program(input: String) -> Vec<isize> {
    input.split(",").flat_map(str::parse::<isize>).collect()
}

#[derive(PartialEq, Debug)]
pub enum ProgYield {
    Output(isize),
    Input,
    Halt,
}

impl ProgYield {
    pub fn unwrap(self) -> isize {
        match self {
            ProgYield::Output(i) => i,
            _ => panic!("unwrap called on a halt/input instruction"),
        }
    }
}

pub struct Computer {
    program: Vec<isize>,
    iptr: usize,
    input: VecDeque<isize>,
    relative_base: isize,
}

impl Computer {
    pub fn new(prog: Vec<isize>) -> Computer {
        Computer {
            program: prog,
            iptr: 0,
            input: VecDeque::new(),
            relative_base: 0,
        }
    }

    pub fn input(&mut self, item: isize) {
        self.input.push_back(item);
    }

    pub fn input_ascii(&mut self, prog: &[&str]) {
        prog.iter()
            .flat_map(|l| {
                l.chars()
                    .map(|c| c as u8 as isize)
                    .chain(std::iter::once(10))
            })
            .for_each(|i| {
                self.run_and_return_output();
                self.input(i);
            });
    }

    pub fn set(&mut self, loc: usize, item: isize) {
        self.program[loc] = item;
    }

    fn check_dest(&mut self, offset: usize, param_modes: isize) -> usize {
        let mode = param_modes / 10_isize.pow(offset as u32 - 1);
        let dest = (self.program[self.iptr + offset]
            + if mode == 2 { self.relative_base } else { 0 }) as usize;
        if dest >= self.program.len() {
            self.program.resize(dest + 100, 0);
        }
        dest
    }

    fn read_param(&self, loc: usize, mode: isize) -> isize {
        let prog_value = self.program[loc];
        match mode {
            // Position mode
            0 => {
                if prog_value as usize > self.program.len() {
                    0
                } else {
                    self.program[prog_value as usize]
                }
            }
            // Immediate mode
            1 => prog_value,
            // Relative mode
            2 => {
                let addr = (self.relative_base + prog_value) as usize;
                if addr > self.program.len() {
                    0
                } else {
                    self.program[addr]
                }
            }
            _ => unreachable!("Uknown param mode"),
        }
    }

    fn get_params(&self, n: usize, mut param_modes: isize) -> Vec<isize> {
        let mut ret = Vec::new();
        for i in 1..=n {
            let mode = param_modes % 10;
            ret.push(self.read_param(self.iptr + i, mode));
            param_modes = param_modes / 10;
        }
        ret
    }

    pub fn run_and_return_output(&mut self) -> ProgYield {
        loop {
            let opcode = self.program[self.iptr] % 100;
            let param_modes = self.program[self.iptr] / 100;
            match opcode {
                1 => {
                    // Add 1, 2, store 3
                    let params = self.get_params(2, param_modes);
                    let dest = self.check_dest(3, param_modes);
                    self.program[dest] = params[0] + params[1];
                    self.iptr += 4;
                }
                2 => {
                    // Mult 1, 2, store 3
                    let params = self.get_params(2, param_modes);
                    let dest = self.check_dest(3, param_modes);
                    self.program[dest] = params[0] * params[1];
                    self.iptr += 4;
                }
                3 => {
                    // Input and store 1
                    let dest = self.check_dest(1, param_modes);
                    if self.input.is_empty() {
                        return ProgYield::Input;
                    }
                    self.program[dest] = self.input.pop_front().unwrap();
                    self.iptr += 2;
                }
                4 => {
                    // Output 1
                    let params = self.get_params(1, param_modes);
                    self.iptr += 2;
                    return ProgYield::Output(params[0]);
                }
                5 => {
                    // JNZ 1 to 2
                    let params = self.get_params(2, param_modes);
                    if params[0] != 0 {
                        self.iptr = params[1] as usize;
                    } else {
                        self.iptr += 3;
                    }
                }
                6 => {
                    // JEZ 1 to 2
                    let params = self.get_params(2, param_modes);
                    if params[0] == 0 {
                        self.iptr = params[1] as usize;
                    } else {
                        self.iptr += 3;
                    }
                }
                7 => {
                    // 1 LT 2, store 3
                    let params = self.get_params(2, param_modes);
                    let dest = self.check_dest(3, param_modes);
                    self.program[dest] = if params[0] < params[1] { 1 } else { 0 };
                    self.iptr += 4;
                }
                8 => {
                    // 1 EQ 2, store 3
                    let params = self.get_params(2, param_modes);
                    let dest = self.check_dest(3, param_modes);
                    self.program[dest] = if params[0] == params[1] { 1 } else { 0 };
                    self.iptr += 4;
                }
                9 => {
                    // Relbase adjust
                    let params = self.get_params(1, param_modes);
                    self.relative_base += params[0];
                    self.iptr += 2;
                }
                99 => return ProgYield::Halt,
                other => unreachable!("Opcode {} unknown", other),
            }
        }
    }
}
