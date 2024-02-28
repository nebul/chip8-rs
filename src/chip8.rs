use rand::Rng;

pub trait Instruction {
    fn execute(&self, chip8: &mut Chip8);
    fn display(&self) -> String;
}

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub v: [u8; 16], 
    pub i: u16, // Index register
    pub pc: u16, // Program counter
    pub gfx: [u8; 64 * 32],
    pub delay_timer: u8, 
    pub sound_timer: u8, 
    pub stack: [u16; 16], 
    pub sp: usize,
    pub keypad: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keypad: [0; 16],
        }
    }

    pub fn execute_instruction(&mut self, instruction: &dyn Instruction) {
        instruction.execute(self);
    }

    pub fn emulate_cycle(&mut self) 
    {
        let opcode = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);
        self.pc += 2;

        let instruction: Box<dyn Instruction> = match opcode & 0xF000 {
            0x0000 => match opcode & 0xFFFF {
                0x00E0 => Box::new(Cls),
                0x00EE => Box::new(Ret),
                _ => Box::new(InvalidInstruction),
            },
            0x1000 => Box::new(Jmp { address: opcode & 0x0FFF }),
            0x2000 => Box::new(Call { address: opcode & 0x0FFF }),
            0x3000 => Box::new(SeVxByte {
                x: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x4000 => Box::new(SneVxByte {
                x: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x5000 => Box::new(SeVxVy {
                x: ((opcode & 0x0F00) >> 8) as u8,
                y: ((opcode & 0x00F0) >> 4) as u8,
            }),
            0x6000 => Box::new(LdVxByte {
                x: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x7000 => Box::new(AddVxByte {
                x: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0x8000 => match opcode & 0x000F {
                0x0000 => Box::new(LdVxVy {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                    y: ((opcode & 0x00F0) >> 4) as u8,
                }),
                0x0001 => Box::new(OrVxVy {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                    y: ((opcode & 0x00F0) >> 4) as u8,
                }),
                0x0002 => Box::new(AndVxVy {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                    y: ((opcode & 0x00F0) >> 4) as u8,
                }),
                0x0003 => Box::new(XorVxVy {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                    y: ((opcode & 0x00F0) >> 4) as u8,
                }),
                0x0004 => Box::new(AddVxVy {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                    y: ((opcode & 0x00F0) >> 4) as u8,
                }),
                0x0005 => Box::new(SubVxVy {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                    y: ((opcode & 0x00F0) >> 4) as u8,
                }),
                0x0006 => Box::new(ShrVxVy {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                    y: ((opcode & 0x00F0) >> 4) as u8,
                }),
                0x0007 => Box::new(SubnVxVy {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                    y: ((opcode & 0x00F0) >> 4) as u8,
                }),
                0x000E => Box::new(ShlVxVy {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                    y: ((opcode & 0x00F0) >> 4) as u8,
                }),
                _ => Box::new(InvalidInstruction),
            },
            0x9000 => Box::new(SneVxVy {
                x: ((opcode & 0x0F00) >> 8) as u8,
                y: ((opcode & 0x00F0) >> 4) as u8,
            }),
            0xA000 => Box::new(LdIAddr { address: opcode & 0x0FFF }),
            0xB000 => Box::new(JmpV0Addr { address: opcode & 0x0FFF }),
            0xC000 => Box::new(RndVxByte {
                x: ((opcode & 0x0F00) >> 8) as u8,
                byte: (opcode & 0x00FF) as u8,
            }),
            0xD000 => Box::new(DrwVxVyNibble {
                x: ((opcode & 0x0F00) >> 8) as u8,
                y: ((opcode & 0x00F0) >> 4) as u8,
                n: (opcode & 0x000F) as u8,
            }),
            0xE000 => match opcode & 0x00FF {
                0x009E => Box::new(SkpVx {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x00A1 => Box::new(SknpVx {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                _ => Box::new(InvalidInstruction),
            },
            0xF000 => match opcode & 0x00FF {
                0x0007 => Box::new(LdVxDT {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x000A => Box::new(LdVxK {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0015 => Box::new(LdDTVx {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0018 => Box::new(LdSTVx {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x001E => Box::new(AddIVx {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0029 => Box::new(LdFVx {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0033 => Box::new(LdBVx {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0055 => Box::new(LdIVx {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                0x0065 => Box::new(LdVxI {
                    x: ((opcode & 0x0F00) >> 8) as u8,
                }),
                _ => Box::new(InvalidInstruction),
            },
            _ => Box::new(InvalidInstruction),
        };

        self.execute_instruction(&*instruction);
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[i + 0x200] = byte;
        }
    }

    pub fn get_graphics(&self) -> &[u8; 64 * 32] {
        &self.gfx
    }

    pub fn set_key(&mut self, key: usize, pressed: bool) {
        self.keypad[key as usize] = if pressed { 1 } else { 0 };
    }

    pub fn tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!");
            }
            self.sound_timer -= 1;
        }
    }
}

pub struct Cls;
impl Instruction for Cls {
    fn execute(&self, chip8: &mut Chip8) {
        for i in chip8.gfx.iter_mut() {
            *i = 0;
        }
    }

    fn display(&self) -> String {
        "CLS".to_string()
    }
}

pub struct Sys;
impl Instruction for Sys {
    fn execute(&self, _chip8: &mut Chip8) {
        // Do nothing
    }

    fn display(&self) -> String {
        "SYS".to_string()
    }
}

pub struct Ret;
impl Instruction for Ret {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.sp -= 1;
        chip8.pc = chip8.stack[chip8.sp];
    }

    fn display(&self) -> String {
        "RET".to_string()
    }
}


pub struct Jmp {
    address: u16,
}

impl Instruction for Jmp {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.pc = self.address;
    }

    fn display(&self) -> String {
        format!("JMP to {:#X}", self.address)
    }
}

pub struct Call {
    address: u16,
}

impl Instruction for Call {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.stack[chip8.sp] = chip8.pc;
        chip8.sp += 1;
        chip8.pc = self.address;
    }

    fn display(&self) -> String {
        format!("CALL {:#X}", self.address)
    }
}

pub struct SeVxByte {
    x: u8,
    byte: u8,
}

impl Instruction for SeVxByte {
    fn execute(&self, chip8: &mut Chip8) {
        if chip8.v[self.x as usize] == self.byte {
            chip8.pc += 2;
        }
    }

    fn display(&self) -> String {
        format!("SE V{:X}, {:#X}", self.x, self.byte)
    }
}

pub struct SneVxByte {
    x: u8,
    byte: u8,
}

impl Instruction for SneVxByte {
    fn execute(&self, chip8: &mut Chip8) {
        if chip8.v[self.x as usize] != self.byte {
            chip8.pc += 2;
        }
    }

    fn display(&self) -> String {
        format!("SNE V{:X}, {:#X}", self.x, self.byte)
    }
}

pub struct SeVxVy {
    x: u8,
    y: u8,
}

impl Instruction for SeVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        if chip8.v[self.x as usize] == chip8.v[self.y as usize] {
            chip8.pc += 2;
        }
    }

    fn display(&self) -> String {
        format!("SE V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct LdVxByte {
    x: u8,
    byte: u8,
}

impl Instruction for LdVxByte {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.v[self.x as usize] = self.byte;
    }

    fn display(&self) -> String {
        format!("LD V{:X}, {:#X}", self.x, self.byte)
    }
}

pub struct AddVxByte {
    x: u8,
    byte: u8,
}

impl Instruction for AddVxByte {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.v[self.x as usize] = chip8.v[self.x as usize].wrapping_add(self.byte);
    }

    fn display(&self) -> String {
        format!("ADD V{:X}, {:#X}", self.x, self.byte)
    }
}

pub struct LdVxVy {
    x: u8,
    y: u8,
}

impl Instruction for LdVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.v[self.x as usize] = chip8.v[self.y as usize];
    }

    fn display(&self) -> String {
        format!("LD V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct OrVxVy {
    x: u8,
    y: u8,
}

impl Instruction for OrVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.v[self.x as usize] |= chip8.v[self.y as usize];
    }

    fn display(&self) -> String {
        format!("OR V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct AndVxVy {
    x: u8,
    y: u8,
}

impl Instruction for AndVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.v[self.x as usize] &= chip8.v[self.y as usize];
    }

    fn display(&self) -> String {
        format!("AND V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct XorVxVy {
    x: u8,
    y: u8,
}

impl Instruction for XorVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.v[self.x as usize] ^= chip8.v[self.y as usize];
    }

    fn display(&self) -> String {
        format!("XOR V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct AddVxVy {
    x: u8,
    y: u8,
}

impl Instruction for AddVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        let (result, overflow) = chip8.v[self.x as usize].overflowing_add(chip8.v[self.y as usize]);
        chip8.v[0xF] = if overflow { 1 } else { 0 };
        chip8.v[self.x as usize] = result;
    }

    fn display(&self) -> String {
        format!("ADD V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct SubVxVy {
    x: u8,
    y: u8,
}

impl Instruction for SubVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        let (result, overflow) = chip8.v[self.x as usize].overflowing_sub(chip8.v[self.y as usize]);
        chip8.v[0xF] = if overflow { 0 } else { 1 };
        chip8.v[self.x as usize] = result;
    }

    fn display(&self) -> String {
        format!("SUB V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct ShrVxVy {
    x: u8,
    y: u8,
}

impl Instruction for ShrVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.v[0xF] = chip8.v[self.x as usize] & 0x1;
        chip8.v[self.x as usize] >>= 1;
    }

    fn display(&self) -> String {
        format!("SHR V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct SubnVxVy {
    x: u8,
    y: u8,
}

impl Instruction for SubnVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        let (result, overflow) = chip8.v[self.y as usize].overflowing_sub(chip8.v[self.x as usize]);
        chip8.v[0xF] = if overflow { 0 } else { 1 };
        chip8.v[self.x as usize] = result;
    }

    fn display(&self) -> String {
        format!("SUBN V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct ShlVxVy {
    x: u8,
    y: u8,
}

impl Instruction for ShlVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.v[0xF] = chip8.v[self.x as usize] >> 7;
        chip8.v[self.x as usize] <<= 1;
    }

    fn display(&self) -> String {
        format!("SHL V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct SneVxVy {
    x: u8,
    y: u8,
}

impl Instruction for SneVxVy {
    fn execute(&self, chip8: &mut Chip8) {
        if chip8.v[self.x as usize] != chip8.v[self.y as usize] {
            chip8.pc += 2;
        }
    }

    fn display(&self) -> String {
        format!("SNE V{:X}, V{:X}", self.x, self.y)
    }
}

pub struct LdIAddr {
    address: u16,
}

impl Instruction for LdIAddr {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.i = self.address;
    }

    fn display(&self) -> String {
        format!("LD I, {:#X}", self.address)
    }
}

pub struct JmpV0Addr {
    address: u16,
}

impl Instruction for JmpV0Addr {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.pc = self.address + (chip8.v[0] as u16);
    }

    fn display(&self) -> String {
        format!("JMP V0, {:#X}", self.address)
    }
}

pub struct RndVxByte {
    x: u8,
    byte: u8,
}

impl Instruction for RndVxByte {
    fn execute(&self, chip8: &mut Chip8) {
        let mut rng = rand::thread_rng();
        chip8.v[self.x as usize] = rng.gen::<u8>() & self.byte;
    }

    fn display(&self) -> String {
        format!("RND V{:X}, {:#X}", self.x, self.byte)
    }
}

pub struct DrwVxVyNibble {
    x: u8,
    y: u8,
    n: u8,
}

impl Instruction for DrwVxVyNibble {
    fn execute(&self, chip8: &mut Chip8) {
        let x = chip8.v[self.x as usize] as usize;
        let y = chip8.v[self.y as usize] as usize;
        let height = self.n as usize;

        chip8.v[0xF] = 0;
        for yline in 0..height {
            let pixel = chip8.memory[chip8.i as usize + yline];
            for xline in 0..8 {
                if (pixel & (0x80 >> xline)) != 0 {
                    if chip8.gfx[(x + xline + ((y + yline) * 64)) as usize] == 1 {
                        chip8.v[0xF] = 1;
                    }
                    chip8.gfx[(x + xline + ((y + yline) * 64)) as usize] ^= 1;
                }
            }
        }
    }

    fn display(&self) -> String {
        format!("DRW V{:X}, V{:X}, {:#X}", self.x, self.y, self.n)
    }
}

pub struct SkpVx {
    x: u8,
}

impl Instruction for SkpVx {
    fn execute(&self, chip8: &mut Chip8) {
        if chip8.keypad[chip8.v[self.x as usize] as usize] != 0 {
            chip8.pc += 2;
        }
    }

    fn display(&self) -> String {
        format!("SKP V{:X}", self.x)
    }
}

pub struct SknpVx {
    x: u8,
}

impl Instruction for SknpVx {
    fn execute(&self, chip8: &mut Chip8) {
        if chip8.keypad[chip8.v[self.x as usize] as usize] == 0 {
            chip8.pc += 2;
        }
    }

    fn display(&self) -> String {
        format!("SKNP V{:X}", self.x)
    }
}

pub struct LdVxDT {
    x: u8,
}

impl Instruction for LdVxDT {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.v[self.x as usize] = chip8.delay_timer;
    }

    fn display(&self) -> String {
        format!("LD V{:X}, DT", self.x)
    }
}

pub struct LdVxK {
    x: u8,
}

impl Instruction for LdVxK {
    fn execute(&self, chip8: &mut Chip8) {
        let mut key = 0;
        for i in 0..16 {
            if chip8.keypad[i] != 0 {
                key = i as u8;
                break;
            }
        }

        if key == 0 {
            return;
        }

        chip8.v[self.x as usize] = key;
    }

    fn display(&self) -> String {
        format!("LD V{:X}, K", self.x)
    }
}

pub struct LdDTVx {
    x: u8,
}

impl Instruction for LdDTVx {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.delay_timer = chip8.v[self.x as usize];
    }

    fn display(&self) -> String {
        format!("LD DT, V{:X}", self.x)
    }
}

pub struct LdSTVx {
    x: u8,
}

impl Instruction for LdSTVx {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.sound_timer = chip8.v[self.x as usize];
    }

    fn display(&self) -> String {
        format!("LD ST, V{:X}", self.x)
    }
}

pub struct AddIVx {
    x: u8,
}

impl Instruction for AddIVx {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.i += chip8.v[self.x as usize] as u16;
    }

    fn display(&self) -> String {
        format!("ADD I, V{:X}", self.x)
    }
}

pub struct LdFVx {
    x: u8,
}

impl Instruction for LdFVx {
    fn execute(&self, chip8: &mut Chip8) {
        chip8.i = chip8.v[self.x as usize] as u16 * 5;
    }

    fn display(&self) -> String {
        format!("LD F, V{:X}", self.x)
    }
}

pub struct LdBVx {
    x: u8,
}

impl Instruction for LdBVx {
    fn execute(&self, chip8: &mut Chip8) {
        let vx = chip8.v[self.x as usize];
        chip8.memory[chip8.i as usize] = vx / 100;
        chip8.memory[(chip8.i + 1) as usize] = (vx / 10) % 10;
        chip8.memory[(chip8.i + 2) as usize] = (vx % 100) % 10;
    }

    fn display(&self) -> String {
        format!("LD B, V{:X}", self.x)
    }
}

pub struct LdIVx {
    x: u8,
}

impl Instruction for LdIVx {
    fn execute(&self, chip8: &mut Chip8) {
        for i in 0..=self.x {
            chip8.memory[(chip8.i + i as u16) as usize] = chip8.v[i as usize];
        }
    }

    fn display(&self) -> String {
        format!("LD [I], V{:X}", self.x)
    }
}

pub struct LdVxI {
    x: u8,
}

impl Instruction for LdVxI {
    fn execute(&self, chip8: &mut Chip8) {
        for i in 0..=self.x {
            chip8.v[i as usize] = chip8.memory[(chip8.i + i as u16) as usize];
        }
    }

    fn display(&self) -> String {
        format!("LD V{:X}, [I]", self.x)
    }
}

pub struct InvalidInstruction;

impl Instruction for InvalidInstruction {
    fn execute(&self, _chip8: &mut Chip8) {
        panic!("Invalid instruction")
    }

    fn display(&self) -> String {
        "Invalid instruction".to_string()
    }
}
