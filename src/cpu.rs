use crate::{instruction, io::IOData, sprites::SPRITES};

pub struct cpu {
    pub registers: [u8; 16],
    pub index_reg: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub timers: [u8; 2],
    pub memory: [u8; 4096],
    pub io: IOData,
    pub nop: bool,
}

impl cpu {
    pub fn new() -> Self {
        cpu {
            registers: [0; 16],
            index_reg: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            timers: [0; 2],
            memory: [0; 4096],
            io: IOData::new(),
            nop: false,
        }
    }

    pub fn decrement_timers(&mut self) {
        for timer in self.timers.iter_mut() {
            if *timer > 0 {
                *timer -= 1;
            }
        }
    }

    pub fn set_timer(&mut self, timer_index: usize, value: u8) {
        if timer_index < self.timers.len() {
            self.timers[timer_index] = value;
        }
    }

    /// # **Load Program**
    /// Loads a program into the CPU's memory starting at address 0x200.
    /// # Arguments
    /// * `program` - A byte slice representing the program to be loaded.
    pub fn load_program(&mut self, program: &[u8]) {
        let start_address = 0x200;
        let end_address = start_address + program.len();
        self.memory[start_address..end_address].copy_from_slice(program);
    }

    /// # **Get Program Counter**
    /// Returns the current value of the program counter (pc).
    fn get_program_counter(&self) -> u16 {
        let pc: u16 = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[(self.pc + 1) as usize] as u16);
        pc
    }

    /// # **Decode Instruction**
    /// Decodes the current instruction pointed to by the program counter (pc).
    pub fn decode(&mut self) {
        let opcode = self.get_program_counter();
        let grp = opcode >> 12;

        match grp {
            0x0 => match opcode {
                0x00E0 => instruction::CLS(self),
                0x00EE => instruction::RET(self),
                _ => {
                    let nnn = opcode & 0x0FFF;
                    instruction::SYS(self, nnn)
                }
            },
            0x1 => {
                let nnn = opcode & 0x0FFF;
                instruction::JP(self, nnn)
            }
            0x2 => {
                let nnn = opcode & 0x0FFF;
                instruction::CALL(self, nnn);
            }
            0x3 => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let kk = (opcode & 0x00FF) as u8;
                instruction::SE_VX_byte(self, x, kk);
            }
            0x4 => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let kk = (opcode & 0x00FF) as u8;
                instruction::SNE_VX_byte(self, x, kk);
            }
            0x5 => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let y = ((opcode & 0x00F0) >> 4) as u8;
                instruction::SE_VX_VY(self, x, y);
            }
            0x6 => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let kk = (opcode & 0x00FF) as u8;
                instruction::LD_VX_byte(self, x, kk);
            }
            0x7 => {
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let kk = (opcode & 0x00FF) as u8;
                instruction::ADD_VX_byte(self, x, kk);
            }
            0x8 => {
                // Gets last 4 bits
                let pattern = opcode & 0x000F;
                match pattern {
                    0x0 => {
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        instruction::LD_VX_VY(self, x, y);
                    }
                    0x1 => {
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        instruction::OR_VX_VY(self, x, y);
                    }
                    0x2 => {
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        instruction::AND_VX_VY(self, x, y);
                    }
                    0x3 => {
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        instruction::XOR_VX_VY(self, x, y);
                    }
                    0x4 => {
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        instruction::ADD_VX_VY(self, x, y);
                    }
                    0x5 => {
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        instruction::SUB_VX_VY(self, x, y);
                    }
                    0x6 => {
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        instruction::SHR_VX(self, x);
                    }
                    0x7 => {
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        let y = ((opcode & 0x00F0) >> 4) as u8;
                        instruction::SUBN_VX_VY(self, x, y);
                    }
                    0xE => {
                        let x = ((opcode & 0x0F00) >> 8) as u8;
                        instruction::SHL_VX(self, x);
                    }
                    _ => {
                        panic!("Unknown Pattern: {:04X}", opcode)
                    }
                }
            }
            0x9 => {
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                instruction::SNE_VX_VY(self, x, y);
            }
            0xA => {
                let nnn = opcode & 0x0FFF;
                instruction::LD_I_addr(self, nnn);
            }
            0xB => {
                let nnn = opcode & 0x0FFF;
                instruction::JP_V0_addr(self, nnn);
            }
            0xC => {
                let x = (opcode & 0x0F00) >> 8;
                let kk = (opcode & 0x00FF) as u8;
                instruction::RND_VX_byte(self, x, kk);
            }
            0xD => {
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;
                let n = (opcode & 0x000F) as u8;
                instruction::DRW_VX_VY_nibble(self, x, y, n);
            }
            0xE => {
                let x = (opcode & 0x0F00) >> 8;
                let pattern = opcode & 0x00FF;
                match pattern {
                    0x9E => {
                        instruction::SKP_VX(self, x);
                    }
                    0xA1 => {
                        instruction::SKNP_VX(self, x);
                    }
                    _ => {
                        panic!("Unknown Pattern: {:04X}", opcode);
                    }
                }
            }
            0xF => {
                let x = (opcode & 0x0F00) >> 8;
                let pattern = opcode & 0x00FF;
                match pattern {
                    0x07 => {
                        instruction::LD_VX_DT(self, x);
                    }
                    0x0A => {
                        instruction::LD_VX_K(self, x);
                    }
                    0x15 => {
                        instruction::LD_DT_VX(self, x);
                    }
                    0x18 => {
                        instruction::LD_ST_VX(self, x);
                    }
                    0x1E => {
                        instruction::ADD_I_VX(self, x);
                    }
                    0x29 => {
                        instruction::LD_F_VX(self, x);
                    }
                    0x33 => {
                        instruction::LD_B_VX(self, x);
                    }
                    0x55 => {
                        instruction::LD_I_VX(self, x);
                    }
                    0x65 => {
                        instruction::LD_VX_I(self, x);
                    }
                    _ => {
                        panic!("Unknown Pattern: {:04X}", opcode);
                    }
                }
            }
            _ => {
                panic!("Unknown opcode: {:04X}", opcode);
            }
        }

        if !self.nop  {
            self.pc += 2;
        }
    }

    pub fn load_sprites(&mut self) {
        let start_address = 0x50;
        let end_address = start_address + (SPRITES.len() * 5);
        self.memory[start_address..end_address].copy_from_slice(&SPRITES.as_flattened());
    }
}
