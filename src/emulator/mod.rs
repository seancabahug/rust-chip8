pub mod opcodes;

use self::opcodes::translate_instruction;
use crate::config::*;

type Framebuffer = [[bool; CHIP8_WIDTH as usize]; CHIP8_HEIGHT as usize];

pub struct Emulator {
    ram: [u8; RAM_CAPACITY],
    v: [u8; NUM_REGISTERS],
    i: usize,
    delay_timer: u8,
    sound_timer: u8,
    pc: usize, // The program counter should be 16-bit
    sp: usize,
    stack: [usize; STACK_CAPACITY],
    framebuffer: Framebuffer,
    input_buffer: [bool; 16],
}

impl Emulator {
    pub fn init(rom: Vec<u8>) -> Emulator {
        let mut emulator = Emulator {
            ram: [0; RAM_CAPACITY],
            v: [0; NUM_REGISTERS],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200, // Most programs start @ 0x200
            sp: 0,
            stack: [0; STACK_CAPACITY],
            framebuffer: [[false; CHIP8_WIDTH as usize]; CHIP8_HEIGHT as usize],
            input_buffer: [false; 16],
        };

        // Loading font set into memory
        for (i, &byte) in FONT_SET.iter().enumerate() {
            emulator.ram[i] = byte;
        }

        // Loading ROM into memory
        for (i, &byte) in rom.iter().enumerate() {
            emulator.ram[0x200 + i] = byte;
        }

        emulator
    }

    pub fn get_framebuffer(&self) -> Framebuffer {
        self.framebuffer
    }

    pub fn set_key_state(&mut self, key: usize, pressed: bool) {
        self.input_buffer[key] = pressed;
    }

    pub fn decrement_delay_timer(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }

    pub fn next_instruction(&mut self) {
        let instruction = (self.ram[self.pc] as usize) << 8 | self.ram[self.pc + 1] as usize;
        let opcode = translate_instruction(instruction as usize);

        use self::opcodes::Opcode::*;
        match opcode {
            Clear => self.framebuffer = [[false; CHIP8_WIDTH as usize]; CHIP8_HEIGHT as usize],
            Return => {
                self.pc = self.stack[self.sp - 1];
                self.sp -= 1;
            }
            Jump { addr } => self.pc = addr - 2,
            Call { addr } => {
                self.sp += 1;
                self.stack[self.sp - 1] = self.pc;
                self.pc = addr - 2;
            }
            SkipIfEqValue { vx, value } => {
                if self.v[vx] == value {
                    self.pc += 2;
                }
            }
            SkipIfNotEqValue { vx, value } => {
                if self.v[vx] != value {
                    self.pc += 2;
                }
            }
            SkipIfEqRegister { vx, vy } => {
                if self.v[vx] == self.v[vy] {
                    self.pc += 2;
                }
            }
            SetValue { vx, value } => self.v[vx] = value,
            AddValue { vx, value } => self.v[vx] = self.v[vx].wrapping_add(value), // wrapping_add used to allow overflow
            SetFromRegister { vx, vy } => self.v[vx] = self.v[vy],
            BitwiseOr { vx, vy } => self.v[vx] |= self.v[vy],
            BitwiseAnd { vx, vy } => self.v[vx] &= self.v[vy],
            BitwiseXor { vx, vy } => self.v[vx] ^= self.v[vy],
            Add { vx, vy } => {
                let sum = self.v[vx] as u16 + self.v[vy] as u16;
                self.v[0xF] = if sum > 0xFF { 1 } else { 0 };
                self.v[vx] = (sum & 0xFF) as u8;
            }
            Subtract { vx, vy } => {
                self.v[0xF] = if self.v[vx] > self.v[vy] { 1 } else { 0 };
                self.v[vx] = self.v[vx].wrapping_sub(self.v[vy])
            }
            BitwiseShiftRight { vx } => {
                self.v[0xF] = if self.v[vx] & 1 == 1 { 1 } else { 0 };
                self.v[vx] = self.v[vx] >> 1
            }
            SubtractReversed { vx, vy } => {
                self.v[0xF] = if self.v[vy] > self.v[vx] { 1 } else { 0 };
                self.v[vx] = self.v[vy].wrapping_sub(self.v[vx])
            }
            BitwiseShiftLeft { vx } => {
                self.v[0xF] = if (self.v[vx] >> 7) & 1 == 1 { 1 } else { 0 };
                self.v[vx] = self.v[vx] << 1
            }
            SkipIfNotEqRegister { vx, vy } => {
                if self.v[vx] != self.v[vy] {
                    self.pc += 2;
                }
            }
            SetI { value } => self.i = value,
            Draw { vx, vy, n } => {
                let mut unset_occurred = false;
                for byte_index in 0..n {
                    for bit_index in 0..8 {
                        if (self.ram[self.i + byte_index] >> (7 - bit_index)) & 1 == 1 {
                            let x = (self.v[vx] as usize + bit_index) % CHIP8_WIDTH as usize;
                            let y = (self.v[vy] as usize + byte_index) % CHIP8_HEIGHT as usize;
                            self.framebuffer[y][x] = !self.framebuffer[y][x];
                            if !self.framebuffer[y][x] {
                                unset_occurred = true;
                            }
                        }
                    }
                }

                self.v[0xF] = if unset_occurred { 1 } else { 0 }
            }
            AddIFromRegister { vx } => self.i += self.v[vx] as usize,
            StoreBCD { vx } => {
                let digits = self.v[vx]
                    .to_string()
                    .chars()
                    .map(|x| x.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>();

                for x in 0..3 {
                    self.ram[self.i + x] = digits[x];
                }
            }
            StoreRegistersInMemory { vx } => {
                for x in 0..=vx {
                    self.ram[self.i + x] = self.v[x]
                }
            }
            ReadRegistersFromMemory { vx } => {
                for x in 0..=vx {
                    self.v[x] = self.ram[self.i + x]
                }
            }
            GetDelayTimerValue { vx } => self.v[vx] = self.delay_timer,
            SetDelayTimerFromRegister { vx } => self.delay_timer = self.v[vx],
            WaitForKeyPress { vx } => {
                let mut key_pressed = false;

                for i in 0..self.input_buffer.len() {
                    if self.input_buffer[i] {
                        self.v[vx] = i as u8;
                        key_pressed = true;
                    }
                }

                if !key_pressed {
                    self.pc -= 2;
                }
            }
            SetIToDigitSpriteLocation { vx } => self.i = self.v[vx] as usize * 5,
            SkipIfKeyPressed { vx } => {
                if self.input_buffer[self.v[vx] as usize] {
                    self.pc += 2
                }
            }
            SkipIfKeyNotPressed { vx } => {
                if !self.input_buffer[self.v[vx] as usize] {
                    self.pc += 2
                }
            }
            _ => println!("UNIMPLEMENTED: {:?}", opcode),
        }

        self.pc += 2;
    }
}

impl std::fmt::Display for Emulator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Emulator(pc={:#x})", self.pc)
    }
}
