pub type OPCODETYPE = u16;

pub struct CPU {
    // current_operation: OPCODETYPE,
    registers: [u8; 16],
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

            let nnn = opcode & 0xFFF;
            let _kk = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_xy(x, y),

                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    pub fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
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

    pub fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }
}
#[cfg(test)]
mod tests {
    use crate::CPU;

    #[test]
    fn basic_addition() {
        let mut cpu = CPU {
            // current_operation: 0,
            registers: [0; 16],
            position_in_memory: 0,
            memory: [0; 4096],
            stack: [0; 16],
            stack_pointer: 0,
        };

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

        assert_eq!(cpu.registers[0], 35);
    }

    #[test]
    fn addition_and_multiplication() {
        let mut cpu = CPU {
            registers: [0; 16],
            position_in_memory: 0,
            memory: [0; 4096],
            stack: [0; 16],
            stack_pointer: 0,
        };

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

        assert_eq!(cpu.registers[0], 45);
    }
}
