pub struct Cpu {
    pub registers: [u8; 16],
    pub memory: [u8; 4096],
}

impl Cpu {
    pub fn new() -> Cpu {
        println!("Hello");
        Cpu {
            registers: [0; 16],
            memory: [0; 4096],
        }
    }
}
