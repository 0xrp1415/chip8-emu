#![allow(non_snake_case)]

use crate::cpu;
use rand;

/// Clear the display
pub fn CLS(cpu: &mut cpu::cpu) {
    cpu.io.clear_display();
}

/// Return from a subroutine
pub fn RET(cpu: &mut cpu::cpu) {
    cpu.pc = cpu.stack[cpu.sp as usize];
    cpu.sp -= 1;
}

/// Jump to a machine code routine at nnn (ignored on modern interpreters)
pub fn SYS(cpu: &mut cpu::cpu, nnn: u16) {
    // Ignored by mordern intterpreters
}

/// Jump to location nnn
pub fn JP(cpu: &mut cpu::cpu, nnn: u16) {
    cpu.pc = nnn;
    cpu.noinc = true;
}

/// Call subroutine at nnn
pub fn CALL(cpu: &mut cpu::cpu, nnn: u16) {
    cpu.sp += 1;
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.pc = nnn;
}

/// Skip next instruction if Vx = kk
pub fn SE_VX_byte(cpu: &mut cpu::cpu, x: u8, kk: u8) {
    if cpu.registers[x as usize] == kk {
        cpu.pc += 2;
    }
}

/// Skip next instruction if Vx != kk
pub fn SNE_VX_byte(cpu: &mut cpu::cpu, x: u8, kk: u8) {
    if cpu.registers[x as usize] != kk {
        cpu.pc += 2;
    }
}

/// Skip next instruction if Vx = Vy
pub fn SE_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    if cpu.registers[x as usize] == cpu.registers[y as usize] {
        cpu.pc += 2;
    }
}

/// Set Vx = kk
pub fn LD_VX_byte(cpu: &mut cpu::cpu, x: u8, kk: u8) {
    cpu.registers[x as usize] = kk;
}

/// Set Vx = Vx + kk
pub fn ADD_VX_byte(cpu: &mut cpu::cpu, x: u8, kk: u8) {
    let (result, _) = cpu.registers[x as usize].overflowing_add(kk);
    cpu.registers[x as usize] = result;
}

/// Set Vx = Vy
pub fn LD_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    cpu.registers[x as usize] = cpu.registers[y as usize];
}

/// Set Vx = Vx OR Vy
pub fn OR_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    cpu.registers[x as usize] |= cpu.registers[y as usize];
}

/// Set Vx = Vx AND Vy
pub fn AND_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    cpu.registers[x as usize] &= cpu.registers[y as usize];
}

/// Set Vx = Vx XOR Vy
pub fn XOR_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    cpu.registers[x as usize] ^= cpu.registers[y as usize];
}

/// Set Vx = Vx + Vy, set VF = carry
pub fn ADD_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    let (result, carry) = cpu.registers[x as usize].overflowing_add(cpu.registers[y as usize]);
    cpu.registers[x as usize] = result;
    cpu.registers[0xF] = if carry { 1 } else { 0 };
}

/// Set Vx = Vx - Vy, set VF = NOT borrow
pub fn SUB_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    let (result, borrow) = cpu.registers[x as usize].overflowing_sub(cpu.registers[y as usize]);
    cpu.registers[x as usize] = result;
    cpu.registers[0xF] = if borrow { 0 } else { 1 };
}

/// Set Vx = Vx SHR 1
pub fn SHR_VX(cpu: &mut cpu::cpu, x: u8) {
    let lsb = cpu.registers[x as usize] & 0x1;
    cpu.registers[x as usize] >>= 1;
    cpu.registers[0xF] = lsb;
}

/// Set Vx = Vy - Vx, set VF = NOT borrow
pub fn SUBN_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    let (result, borrow) = cpu.registers[y as usize].overflowing_sub(cpu.registers[x as usize]);
    cpu.registers[x as usize] = result;
    cpu.registers[0xF] = if borrow { 0 } else { 1 };
}

/// Set Vx = Vx SHL 1
pub fn SHL_VX(cpu: &mut cpu::cpu, x: u8) {
    let msb = (cpu.registers[x as usize] & 0x80) >> 7;
    cpu.registers[x as usize] <<= 1;
    cpu.registers[0xF] = msb;
}

/// Skip next instruction if Vx != Vy
pub fn SNE_VX_VY(cpu: &mut cpu::cpu, x: u16, y: u16) {
    if cpu.registers[x as usize] != cpu.registers[y as usize] {
        cpu.pc += 2;
    }
}

/// Set I = nnn
pub fn LD_I_addr(cpu: &mut cpu::cpu, nnn: u16) {
    cpu.index_reg = nnn;
}

/// Jump to location nnn + V0
pub fn JP_V0_addr(cpu: &mut cpu::cpu, nnn: u16) {
    cpu.pc = nnn + cpu.registers[0] as u16;
}

/// Set Vx = random byte AND kk
pub fn RND_VX_byte(cpu: &mut cpu::cpu, x: u16, kk: u8) {
    let random_byte: u8 = rand::random();
    cpu.registers[x as usize] = random_byte & kk;
}

/// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
pub fn DRW_VX_VY_nibble(cpu: &mut cpu::cpu, x: u16, y: u16, n: u8) {
    let vx = cpu.registers[x as usize] as usize;
    let vy = cpu.registers[y as usize] as usize;
    cpu.registers[0xF] = 0;

    for byte_index in 0..n as usize {
        let sprite_byte = cpu.memory[(cpu.index_reg + byte_index as u16) as usize];
        for bit_index in 0..8 {
            let sprite_pixel = (sprite_byte >> (7 - bit_index)) & 0x1;
            let x_coord = (vx + bit_index) % 64;
            let y_coord = (vy + byte_index) % 32;

            let current_pixel = cpu.io.display[y_coord][x_coord];
            let new_pixel = current_pixel ^ sprite_pixel;

            if current_pixel == 1 && new_pixel == 0 {
                cpu.registers[0xF] = 1;
            }

            cpu.io.set_pixel(x_coord, y_coord, new_pixel);
        }
    }
}

/// Skip next instruction if key with the value of Vx is pressed
pub fn SKP_VX(cpu: &mut cpu::cpu, x: u16) {
    if cpu.io.keys[cpu.registers[x as usize] as usize]  {
        cpu.pc += 2;
    }
}

/// Skip next instruction if key with the value of Vx is not pressed
pub fn SKNP_VX(cpu: &mut cpu::cpu, x: u16) {
    if !cpu.io.keys[cpu.registers[x as usize] as usize]  {
        cpu.pc += 2;
    }
}

/// Set Vx = delay timer value
pub fn LD_VX_DT(cpu: &mut cpu::cpu, x: u16) {
    cpu.registers[x as usize] = cpu.timers[0];
}

/// Wait for a key press, store the value of the key in Vx
pub fn LD_VX_K(cpu: &mut cpu::cpu, x: u16) {
    cpu.noinc = true;
    if cpu.io.keys[9] {
        cpu.registers[x as usize] = 9;
        cpu.noinc = false;
    }
}

/// Set delay timer = Vx
pub fn LD_DT_VX(cpu: &mut cpu::cpu, x: u16) {
    cpu.timers[0] = cpu.registers[x as usize];
}

/// Set sound timer = Vx
pub fn LD_ST_VX(cpu: &mut cpu::cpu, x: u16) {
    cpu.timers[1] = cpu.registers[x as usize];
}

/// Set I = I + Vx
pub fn ADD_I_VX(cpu: &mut cpu::cpu, x: u16) {
    cpu.index_reg += cpu.registers[x as usize] as u16;
}

/// Set I = location of sprite for digit Vx
pub fn LD_F_VX(cpu: &mut cpu::cpu, x: u16) {
    cpu.index_reg = (cpu.registers[x as usize] as u16) * 5;
}

/// Store BCD representation of Vx in memory locations I, I+1, and I+2
pub fn LD_B_VX(cpu: &mut cpu::cpu, x: u16) {
    let value = cpu.registers[x as usize];
    cpu.memory[cpu.index_reg as usize] = value / 100;
    cpu.memory[(cpu.index_reg + 1) as usize] = (value % 100) / 10;
    cpu.memory[(cpu.index_reg + 2) as usize] = value % 10;
}

/// Store registers V0 through Vx in memory starting at location I
pub fn LD_I_VX(cpu: &mut cpu::cpu, x: u16) {
    for i in 0..=x {
        cpu.memory[(cpu.index_reg + i) as usize] = cpu.registers[i as usize];
    }
}

/// Read registers V0 through Vx from memory starting at location I
pub fn LD_VX_I(cpu: &mut cpu::cpu, x: u16) {
    for i in 0..=x {
        cpu.registers[i as usize] = cpu.memory[(cpu.index_reg + i) as usize];
    }
}

