extern crate chip8_emulator;

use chip8_emulator::cpu::*;
use chip8_emulator::keypad::*;

pub struct CpuFixture {
    pub cpu: Cpu,
}

impl CpuFixture {
    pub fn new() -> CpuFixture {
        let keypad = Keypad::new();
        let cpu = Cpu::new(keypad);
        CpuFixture {cpu}
    }

    pub fn load_op_codes (&mut self, opcodes: &[u16]) {
        let mut program = [0; 3584];
        opcodes.iter()
            .enumerate()
            .for_each(|(i, b)| {
                program[i * 2] = ((*b & 0xFF00) >> 8) as u8;
                program[(i * 2 ) + 1] = (*b & 0x00FF) as u8;
            });

        self.cpu.load_program(&program);
    }
}