mod emu;
mod util;

fn main() {
    let mut program = util::read_ints("./inp.txt");

    program[1] = 12;
    program[2] = 2;

    let mut emu = emu::Emu::new(program);
    emu.run();

    println!("\npos 0: {}", emu.program[0]);
}
