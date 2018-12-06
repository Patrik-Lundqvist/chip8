use keypad::Keypad;
use ::nibbles::*;
use ::sprites::*;
use rand::rngs::{OsRng};
use rand::RngCore;

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const PC_STEP: u16 = 2;
const PROGRAM_START: usize = 0x200;

enum Action {
    Nothing,
    ContinueAfterKeypress(u8),
    Continue,
    SkipNext,
    Jump(u16),
    EnterSubroutine(u16),
    ExitSubroutine,
}

pub struct Registers {
    pub v: [u8;16],
    pub i: u16,
    pub sound_timer: u8,
    pub delay_timer: u8,
}

pub struct Cpu {
    pub memory: [u8;4096],
    pub video_memory: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub registers: Registers,
    pub program_counter: u16,
    pub stack: [u16; 16],
    pub stack_pointer: usize,

    await_key: Option<u8>,

    pub keypad: Keypad,
}

impl Cpu {
    pub fn new(keypad: Keypad) -> Cpu {
        let mut cpu = Cpu {
            memory: [0; 4096],
            video_memory: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            registers: Registers {
                v: [0; 16],
                i: 0,
                sound_timer: 0,
                delay_timer: 0
            },
            stack: [0; 16],
            stack_pointer: 0,
            program_counter: PROGRAM_START as u16,
            await_key: None,
            keypad: keypad,
        };

        FONT_SPRITES.iter()
            .flat_map(|x| x)
            .enumerate()
            .for_each(|(i, x)| cpu.memory[i] = *x);

        return cpu;
    }

    pub fn reset (&mut self) {
        self.memory = [0; 4096];
        self.video_memory = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
        self.registers = Registers {
            v: [0; 16],
            i: 0,
            sound_timer: 0,
            delay_timer: 0
        };
        self.stack = [0; 16];
        self.stack_pointer = 0;
        self.program_counter = PROGRAM_START as u16;
        self.await_key = None;

        FONT_SPRITES.iter()
            .flat_map(|x| x)
            .enumerate()
            .for_each(|(i, x)| self.memory[i] = *x);
    }

    pub fn load_program (&mut self, bytes: &[u8]) {
        for (i, b) in bytes.iter().enumerate() {
            let index = PROGRAM_START + i;
            let last_index = self.memory.len() - 1;

            if index > last_index {
                break;
            }

            self.memory[index] = *b;
        }
    }

    pub fn execute_cycle(&mut self) -> [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT] {
        if self.registers.delay_timer > 0 {
            self.registers.delay_timer -= 1
        }

        for _ in 0..8 {
            self.execute_next_op_code();
        }

        return self.video_memory;
    }

    pub fn execute_next_op_code(&mut self) {
        let next_action = match self.await_key {
            Some(address) => {
                match self.keypad.get_first_pressed_key() {
                    Some(key) => {
                        self.registers.v[address as usize] = key;
                        self.await_key = None;

                        let opcode = self.get_current_opcode();
                        self.run_opcode(opcode)
                    },
                    None => Action::Nothing,
                }
            },
            None => {
                let opcode = self.get_current_opcode();
                self.run_opcode(opcode)
            },
        };

        self.handle_action(next_action);
    }

    fn get_current_opcode (&mut self) -> u16 {
        let part1 = self.memory[self.program_counter as usize];
        let part2 = self.memory[(self.program_counter + 1) as usize];

        let opcode: u16 = ((part1 as u16) << 8) | (part2 as u16);

        return opcode;
    }

    fn handle_action (&mut self, action: Action) {
        match action {
            Action::Nothing => {},
            Action::ContinueAfterKeypress(address) => {
                self.await_key = Some(address);
                self.program_counter += PC_STEP;
            },
            Action::Continue => {
                self.program_counter += PC_STEP;
            },
            Action::SkipNext => {
                self.program_counter += PC_STEP * 2;
            },
            Action::Jump(address) => {
                self.program_counter = address;
            },
            Action::EnterSubroutine(address) => {
                self.stack[self.stack_pointer] = self.program_counter + PC_STEP;
                self.stack_pointer += 1;
                self.program_counter = address;
            },
            Action::ExitSubroutine => {
                self.stack_pointer -= 1;
                self.program_counter = self.stack[self.stack_pointer];
            },
        }
    }

    fn run_opcode (&mut self, opcode: u16) -> Action {
        let nibbles: (u8, u8, u8, u8) = (
            get_nibble_1(opcode),
            get_nibble_2(opcode),
            get_nibble_3(opcode),
            get_nibble_4(opcode)
        );

        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => {
                self.video_memory = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
                Action::Continue
            },
            (0x0, 0x0, 0xE, 0xE) => {
                Action::ExitSubroutine
            },
            (0x1, n1, n2, n3) => {
                let address = concat_nibbles_3(n1, n2, n3);
                Action::Jump(address)
            },
            (0x2, n1, n2, n3) => {
                let address = concat_nibbles_3(n1, n2, n3);
                Action::EnterSubroutine(address)
            },
            (0x3, x, k1, k2) => {
                let v_address = x as usize;
                let value = concat_nibbles_2(k1, k2);

                if self.registers.v[v_address] == value {
                    Action::SkipNext
                } else {
                    Action::Continue
                }
            },
            (0x4, x, k1, k2) => {
                let v_address = x as usize;
                let value = concat_nibbles_2(k1, k2);

                if self.registers.v[v_address] != value {
                    Action::SkipNext
                } else {
                    Action::Continue
                }
            },
            (0x5, x, y, 0x0) => {
                let v_address1 = x as usize;
                let v_address2 = y as usize;
                let v_value1 = self.registers.v[v_address1];
                let v_value2 = self.registers.v[v_address2];

                if v_value1 == v_value2 {
                    Action::SkipNext
                } else {
                    Action::Continue
                }
            },
            (0x6, x, k1, k2) => {
                let v_address = x as usize;
                let value = concat_nibbles_2(k1, k2);

                self.registers.v[v_address] = value;
                Action::Continue
            },
            (0x7, x, k1, k2) => {
                let value = concat_nibbles_2(k1, k2) as u16;
                let v_address = x as usize;
                let v_value = self.registers.v[v_address] as u16;
                let sum: u16 = v_value + value;

                self.registers.v[v_address] = sum as u8;
                Action::Continue
            },
            (0x8, x, y, 0x0) => {
                let v_address1 = x as usize;
                let v_address2 = y as usize;

                self.registers.v[v_address1] = self.registers.v[v_address2];
                Action::Continue
            },
            (0x8, x, y, 0x1) => {
                let v_address1 = x as usize;
                let v_address2 = y as usize;
                let v_value1 = self.registers.v[v_address1];
                let v_value2 = self.registers.v[v_address2];

                self.registers.v[v_address1] = v_value1 | v_value2;
                Action::Continue
            },
            (0x8, x, y, 0x2) => {
                let v_address1 = x as usize;
                let v_address2 = y as usize;
                let v_value1 = self.registers.v[v_address1];
                let v_value2 = self.registers.v[v_address2];

                self.registers.v[v_address1] = v_value1 & v_value2;
                Action::Continue
            },
            (0x8, x, y, 0x3) => {
                let v_address1 = x as usize;
                let v_address2 = y as usize;
                let v_value1 = self.registers.v[v_address1];
                let v_value2 = self.registers.v[v_address2];

                self.registers.v[v_address1] = v_value1 ^ v_value2;
                Action::Continue
            },
            (0x8, x, y, 0x4) => {
                let v_address1 = x as usize;
                let v_address2 = y as usize;
                let v_value1 = self.registers.v[v_address1] as u16;
                let v_value2 = self.registers.v[v_address2] as u16;

                let result = v_value1 + v_value2;
                self.registers.v[v_address1] = result as u8;
                self.registers.v[0xF] = (result > 0xFF) as u8;
                Action::Continue
            },
            (0x8, x, y, 0x5) => {
                let result_address = x as usize;
                let v_address1 = x as usize;
                let v_address2 = y as usize;
                let v_value1 = self.registers.v[v_address1];
                let v_value2 = self.registers.v[v_address2];

                self.registers.v[0xF] = (v_value1 > v_value2) as u8;

                let result = v_value1.checked_sub(v_value2);
                self.registers.v[result_address] = result.unwrap_or(0);
                Action::Continue
            },
            (0x8, x, _, 0x6) => {
                let v_address = x as usize;
                let v_value = self.registers.v[v_address];

                self.registers.v[0xF] = v_value & 0x1;

                self.registers.v[v_address] = self.registers.v[v_address] >> 1;
                Action::Continue
            },
            (0x8, x, y, 0x7) => {
                let result_address = x as usize;
                let v_address1 = x as usize;
                let v_address2 = y as usize;
                let v_value1 = self.registers.v[v_address1];
                let v_value2 = self.registers.v[v_address2];

                self.registers.v[0xF] = (v_value1 < v_value2) as u8;

                let result = v_value2.checked_sub(v_value1);
                self.registers.v[result_address] = result.unwrap_or(0);
                Action::Continue
            },
            (0x8, x, _, 0xE) => {
                let v_address1 = x as usize;
                let v_value1 = self.registers.v[v_address1];

                self.registers.v[0xF] = v_value1 >> 7;

                self.registers.v[v_address1] = v_value1 << 1;
                Action::Continue
            },
            (0x9, x, y, 0x0) => {
                let v_address1 = x as usize;
                let v_address2 = y as usize;
                let v_value1 = self.registers.v[v_address1];
                let v_value2 = self.registers.v[v_address2];

                if v_value1 != v_value2 {
                    Action::SkipNext
                } else {
                    Action::Continue
                }
            },
            (0xA, n1, n2, n3) => {
                let value = concat_nibbles_3(n1, n2, n3);
                self.registers.i = value;
                Action::Continue
            },
            (0xB, n1, n2, n3) => {
                let address = concat_nibbles_3(n1, n2, n3);
                let v_value = self.registers.v[0];
                Action::Jump(address + (v_value as u16))
            },
            (0xC, x, k1, k2) => {
                let v_address1 = x as usize;
                let value = concat_nibbles_2(k1, k2);
                let mut rng = match OsRng::new() {
                    Ok(g) => g,
                    Err(e) => panic!("Error opening OS RNG: {}", e)
                };
                let random_number = rng.next_u32() as u8;
                self.registers.v[v_address1] = random_number & value;
                Action::Continue
            },
            (0xD, x, y, n) => {
                let v_address1 = x as usize;
                let v_address2 = y as usize;
                let sprite_len = n;

                self.registers.v[0xF] = 0;

                for row in 0..sprite_len {
                    let sprite_row = self.memory[(self.registers.i + (row as u16)) as usize];
                    let y_draw_position = (self.registers.v[v_address2] + row) % (DISPLAY_HEIGHT as u8);
                    for column in 0..8 {
                        let x_draw_position = (self.registers.v[v_address1] + column) % (DISPLAY_WIDTH as u8);
                        let pixel = &mut self.video_memory[y_draw_position as usize][x_draw_position as usize];
                        let sprite_pixel = ((sprite_row >> 7 - column) & 1) != 0;
                        let pixel_erased = sprite_pixel & *pixel;
                        *pixel ^= sprite_pixel;
                        self.registers.v[0xF] |= pixel_erased as u8;
                    }
                }

                Action::Continue
            },
            (0xE, x, 9, 0xE) => {
                let v_address = x as usize;
                let key = self.registers.v[v_address];
                if self.keypad.key_is_pressed(key) {
                    Action::SkipNext
                } else {
                    Action::Continue
                }
            },
            (0xE, x, 0xA, 0x1) => {
                let v_address = x as usize;
                let key = self.registers.v[v_address];
                if !self.keypad.key_is_pressed(key) {
                    Action::SkipNext
                } else {
                    Action::Continue
                }
            },
            (0xF, x, 0x0, 0x7) => {
                let v_address = x as usize;
                self.registers.v[v_address] = self.registers.delay_timer;
                Action::Continue
            },
            (0xF, x, 0x0, 0xA) => {
                 Action::ContinueAfterKeypress(x)
            },
            (0xF, x, 0x1, 0x5) => {
                let v_address = x as usize;
                let v_value = self.registers.v[v_address];
                self.registers.delay_timer = v_value;
                Action::Continue
            },
            (0xF, x, 0x1, 0x8) => {
                let v_address = x as usize;
                let v_value = self.registers.v[v_address];
                self.registers.sound_timer = v_value;

                Action::Continue
            },
            (0xF, x, 0x1, 0xE) => {
                let v_address = x as usize;
                let v_value = self.registers.v[v_address];
                self.registers.i += v_value as u16;

                Action::Continue
            },
            (0xF, x, 0x2, 0x9) => {
                let v_address = x as usize;
                let v_value = self.registers.v[v_address];

                self.registers.i = (v_value * 5) as u16;
                Action::Continue
            },
            (0xF, x, 0x3, 0x3) => {
                let v_address = x as usize;
                let v_value = self.registers.v[v_address];

                self.memory[self.registers.i as usize] = v_value / 100;
                self.memory[(self.registers.i + 1) as usize] = (v_value / 10) % 10;
                self.memory[(self.registers.i + 2) as usize] = (v_value % 100) % 10;
                Action::Continue
            },
            (0xF, x, 0x5, 0x5) => {
                let v_address = x as usize;

                for i in 0..(v_address + 1)  {
                    self.memory[(self.registers.i + (i as u16)) as usize] = self.registers.v[i as usize];
                }

                Action::Continue
            },
            (0xF, x, 0x6, 0x5) => {
                let v_address = x as usize;

                for i in 0..(v_address + 1)  {
                    self.registers.v[i as usize] = self.memory[(self.registers.i + (i as u16)) as usize];
                }
                Action::Continue
            },
            _ => Action::Nothing
        }
    }
}