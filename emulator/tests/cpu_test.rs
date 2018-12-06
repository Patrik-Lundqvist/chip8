extern crate chip8_emulator;

mod cpu_fixture;
use chip8_emulator::cpu::*;
use cpu_fixture::CpuFixture;

#[test]
fn test_opcode_00e0() {
    let mut fixture = CpuFixture::new();

    fixture.cpu.video_memory[5][5] = true;
    fixture.cpu.video_memory[10][2] = true;

    fixture.load_op_codes(&[0x00E0]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.video_memory[5][5], false);
    assert_eq!(fixture.cpu.video_memory[10][2], false);
}

#[test]
fn test_opcode_00ee() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.stack_pointer = 1;
    fixture.cpu.stack[0] = 0x123;

    fixture.load_op_codes(&[0x00EE]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.stack_pointer, 0);
    assert_eq!(fixture.cpu.program_counter, 0x123);
}

#[test]
fn test_opcode_1nnn() {
    let mut fixture = CpuFixture::new();
    fixture.load_op_codes(&[0x1123]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.program_counter, 0x123);
}

#[test]
fn test_opcode_2nnn() {
    let mut fixture = CpuFixture::new();
    let current_sp = fixture.cpu.stack_pointer;
    let current_pc = fixture.cpu.program_counter;

    fixture.load_op_codes(&[0x2100]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.stack_pointer, current_sp + 1);
    assert_eq!(fixture.cpu.stack[current_sp], current_pc + PC_STEP);
    assert_eq!(fixture.cpu.program_counter, 0x100);
}

#[test]
fn test_opcode_3xkk() {
    let mut fixture = CpuFixture::new();
    let initial_pc = fixture.cpu.program_counter;
    fixture.cpu.registers.v[0xA] = 0x44;

    fixture.load_op_codes(&[0x3A22, 0x3A44]);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + PC_STEP);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + 6);
}

#[test]
fn test_opcode_4xkk() {
    let mut fixture = CpuFixture::new();
    let initial_pc = fixture.cpu.program_counter;
    fixture.cpu.registers.v[0xA] = 0x44;

    fixture.load_op_codes(&[0x4A44, 0x4A22]);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + PC_STEP);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + 6);
}

#[test]
fn test_opcode_5xy0() {
    let mut fixture = CpuFixture::new();
    let initial_pc = fixture.cpu.program_counter;
    fixture.cpu.registers.v[0xA] = 0x44;
    fixture.cpu.registers.v[0xB] = 0x22;
    fixture.cpu.registers.v[0xC] = 0x44;

    fixture.load_op_codes(&[0x5AB0, 0x5AC0]);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + PC_STEP);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + PC_STEP * 3);
}

#[test]
fn test_opcode_6xkk() {
    let mut fixture = CpuFixture::new();

    fixture.load_op_codes(&[0x6A22]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xA], 0x22);
}

#[test]
fn test_opcode_7xkk() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0x44;

    fixture.load_op_codes(&[0x7A44]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xA], 0x88);
}

#[test]
fn test_opcode_8xy0() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0x44;
    fixture.cpu.registers.v[0xB] = 0x22;

    fixture.load_op_codes(&[0x8AB0]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xA], 0x22);
}

#[test]
fn test_opcode_8xy1() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0x44;
    fixture.cpu.registers.v[0xB] = 0x22;

    fixture.load_op_codes(&[0x8AB1]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xA], 0x44 | 0x22);
}

#[test]
fn test_opcode_8xy2() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0x44;
    fixture.cpu.registers.v[0xB] = 0x22;

    fixture.load_op_codes(&[0x8AB2]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xA], 0x44 & 0x22);
}

#[test]
fn test_opcode_8xy3() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0x44;
    fixture.cpu.registers.v[0xB] = 0x22;

    fixture.load_op_codes(&[0x8AB3]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xA], 0x44 ^ 0x22);
}

#[test]
fn test_opcode_8xy4() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0xFF;
    fixture.cpu.registers.v[0xB] = 0x64;
    fixture.cpu.registers.v[0xC] = 0x10;
    fixture.cpu.registers.v[0xD] = 0x20;
    fixture.load_op_codes(&[0x8AB4, 0x8CD4]);

    let ab_addition_result: u16 = fixture.cpu.registers.v[0xA] as u16 + fixture.cpu.registers.v[0xB] as u16;
    let ab_addition_result_lowest_8bits: u16 = ab_addition_result & 0xFF;

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.registers.v[0xA] as u16, ab_addition_result_lowest_8bits);
    assert_eq!(fixture.cpu.registers.v[0xF], 1);


    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.registers.v[0xC], 0x10 + 0x20);
    assert_eq!(fixture.cpu.registers.v[0xF], 0);
}

#[test]
fn test_opcode_8xy5() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0xFF;
    fixture.cpu.registers.v[0xB] = 0x64;
    fixture.load_op_codes(&[0x8AB5, 0x8BA5]);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.registers.v[0xF], 1);
    assert_eq!(fixture.cpu.registers.v[0xA], 155);

    fixture.cpu.registers.v[0xA] = 0xFF;
    fixture.cpu.registers.v[0xB] = 0x64;

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.registers.v[0xF], 0);
    assert_eq!(fixture.cpu.registers.v[0xB], 0);
}

#[test]
fn test_opcode_8xy6() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0x65;
    fixture.load_op_codes(&[0x8AB6, 0x8AB6]);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.registers.v[0xF], 1);
    assert_eq!(fixture.cpu.registers.v[0xA], 0x65 >> 1);

    fixture.cpu.registers.v[0xA] = 0x64;

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.registers.v[0xF], 0);
    assert_eq!(fixture.cpu.registers.v[0xA], 0x64 >> 1);
}

#[test]
fn test_opcode_8xy7() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0xFF;
    fixture.cpu.registers.v[0xB] = 0x64;
    fixture.load_op_codes(&[0x8BA7, 0x8AB7]);

    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xF], 1);
    assert_eq!(fixture.cpu.registers.v[0xB], 155);

    fixture.cpu.registers.v[0xA] = 0xFF;
    fixture.cpu.registers.v[0xB] = 0x64;
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xF], 0);
    assert_eq!(fixture.cpu.registers.v[0xA], 0);
}

#[test]
fn test_opcode_8xye() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0xFF;

    fixture.load_op_codes(&[0x8ABE, 0x8ABE]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xF], 1);
    assert_eq!(fixture.cpu.registers.v[0xA], 0xFF << 1);
    fixture.cpu.registers.v[0xA] = 0x64;

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.registers.v[0xF], 0);
    assert_eq!(fixture.cpu.registers.v[0xA], 0x64 << 1);
}

#[test]
fn test_opcode_9xy0() {
    let mut fixture = CpuFixture::new();
    let initial_pc = fixture.cpu.program_counter;
    fixture.load_op_codes(&[0x9AC0, 0x9AB0]);
    fixture.cpu.registers.v[0xA] = 0x44;
    fixture.cpu.registers.v[0xB] = 0x22;
    fixture.cpu.registers.v[0xC] = 0x44;

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + PC_STEP);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter,  initial_pc + (PC_STEP * 3));
}

#[test]
fn test_opcode_annn() {
    let mut fixture = CpuFixture::new();

    fixture.load_op_codes(&[0xA123]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.i, 0x123);
}

#[test]
fn test_opcode_bnnn() {
    let mut fixture = CpuFixture::new();

    fixture.cpu.registers.v[0] = 5;
    fixture.load_op_codes(&[0xB123]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.program_counter, 0x123 + (fixture.cpu.registers.v[0] as u16));
}

#[test]
fn test_opcode_cxkk() {
    let mut fixture = CpuFixture::new();

    fixture.load_op_codes(&[0xC000]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0], 0);

    fixture.load_op_codes(&[0xC0AA]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0], fixture.cpu.registers.v[0] & 0xAA);
}

#[test]
fn test_opcode_dxyn_x() {
    let mut fixture = CpuFixture::new();

    fixture.cpu.registers.i = 0x600;
    fixture.cpu.registers.v[1] = 5;
    fixture.cpu.registers.v[2] = 10;
    fixture.load_op_codes(&[0xD121, 0xD121, 0xD121]);


    fixture.cpu.memory[0x600] = 0b10101010;
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xF], 0);
    assert_eq!(fixture.cpu.video_memory[10][5], true);
    assert_eq!(fixture.cpu.video_memory[10][6], false);
    assert_eq!(fixture.cpu.video_memory[10][7], true);

    fixture.cpu.memory[0x600] = 0b01111111;
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0xF], 1);
    assert_eq!(fixture.cpu.video_memory[10][5], true);
    assert_eq!(fixture.cpu.video_memory[10][6], true);
    assert_eq!(fixture.cpu.video_memory[10][7], false);

    fixture.cpu.registers.v[1] = 62;
    fixture.cpu.registers.v[2] = 20;
    fixture.cpu.memory[0x600] = 0b10101010;
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.video_memory[20][62], true);
    assert_eq!(fixture.cpu.video_memory[20][63], false);
    assert_eq!(fixture.cpu.video_memory[20][0], true);
    assert_eq!(fixture.cpu.video_memory[20][1], false);
    assert_eq!(fixture.cpu.video_memory[20][2], true);
    assert_eq!(fixture.cpu.video_memory[20][3], false);
    assert_eq!(fixture.cpu.video_memory[20][4], true);
    assert_eq!(fixture.cpu.video_memory[20][5], false);
}

#[test]
fn test_opcode_dxyn_y() {
    let mut fixture = CpuFixture::new();

    fixture.cpu.registers.i = 0x600;
    fixture.cpu.registers.v[1] = 5;
    fixture.cpu.registers.v[2] = 10;
    fixture.load_op_codes(&[0xD124]);

    fixture.cpu.memory[0x600] = 0b10101010;
    fixture.cpu.memory[0x601] = 0b00101010;
    fixture.cpu.memory[0x602] = 0b10101010;
    fixture.cpu.memory[0x603] = 0b10101010;

    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.video_memory[10][5], true);
    assert_eq!(fixture.cpu.video_memory[11][5], false);
    assert_eq!(fixture.cpu.video_memory[12][5], true);
    assert_eq!(fixture.cpu.video_memory[13][5], true);
}

#[test]
fn test_opcode_ex9e() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0x5;
    let initial_pc = fixture.cpu.program_counter;
    fixture.load_op_codes(&[0xEA9E, 0xEA9E]);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + PC_STEP);

    fixture.cpu.keypad.press_key(0x5);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + (PC_STEP * 3));
}

#[test]
fn test_opcode_exa1() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0xA] = 0x5;
    let initial_pc = fixture.cpu.program_counter;
    fixture.load_op_codes(&[0xEAA1, 0xEAA1]);

    fixture.cpu.keypad.press_key(0x5);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.program_counter, initial_pc + PC_STEP);
    fixture.cpu.keypad.release_key(0x5);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + (PC_STEP * 3));
}

#[test]
fn test_opcode_fx07() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.delay_timer = 0xFF;

    fixture.load_op_codes(&[0xF107]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[1], fixture.cpu.registers.delay_timer);
}

#[test]
fn test_opcode_fx0a() {
    let mut fixture = CpuFixture::new();
    let initial_pc = fixture.cpu.program_counter;
    fixture.load_op_codes(&[0xF50A, 0x00E0, 0x00E0]);

    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + PC_STEP);

    fixture.cpu.keypad.press_key(0xA);
    fixture.cpu.execute_next_op_code();
    assert_eq!(fixture.cpu.program_counter, initial_pc + (PC_STEP * 2));


    assert_eq!(fixture.cpu.registers.v[0x5], 0xA);
}

#[test]
fn test_opcode_fx15() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[1] = 0xFF;

    fixture.load_op_codes(&[0xF115]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.delay_timer, 0xFF);
}

#[test]
fn test_opcode_fx18() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[1] = 0xFF;

    fixture.load_op_codes(&[0xF118]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.sound_timer, 0xFF);
}

#[test]
fn test_opcode_fx1e() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.i = 0x15;
    fixture.cpu.registers.v[1] = 0xAA;

    fixture.load_op_codes(&[0xF11E]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.i, 0x15 + 0xAA);
}

#[test]
fn test_opcode_fx33() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.i = 0x602;
    fixture.cpu.registers.v[5] = 239;

    fixture.load_op_codes(&[0xF533]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.memory[0x602], 2);
    assert_eq!(fixture.cpu.memory[0x603], 3);
    assert_eq!(fixture.cpu.memory[0x604], 9);
}

#[test]
fn test_opcode_fx55() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.i = 0x605;
    fixture.cpu.registers.v[0] = 0xA1;
    fixture.cpu.registers.v[1] = 0xA2;
    fixture.cpu.registers.v[2] = 0xA3;

    fixture.load_op_codes(&[0xFF55]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.memory[0x605], 0xA1);
    assert_eq!(fixture.cpu.memory[0x606], 0xA2);
    assert_eq!(fixture.cpu.memory[0x607], 0xA3);
    assert_eq!(fixture.cpu.memory[0x608], 0);
}

#[test]
fn test_opcode_fx65() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.i = 0x605;
    fixture.load_op_codes(&[0xFF65]);
    fixture.cpu.memory[0x605] = 0xA1;
    fixture.cpu.memory[0x606] = 0xA2;
    fixture.cpu.memory[0x607] = 0xA3;

    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.v[0], 0xA1);
    assert_eq!(fixture.cpu.registers.v[1], 0xA2);
    assert_eq!(fixture.cpu.registers.v[2], 0xA3);
    assert_eq!(fixture.cpu.registers.v[3], 0);
}

#[test]
fn test_opcode_fx29() {
    let mut fixture = CpuFixture::new();
    fixture.cpu.registers.v[0x1] = 0xA;

    fixture.load_op_codes(&[0xF129]);
    fixture.cpu.execute_next_op_code();

    assert_eq!(fixture.cpu.registers.i, 50);
}

#[test]
fn test_load_program_with_overflow() {
    let mut fixture = CpuFixture::new();
    let mut program: [u8; 5000] = [0; 5000];

    for (i, elem) in program.iter_mut().enumerate() {
        *elem = (1 + i % 8) as u8
    }

    fixture.cpu.load_program(&program);
    assert_eq!(*fixture.cpu.memory.last().unwrap(), 8);
}