use crate::cpu;

/// Clear the display
pub fn CLS(cpu: &mut cpu::cpu) {
    // Implementation needed
}

/// Return from a subroutine
pub fn RET(cpu: &mut cpu::cpu) {
    // Implementation needed
}

/// Jump to a machine code routine at nnn (ignored on modern interpreters)
pub fn SYS(cpu: &mut cpu::cpu, nnn: u16) {
    
}

/// Jump to location nnn
pub fn JP(cpu: &mut cpu::cpu, nnn: u16) {
    // Implementation needed
}

/// Call subroutine at nnn
pub fn CALL(cpu: &mut cpu::cpu, nnn: u16) {
    // Implementation needed
}

/// Skip next instruction if Vx = kk
pub fn SE_VX_byte(cpu: &mut cpu::cpu, x: u8, kk: u8) {
    // Implementation needed
}

/// Skip next instruction if Vx != kk
pub fn SNE_VX_byte(cpu: &mut cpu::cpu, x: u8, kk: u8) {
    // Implementation needed
}

/// Skip next instruction if Vx = Vy
pub fn SE_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    // Implementation needed
}

/// Set Vx = kk
pub fn LD_VX_byte(cpu: &mut cpu::cpu, x: u8, kk: u8) {
    // Implementation needed
}

/// Set Vx = Vx + kk
pub fn ADD_VX_byte(cpu: &mut cpu::cpu, x: u8, kk: u8) {
    // Implementation needed
}

/// Set Vx = Vy
pub fn LD_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    // Implementation needed
}

/// Set Vx = Vx OR Vy
pub fn OR_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    // Implementation needed
}

/// Set Vx = Vx AND Vy
pub fn AND_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    // Implementation needed
}

/// Set Vx = Vx XOR Vy
pub fn XOR_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    // Implementation needed
}

/// Set Vx = Vx + Vy, set VF = carry
pub fn ADD_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    // Implementation needed
}

/// Set Vx = Vx - Vy, set VF = NOT borrow
pub fn SUB_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    // Implementation needed
}

/// Set Vx = Vx SHR 1
pub fn SHR_VX(cpu: &mut cpu::cpu, x: u8) {
    // Implementation needed
}

/// Set Vx = Vy - Vx, set VF = NOT borrow
pub fn SUBN_VX_VY(cpu: &mut cpu::cpu, x: u8, y: u8) {
    // Implementation needed
}

/// Set Vx = Vx SHL 1
pub fn SHL_VX(cpu: &mut cpu::cpu, x: u8) {
    // Implementation needed
}

/// Skip next instruction if Vx != Vy
pub fn SNE_VX_VY(cpu: &mut cpu::cpu, x: u16, y: u16) {
    // Implementation needed
}

/// Set I = nnn
pub fn LD_I_addr(cpu: &mut cpu::cpu, nnn: u16) {
    // Implementation needed
}

/// Jump to location nnn + V0
pub fn JP_V0_addr(cpu: &mut cpu::cpu, nnn: u16) {
    // Implementation needed
}

/// Set Vx = random byte AND kk
pub fn RND_VX_byte(cpu: &mut cpu::cpu, x: u16, kk: u8) {
    // Implementation needed
}

/// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
pub fn DRW_VX_VY_nibble(cpu: &mut cpu::cpu, x: u16, y: u16, n: u8) {
    // Implementation needed
}

/// Skip next instruction if key with the value of Vx is pressed
pub fn SKP_VX(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

/// Skip next instruction if key with the value of Vx is not pressed
pub fn SKNP_VX(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

/// Set Vx = delay timer value
pub fn LD_VX_DT(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

/// Wait for a key press, store the value of the key in Vx
pub fn LD_VX_K(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

/// Set delay timer = Vx
pub fn LD_DT_VX(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

/// Set sound timer = Vx
pub fn LD_ST_VX(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

/// Set I = I + Vx
pub fn ADD_I_VX(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

/// Set I = location of sprite for digit Vx
pub fn LD_F_VX(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

/// Store BCD representation of Vx in memory locations I, I+1, and I+2
pub fn LD_B_VX(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

/// Store registers V0 through Vx in memory starting at location I
pub fn LD_I_VX(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

/// Read registers V0 through Vx from memory starting at location I
pub fn LD_VX_I(cpu: &mut cpu::cpu, x: u16) {
    // Implementation needed
}

