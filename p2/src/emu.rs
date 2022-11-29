use std::vec::Vec;

pub struct Emu {
    pub program: Vec<i32>,
    pc: usize,
}

impl Emu {
    pub fn new(program: Vec<i32>) -> Emu {
        Emu { program, pc: 0 }
    }

    pub fn run(&mut self) {
        while !self.run_one() {}
    }

    /// Process next instruction
    /// @returns True if program did not end
    fn run_one(&mut self) -> bool {
        let opcode = self.program[self.pc];
        match opcode {
            99 => {
                println!("Opcode 99 - Program End");
                return true;
            }
            1 => {
                println!(
                    "Add: {}[{}] + {}[{}] -> {}[{}]",
                    self.program[self.pc + 1],
                    self.pc + 1,
                    self.program[self.pc + 2],
                    self.pc + 2,
                    self.program[self.pc + 3],
                    self.pc + 3
                );
                let res = self.read(self.pc + 1) + self.read(self.pc + 2);
                self.write(self.pc + 3, res);
                self.pc += 4;
            }
            2 => {
                println!(
                    "Add: {}[{}] * {}[{}] -> {}[{}]",
                    self.program[self.pc + 1],
                    self.pc + 1,
                    self.program[self.pc + 2],
                    self.pc + 2,
                    self.program[self.pc + 3],
                    self.pc + 3
                );
                let res = self.read(self.pc + 1) * self.read(self.pc + 2);
                self.write(self.pc + 3, res);
                self.pc += 4;
            }
            _ => panic!("Invalid opcode {} at pc {}", self.program[self.pc], self.pc),
        }
        println!("Program: {:?}\n", self.program);
        return false;
    }

    fn read(&mut self, offset: usize) -> i32 {
        self.program[self.program[offset] as usize]
    }

    fn write(&mut self, offset: usize, value: i32) {
        let addr = self.program[offset] as usize;
        self.program[addr] = value;
    }
}
