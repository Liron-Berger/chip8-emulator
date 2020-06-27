extern crate gl;

use rand::Rng;

use crate::cpu::Cpu;
use crate::opcode::Opcode;
use crate::display::Display;

pub fn get_opcode_func(opcode: &Opcode) -> fn(&mut Cpu, Opcode) {
    match (opcode.opcode & 0xf000) >> 12 {
        0x0 => op_0(opcode),
        0x1 => default,
        0x2 => op_2nnn,
        0x3 => op_3xnn,
        0x4 => op_4xnn,
        0x5 => op_5xy0,
        0x6 => op_6xnn,
        0x7 => op_7xnn,
        0x8 => op_8(opcode),
        0x9 => op_9xy0,
        0xa => op_annn,
        0xb => op_bnnn,
        0xc => op_cxnn,
        0xd => op_dxyn,
        0xe => op_e(opcode),
        _ => default,
    }
}

fn op_0(opcode: &Opcode) -> fn(&mut Cpu, Opcode) {
    match opcode.kk {
        0xe0 => op_00e0,
        0xee => op_00ee,
        _ => default
    }
}

fn op_8(opcode: &Opcode) -> fn(&mut Cpu, Opcode) {
    match opcode.n {
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

fn op_e(opcode: &Opcode) -> fn(&mut Cpu, Opcode) {
    match opcode.opcode & 0xff {
        0x9e => op_ex9e,
        0xa1 => op_exa1,
        _ => default,
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_0nnn(cpu: &mut Cpu, opcode: Opcode) {}

#[allow(unused_variables)]
fn op_00e0(cpu: &mut Cpu, opcode: Opcode) {
   println!("Clear the display"); 
}

fn op_00ee(cpu: &mut Cpu, _: Opcode) {
    if cpu.sp > 0 {
        cpu.pc = cpu.stack[cpu.sp as usize];
        cpu.sp -= 1;
    }
}

fn op_1nnn(cpu: &mut Cpu, opcode: Opcode) {
    cpu.pc = opcode.nnn;
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_2nnn(cpu: &mut Cpu, opcode: Opcode) {
    cpu.sp += 1;
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.pc = opcode.nnn;
}

fn op_3xnn(cpu: &mut Cpu, opcode: Opcode) {
    if cpu.registers[opcode.x as usize] == opcode.kk {
        cpu.advance_pc();
    }
}

fn op_4xnn(cpu: &mut Cpu, opcode: Opcode) {
    if cpu.registers[opcode.x as usize] != opcode.kk {
        cpu.advance_pc();
    }
}

fn op_5xy0(cpu: &mut Cpu, opcode: Opcode) {
    if cpu.registers[opcode.x as usize] == cpu.registers[opcode.y as usize] {
        cpu.advance_pc();
    }
}

fn op_6xnn(cpu: &mut Cpu, opcode: Opcode) {
    cpu.registers[opcode.x as usize] = opcode.kk;
}

fn op_7xnn(cpu: &mut Cpu, opcode: Opcode) {
    cpu.set_v(opcode.x, cpu.get_v(opcode.x).wrapping_add(opcode.kk));
}

fn op_8xy0(cpu: &mut Cpu, opcode: Opcode) {
    cpu.set_v(opcode.x, cpu.get_v(opcode.y));
}

fn op_8xy1(cpu: &mut Cpu, opcode: Opcode) {
    cpu.set_v(opcode.x, cpu.get_v(opcode.x) | cpu.get_v(opcode.y));
}

fn op_8xy2(cpu: &mut Cpu, opcode: Opcode) {
    cpu.set_v(opcode.x, cpu.get_v(opcode.x) & cpu.get_v(opcode.y));
}

fn op_8xy3(cpu: &mut Cpu, opcode: Opcode) {
    cpu.set_v(opcode.x, cpu.get_v(opcode.x) ^ cpu.get_v(opcode.y));
}

fn op_8xy4(cpu: &mut Cpu, opcode: Opcode) {
    let (vx, vf) = cpu.get_v(opcode.x).overflowing_add(cpu.get_v(opcode.y));
    cpu.set_v(opcode.x, vx);
    cpu.set_v(Cpu::VF, vf as u8);
}

fn op_8xy5(cpu: &mut Cpu, opcode: Opcode) {
    let (vx, vf) = cpu.get_v(opcode.x).overflowing_sub(cpu.get_v(opcode.y));
    cpu.set_v(opcode.x, vx);
    cpu.set_v(Cpu::VF, vf as u8);
}

fn op_8xy6(cpu: &mut Cpu, opcode: Opcode) {
    let vx = cpu.get_v(opcode.x);
    cpu.set_v(opcode.x, vx >> 1);
    cpu.set_v(Cpu::VF, Cpu::get_u8_lsb(vx));
}

fn op_8xy7(cpu: &mut Cpu, opcode: Opcode) {
    let (vx, vf) = cpu.get_v(opcode.y).overflowing_sub(cpu.get_v(opcode.x));
    cpu.set_v(opcode.x, vx);
    cpu.set_v(Cpu::VF, vf as u8);
}

fn op_8xye(cpu: &mut Cpu, opcode: Opcode) {
    let vx = cpu.get_v(opcode.x);
    cpu.set_v(opcode.x, vx << 1);
    cpu.set_v(Cpu::VF, Cpu::get_u8_msb(vx));
}

fn op_9xy0(cpu: &mut Cpu, opcode: Opcode) {
    if cpu.get_v(opcode.x) == cpu.get_v(opcode.y) {
        cpu.advance_pc();
    }
}

fn op_annn(cpu: &mut Cpu, opcode: Opcode) {
    println!("Set to: {}", opcode.nnn);
    cpu.i = opcode.nnn; 
}

fn op_bnnn(cpu: &mut Cpu, opcode: Opcode) {
    cpu.jump_pc(cpu.get_v(Cpu::V0) as u16 + opcode.nnn)
}

fn op_cxnn(cpu: &mut Cpu, opcode: Opcode) {
    let mut rng = rand::thread_rng();
    cpu.set_v(opcode.x, (rng.gen_range(0, 0xff) as u8) & opcode.kk);
}

fn op_dxyn(cpu: &mut Cpu, opcode: Opcode) {
    let (mut x, mut y) = (cpu.get_v(opcode.x), cpu.get_v(opcode.y));

    for i in cpu.i..cpu.i + opcode.n as u16 {
        let mut byte = cpu.ram[i as usize];
        for j in 0..7 {
            if ((x + j) as usize) < Display::HEIGHT {
                cpu.registers[Cpu::VF as usize] |= cpu.display.draw_pixel(x + j, y, Cpu::get_u8_msb(byte));
            }
            byte = byte << 1;
        }
        y += 1;
    }
    // println!("{}", cpu.display);
}

fn op_ex9e(cpu: &mut Cpu, opcode: Opcode) {
    if cpu.keyboard[cpu.get_v(opcode.x) as usize] == 1 {
        cpu.advance_pc();
    }
    println!("Checking key is down on {}", cpu.get_v(opcode.x));

}

fn op_exa1(cpu: &mut Cpu, opcode: Opcode) {
    if cpu.keyboard[cpu.get_v(opcode.x) as usize] == 0 {
        cpu.advance_pc();
    }
    println!("Checking key is up on {}", cpu.get_v(opcode.x));
}

fn op_fx07(cpu: &mut Cpu, opcode: Opcode) {
    
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx0a(cpu: &mut Cpu, opcode: Opcode) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx15(cpu: &mut Cpu, opcode: Opcode) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx18(cpu: &mut Cpu, opcode: Opcode) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx1e(cpu: &mut Cpu, opcode: Opcode) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx29(cpu: &mut Cpu, opcode: Opcode) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx33(cpu: &mut Cpu, opcode: Opcode) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx55(cpu: &mut Cpu, opcode: Opcode) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_fx65(cpu: &mut Cpu, opcode: Opcode) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn default(_: &mut Cpu, _: Opcode) {}
