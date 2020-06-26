use crate::cpu::Cpu;


pub fn get_opcode_func(opcode: u16) -> fn(&mut Cpu, u16) {
    match opcode {
        0x00EE => op_ooee,
        _ => default,
    }
}

fn op_ooee(cpu: &mut Cpu, _: u16) {
    println!("{} Return from subroutine!", cpu.pc);
}

fn default(_: &mut Cpu, _: u16) {}

