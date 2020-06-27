extern crate gl;
use crate::cpu::Cpu;

pub fn get_opcode_func(opcode: u16) -> fn(&mut Cpu, u16) {
    match opcode {
        0x00E0 => op_00e0,
        0x00EE => op_00ee,
        0x1000..=0x1fff => op_1nnn,
        0x2000..=0x2fff => op_2nnn,
        _ => default,
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_0nnn(cpu: &mut Cpu, opcode: u16) {}

fn op_00e0(cpu: &mut Cpu, opcode: u16) {
   println!("Clear the display"); 
}

fn op_00ee(cpu: &mut Cpu, _: u16) {
    if cpu.sp > 0 {
        cpu.pc = cpu.stack[cpu.sp as usize];
        cpu.sp -= 1;
        println!("{} Return from subroutine!", cpu.pc);
    }
}

fn op_1nnn(cpu: &mut Cpu, opcode: u16) {
    set_pc_to_nnn(cpu, opcode);
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_2nnn(cpu: &mut Cpu, opcode: u16) {
    cpu.sp += 1;
    cpu.stack[cpu.sp as usize] = cpu.pc;
    set_pc_to_nnn(cpu, opcode);
    println!("sp in now {}", cpu.sp);
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_3xnn(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_4xnn(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_5xy0(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_6xnn(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_7xnn(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_8xy0(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_8xy1(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_8xy2(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_8xy3(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_8xy4(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_8xy5(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_8xy6(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_8xy7(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_8xye(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_9xy0(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_annn(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_bnnn(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_cxnn(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_dxyn(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_ex9e(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_exa1(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx07(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx0a(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx15(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx18(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx1e(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx29(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx33(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx55(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx65(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn default(_: &mut Cpu, _: u16) {}

fn set_pc_to_nnn(cpu: &mut Cpu, opcode: u16) {
    cpu.pc = opcode << 4 >> 4;
    println!("{:x} Set the pc to: {}", opcode, opcode << 4 >> 4);
}
