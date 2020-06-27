extern crate gl;
use crate::cpu::Cpu;

pub fn get_opcode_func(opcode: u16) -> fn(&mut Cpu, u16) {
    match (opcode & 0xf000) >> 12 {
        0x0 => op_0(opcode),
        0x1 => default,
        0x2 => op_2nnn,
        0x3 => op_3xnn,
        0x4 => op_4xnn,
        0x5 => op_5xy0,
        0x6 => op_6xnn,
        0x7 => op_7xnn,
        0x8 => op_8(opcode),
        _ => default,
    }
}

fn get_opcode_parts(opcode: u16) -> (u16, u8, u8, u8, u8) {
    let nnn = opcode & 0x0fff;
    let kk = (opcode & 0xff) as u8;
    let n = (opcode & 0xf) as u8;
    let x = ((opcode & 0xf00) >> 8) as u8;
    let y = ((opcode & 0xf0) >> 4) as u8;
    (nnn, kk, n, x, y)
}

fn op_0(opcode: u16) -> fn(&mut Cpu, u16) {
    let (_, kk, _, _, _) = get_opcode_parts(opcode);
    match kk {
        0xe0 => op_00e0,
        0xee => op_00ee,
        _ => default
    }
}

fn op_8(opcode: u16) -> fn(&mut Cpu, u16) {
    let (_, _, n, _, _) = get_opcode_parts(opcode);
    match n {
        0x0 => op_8xy0,
        0x1 => op_8xy1,
        0x2 => op_8xy2,
        0x3 => op_8xy3,
        0x4 => op_8xy4,
        0x5 => op_8xy5,
        0x6 => op_8xy6,
        0x7 => op_8xy7,
        0xe => op_8xye,
        _ => default
    }
}

fn op_9(opcode: u16) -> fn(&mut Cpu, u16) {
    op_3xnn
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

fn op_3xnn(cpu: &mut Cpu, opcode: u16) {
    let (_, kk, _, x, _) = get_opcode_parts(opcode);
    println!("op_3xnn {} {}", x, kk);
    if cpu.registers[x as usize] == kk {
        cpu.pc += 2;
    }
}

fn op_4xnn(cpu: &mut Cpu, opcode: u16) {
    let (_, kk, _, x, _) = get_opcode_parts(opcode);
    println!("op_4xnn {} {}", x, kk);
    if cpu.registers[x as usize] != kk {
        cpu.pc += 2;
    }
}

fn op_5xy0(cpu: &mut Cpu, opcode: u16) {
    let (_, _, _, x, y) = get_opcode_parts(opcode);
    println!("5xy0 {} {}", x, y);
    if cpu.registers[x as usize] == cpu.registers[y as usize] {
        cpu.pc += 2;
    }
}

fn op_6xnn(cpu: &mut Cpu, opcode: u16) {
    let (_, kk, _, x, _) = get_opcode_parts(opcode);
    println!("6xnn {} {}", x, kk);
    cpu.registers[x as usize] = kk;
}

fn op_7xnn(cpu: &mut Cpu, opcode: u16) {
    let (_, kk, _, x, _) = get_opcode_parts(opcode);
    println!("7xnn {} {}", x, kk);
    cpu.registers[x as usize].wrapping_add(kk);
}

fn op_8xy0(cpu: &mut Cpu, opcode: u16) {
    let (_, _, _, x, y) = get_opcode_parts(opcode);
    println!("8xy0 {} {}", x, y);
    cpu.registers[x as usize] = cpu.registers[y as usize];

}

fn op_8xy1(cpu: &mut Cpu, opcode: u16) {
    
}

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
