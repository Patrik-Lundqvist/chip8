extern crate wasm_bindgen;
extern crate chip8_emulator;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Chip8 {
    cpu: chip8_emulator::cpu::Cpu,
}

#[wasm_bindgen]
impl Chip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Chip8 {
        let keypad = chip8_emulator::keypad::Keypad::new();
        Chip8 { cpu: chip8_emulator::cpu::Cpu::new(keypad) }
    }

    pub fn get_contents(&mut self) -> Vec<u8> {
        return self.cpu.video_memory
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .iter()
            .flat_map(|x| x.into_iter())
            .map(|x| *x as u8)
            .collect::<Vec<u8>>();
    }

    pub fn load_content(&mut self, data: &[u8]) {
        self.cpu.load_program(data);
    }

    pub fn execute_cycle(&mut self) {
        self.cpu.execute_cycle();
    }

    pub fn execute_next_op_code(&mut self) {
        self.cpu.execute_next_op_code();
    }

    pub fn press_key(&mut self, key: u8) {
        self.cpu.keypad.press_key(key);
    }

    pub fn release_key(&mut self, key: u8) {
        self.cpu.keypad.release_key(key);
    }

    pub fn release_all_keys(&mut self) {
        self.cpu.keypad.release_all_keys();
    }

    pub fn get_v_register(&mut self) -> Vec<u8> {
        return self.cpu.registers.v
            .iter()
            .cloned()
            .collect::<Vec<u8>>();
    }

    pub fn get_i_register(&mut self) -> u16 {
        return self.cpu.registers.i;
    }

    pub fn get_program_counter(&mut self) -> u16 {
        return self.cpu.program_counter;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }
}
