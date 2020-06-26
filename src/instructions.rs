use crate::cpu::Cpu;

pub fn get_opcode_func(opcode: u16) -> fn(&mut Cpu, u16) {
    match opcode {
        0x00EE => op_ooee,
        _ => default,
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_0nnn(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_00e0(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_ooee(cpu: &mut Cpu, _: u16) {
    println!("{} Return from subroutine!", cpu.pc);
}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_1nnn(cpu: &mut Cpu, opcode: u16) {}

#[allow(unused_variables)]
#[allow(dead_code)]
fn op_2nnn(cpu: &mut Cpu, opcode: u16) {}

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

