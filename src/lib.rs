pub type OPCODETYPE = u16;

pub struct CPU {
    // current_operation: OPCODETYPE,
    registers: [u8; 16],
    // 16 bit register for memory address
    // register: OPCODETYPE,
    // position in memory
    position_in_memory: usize, // usize as it can be used for indexing, spec is u16
    memory: [u8; 4096],
    stack: [OPCODETYPE; 16],
    stack_pointer: usize,
}

impl CPU {
    pub fn read_opcode(&self) -> OPCODETYPE {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as OPCODETYPE;
        let op_byte2 = self.memory[p + 1] as OPCODETYPE;

        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;
            let kk = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                (0x0, 0x0, 0x0, 0x0) => return,
                (0x0, 0x0, 0xE, 0x0) => { /*Not Implemented */ }
                (0x0, 0x0, 0xE, 0xE) => self.ret(),
                (0x1, _, _, _) => self.jump(nnn),
                (0x2, _, _, _) => self.call(nnn),
                (0x3, _, _, _) => self.kk_skip_eq(x, kk),
                (0x4, _, _, _) => self.kk_skip_ne(x, kk),
                (0x5, _, _, 0x0) => self.xy_skip_eq(x, y),
                (0x6, _, _, _) => self.set(x, kk),
                (0x7, _, _, _) => self.add(x, kk),
                (0x8, _, _, 0x0) => self.set(x, self.registers[y as usize]),
                (0x8, _, _, 0x1) => self.or_bitwise_set(x, y),
                (0x8, _, _, 0x2) => self.and_bitwise_set(x, y),
                (0x8, _, _, 0x3) => self.xor_bitwise_set(x, y),
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0x8, _, _, 0x5) => self.sub_xy(x, y),
                (0x9, _, _, 0x0) => self.xy_skip_ne(x, y),

                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    pub fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }
    pub fn jump(&mut self, nnn: OPCODETYPE) {
        self.position_in_memory = nnn as usize;
    }
    pub fn call(&mut self, addr: OPCODETYPE) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.position_in_memory as OPCODETYPE;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    pub fn xy_skip_eq(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];
        if vx == vy {
            self.position_in_memory += 2;
        }
    }
    pub fn xy_skip_ne(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];
        if vx != vy {
            self.position_in_memory += 2;
        }
    }
    pub fn kk_skip_eq(&mut self, x: u8, kk: u8) {
        let vx = self.registers[x as usize];
        if vx == kk {
            self.position_in_memory += 2;
        }
    }
    pub fn kk_skip_ne(&mut self, x: u8, kk: u8) {
        let vx = self.registers[x as usize];
        if vx != kk {
            self.position_in_memory += 2;
        }
    }
    pub fn set(&mut self, vx: u8, kk_vy: u8) {
        self.registers[vx as usize] = kk_vy;
    }
    pub fn add(&mut self, vx: u8, kk_vy: u8) {
        self.registers[vx as usize] += kk_vy;
    }

    pub fn or_bitwise_set(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vx | vy;
    }
    pub fn and_bitwise_set(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vx & vy;
    }
    pub fn xor_bitwise_set(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vx ^ vy;
    }
    pub fn add_xy(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        let (val, overflow) = vx.overflowing_add(vy);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    pub fn sub_xy(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        let (val, overflow) = vx.overflowing_sub(vy);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 0
        } else {
            self.registers[0xF] = 1
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::CPU;

    fn new_cpu() -> CPU {
        CPU {
            // current_operation: 0,
            registers: [0; 16],
            position_in_memory: 0,
            memory: [0; 4096],
            stack: [0; 16],
            stack_pointer: 0,
        }
    }
    #[test]
    fn x3xkk() {
        let mut cpu = new_cpu();
        cpu.registers[0] = 5;
        cpu.registers[1] = 6;
        cpu.registers[2] = 7;
        cpu.registers[3] = 8;

        let mem = &mut cpu.memory;

        mem[0x000] = 0x30;
        mem[0x001] = 0x05;
        mem[0x002] = 0x80;
        mem[0x003] = 0x24;
        mem[0x004] = 0x80;
        mem[0x005] = 0x34;

        cpu.run();
        // 5 + 7 + 8 = 13 // OPCODE 0x3005 skips next instruction as vx == kk
        assert_eq!(cpu.registers[0], 13);
    }
    #[test]
    fn x4xkk() {
        let mut cpu = new_cpu();
        cpu.registers[0] = 5;
        cpu.registers[1] = 6;
        cpu.registers[2] = 7;
        cpu.registers[3] = 8;

        let mem = &mut cpu.memory;

        mem[0x000] = 0x40;
        mem[0x001] = 0x10;
        mem[0x002] = 0x80;
        mem[0x003] = 0x24;
        mem[0x004] = 0x80;
        mem[0x005] = 0x34;

        cpu.run();
        // 5 + 7 + 8 = 13 // OPCODE 0x4010 skips next instruction as vx != kk
        assert_eq!(cpu.registers[0], 13);
    }
    #[test]
    fn x5xy0() {
        let mut cpu = new_cpu();
        cpu.registers[0] = 12;
        cpu.registers[1] = 12;

        let mem = &mut cpu.memory;

        mem[0x000] = 0x50;
        mem[0x001] = 0x10;
        mem[0x002] = 0x80;
        mem[0x003] = 0x14;

        cpu.run();
        // skip vx = vy
        assert_eq!(cpu.registers[0], 0x000C);
    }
    #[test]
    fn x6xkk() {
        let mut cpu = new_cpu();

        let mem = &mut cpu.memory;
        mem[0x000] = 0x60;
        mem[0x001] = 0x10; // kk = 16

        cpu.run();
        // vx = kk
        assert_eq!(cpu.registers[0], 16);
    }
    #[test]
    fn x7xkk() {
        let mut cpu = new_cpu();
        cpu.registers[0] = 10;

        let mem = &mut cpu.memory;

        mem[0x000] = 0x70;
        mem[0x001] = 0x10; // kk = 16

        cpu.run();
        // vx = vy
        assert_eq!(cpu.registers[0], 26);
    }
    #[test]
    fn x8xy0() {
        let mut cpu = new_cpu();
        cpu.registers[0] = 9;
        cpu.registers[1] = 99;

        let mem = &mut cpu.memory;

        mem[0x000] = 0x80;
        mem[0x001] = 0x10;

        cpu.run();
        // vx = vy
        assert_eq!(cpu.registers[0], cpu.registers[1]);
    }
    #[test]
    fn x8xy1() {
        let mut cpu = new_cpu();
        cpu.registers[0] = 9;
        cpu.registers[1] = 99;

        let mem = &mut cpu.memory;

        mem[0x000] = 0x80;
        mem[0x001] = 0x11;

        cpu.run();
        // BITWSIE OR
        assert_eq!(cpu.registers[0], 107);
    }

    #[test]
    fn x8xy2() {
        let mut cpu = new_cpu();
        cpu.registers[0] = 9;
        cpu.registers[1] = 99;

        let mem = &mut cpu.memory;

        mem[0x000] = 0x80;
        mem[0x001] = 0x12;

        cpu.run();
        // BITWISE AND
        assert_eq!(cpu.registers[0], 1);
    }
    #[test]
    fn x8xy3() {
        let mut cpu = new_cpu();
        cpu.registers[0] = 9;
        cpu.registers[1] = 99;

        let mem = &mut cpu.memory;

        mem[0x000] = 0x80;
        mem[0x001] = 0x13;

        cpu.run();
        // BITWISE XOR
        assert_eq!(cpu.registers[0], 106);
    }
    #[test]
    fn x8xy4() {
        let mut cpu = new_cpu();

        // opcode `0x8014`
        // 8 signifies involment of two registers.
        // 0 maps to cpu.registers[0].
        // 1 maps to cpu.registers[1].
        // 4 indicates addition
        // cpu.current_operation = 0x8014;
        cpu.registers[0] = 5;
        cpu.registers[1] = 10;
        cpu.registers[2] = 10;
        cpu.registers[3] = 10;

        let mem = &mut cpu.memory;
        mem[0] = 0x80; // register one high nible  opcode: 0x8014 adds register 1 to 0
        mem[1] = 0x14; // register one low nible ...
        mem[2] = 0x80; // register two high nibble opcode: 0x8024 adds register 2 to 0
        mem[3] = 0x24; // register two low nibble ...
        mem[4] = 0x80; // register three high nibble opcode: 0x8034 adds register 3 to 0
        mem[5] = 0x34; // register three low nibble ...

        cpu.run();
        // "5 + 10 + 10 + 10 = 35"
        assert_eq!(cpu.registers[0], 35);
    }
    #[test]
    fn x8xy5() {
        let mut cpu = new_cpu();
        cpu.registers[0] = 30;
        cpu.registers[1] = 5;
        cpu.registers[2] = 5;
        cpu.registers[3] = 5;

        let mem = &mut cpu.memory;
        mem[0] = 0x80;
        mem[1] = 0x15;
        mem[2] = 0x80;
        mem[3] = 0x25;
        mem[4] = 0x80;
        mem[5] = 0x35;

        cpu.run();
        // 30 - 5 - 5 - 5 = 15
        assert_eq!(cpu.registers[0], 15);
    }
    #[test]
    fn x9xy0() {
        let mut cpu = new_cpu();
        cpu.registers[0] = 11;
        cpu.registers[1] = 12;

        let mem = &mut cpu.memory;

        mem[0x000] = 0x90;
        mem[0x001] = 0x10;
        mem[0x002] = 0x80;
        mem[0x003] = 0x14;

        cpu.run();
        // skip if vx != vy
        assert_eq!(cpu.registers[0],11);
    }
    #[test]
    fn addition_and_multiplication() {
        let mut cpu = new_cpu();

        cpu.registers[0] = 5;
        cpu.registers[1] = 10;

        let mem = &mut cpu.memory;
        mem[0x000] = 0x21; // opcode to 0x2100 and CALL function at 0x100
        mem[0x001] = 0x00; // ...
        mem[0x002] = 0x21; // Opcode to 0x2100 and CALL function at 0x100
        mem[0x003] = 0x00; // ...
        mem[0x004] = 0x00; // Opcode to 0x0000 and HALT
        mem[0x005] = 0x00; // ..

        mem[0x100] = 0x80; // Opcode to 0x8014 and ADD reg 1 to 0
        mem[0x101] = 0x14; // ..
        mem[0x102] = 0x80; // Opcode to 0x8014 and ADD reg 1 to 0
        mem[0x103] = 0x14; // ..
        mem[0x104] = 0x00; // opcode to 0x00EE and RETURN
        mem[0x105] = 0xEE; // ..

        cpu.run();
        // 5 + (10 * 2) + (10 * 2) = 45
        assert_eq!(cpu.registers[0], 45);
    }
}
