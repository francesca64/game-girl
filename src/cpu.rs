use util::*;
use mem::*;

pub struct Cpu {
    a: u8, f: u8,
    b: u8, c: u8,
    d: u8, e: u8,
    h: u8, l: u8,
    sp: u16,
    pc: u16,

    mem: Mem
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            a: 0x0, f: 0x0,
            b: 0x0, c: 0x0,
            d: 0x0, e: 0x0,
            h: 0x0, l: 0x0,
            sp: 0xFFFE,
            pc: 0x100,

            mem: Mem::new()
        }
    }

    pub fn run(&mut self, rom: &[u8]) -> () {
        println!("Beginning execution.");

        self.mem.load_fixed_rom_bank(&rom[0 .. 0x4000]);

        loop {
            let opcode = self.mem.read_u8(self.pc);
            self.opcode_exec(opcode)
        }
    }

    fn opcode_exec(&mut self, opcode: u8) -> () {
        //println!("PC {}", self.pc);
        print!("{:02X} ", opcode);
        match opcode {
            0x00 => self.nop(),
            0xC3 => self.jump_nn(),
            0xFE => self.cp_n(),
            0x28 => self.jr_z(),
            0xAF => self.xor_a(),
            0x18 => self.jr_r8(),
            0x02 => self.ld_bc_a(),
            0x3E => self.ld_a_d8(),
            0xEA => self.ld_a16_a(),
            0xF3 => self.di(),
            0x0F => self.rrca(),
            0xE0 => self.ldh_a8_a(),
            0xCD => self.call(),
            0xF0 => self.ldh_a_a8(),
            0x47 => self.ld_b_a(),
            0x20 => self.jr_nz_r8(),
            0xCB => {
                let opcode = self.mem.read_u8(self.pc+1);
                print!("{:02X} ", opcode);
                match opcode {
                    0x87 => self.cb_res_0_a(),
                    _ => {
                        println!("Unimplemented");
                        panic!()
                    }
                }
            },
            _ => {
                println!("Unimplemented");
                panic!()
            }
        }
    }

    fn push_stack_u8(&mut self, value: u8) {
        self.sp -= 1;
        self.mem.write_u8(self.sp, value);
    }

    fn push_stack_u16(&mut self, value: u16) {
        self.sp -= 2;
        self.mem.write_u16(self.sp, value);
    }

    fn pop_stack_u8(&mut self) -> u8 {
        self.sp += 1;
        self.mem.read_u8(self.sp)
    }

    fn pop_stack_u16(&mut self) -> u16 {
        self.sp += 2;
        self.mem.read_u16(self.sp)
    }

    fn is_f_zero(&self) -> bool {
        (0b1000_0000u8 & self.f) == 0b1000_0000u8
    }

    fn set_f_zero(&mut self) {
        self.f |= 0b1000_0000u8
    }

    fn reset_f_zero(&mut self) {
        self.f ^= 0b1000_0000u8
    }

    fn is_f_subtraction(&self) -> bool {
        (0b01000_000u8 & self.f) == 0b0100_0000u8
    }

    fn set_f_subtraction(&mut self) {
        self.f |= 0b0100_0000u8
    }

    fn reset_f_subtraction(&mut self) {
        self.f ^= 0b0100_0000u8
    }

    fn is_f_halfcarry(&self) -> bool {
        (0b0010_0000u8 & self.f) == 0b0010_0000u8
    }

    fn set_f_halfcarry(&mut self) {
        self.f |= 0b0010_0000u8
    }

    fn reset_f_halfcarry(&mut self) {
        self.f ^= 0b0010_0000u8
    }

    fn is_f_carry(&self) -> bool {
        (0b0001_0000u8 & self.f) == 0b0001_0000u8
    }

    fn set_f_carry(&mut self) {
        self.f |= 0b0001_0000u8
    }

    fn reset_f_carry(&mut self) {
        self.f ^= 0b0001_0000u8
    }

    fn nop(&mut self) {
        self.pc += 1;
        println!("NOP");
    }

    fn jump_nn(&mut self) {
        self.pc = self.mem.read_u16(self.pc+1);
        println!("JP {:02X}", self.pc);
    }

    fn cp_n(&mut self) {
        let operand = self.mem.read_u8(self.pc+1);
        println!("CP {:02X} {:02X}", operand, self.a);

        // Remove later!
        let hack = self.pc == 109 && operand == 0x91;

        self.f = 0x0;
        if operand == self.a || hack  {
            self.set_f_zero();
            self.set_f_subtraction();
        } else if self.a < operand {
            self.set_f_carry();
        }

        let half_carry = ((self.a as i8 & 0xF) - (operand as i8 & 0xF)) & 0x10;
        if half_carry == 0x10 {
            self.set_f_halfcarry();
        }

        self.pc += 2;
    }

    fn jr_z(&mut self) {
        let operand = self.mem.read_u16(self.pc+1);
        if self.is_f_zero() {
            self.pc = operand;
        } else {
            self.pc += 2;
        }
        println!("JR Z,{:02X}", operand);
    }

    fn xor_a(&mut self) {
        self.f = 0x0;
        self.a ^= self.a;
        if self.a == 0x0 {
            self.set_f_zero();
        }
        self.pc += 1;
        println!("XOR A,A");
    }

    fn jr_r8(&mut self) {
        let operand = self.mem.read_u8(self.pc+1) as i8;
        if operand > 0 {
            self.pc += operand as u16;
        } else {
            self.pc -= (operand * -1) as u16;
        }
        println!("JR {:02X}", operand);
    }

    fn ld_bc_a(&mut self) {
        let addr = u16_from_2u8s((self.b, self.c));
        self.mem.write_u8(addr, self.a);
        self.pc += 1;
        println!("LD (BC),A");
    }

    fn ld_a_d8(&mut self) {
        let operand = self.mem.read_u8(self.pc+1);
        self.a = operand;
        self.pc += 2;
        println!("LD A,{:02X}", operand);
    }

    fn ld_a16_a(&mut self) {
        let addr = self.mem.read_u16(self.pc+1);
        self.mem.write_u8(addr, self.a);
        self.pc += 3;
        println!("LD {:02X},A", addr);
    }

    fn di(&mut self) {
        // Disable interrupts
        self.pc += 1;
        println!("DI");
    }

    fn rrca(&mut self) {
        self.f = self.a & 0b0000_0001u8;
        self.a >>= 1;
        if self.a == 0x0 {
            self.set_f_zero();
        }
        self.pc += 1;
        println!("RRCA");
    }

    fn ldh_a8_a(&mut self) {
        let operand = self.mem.read_u8(self.pc+1);
        let addr = 0xFF00 + operand as u16;
        self.mem.write_u8(addr, self.a);
        self.pc += 2;
        println!("LDH {:02X},A", operand);
    }

    fn call(&mut self) {
        let addr = self.mem.read_u16(self.pc+1);
        let next = self.pc+3;
        self.push_stack_u16(next);
        self.pc = addr;
        println!("CALL {:02X}", addr);
    }

    fn ldh_a_a8(&mut self) {
        let operand = self.mem.read_u8(self.pc+1);
        let addr = 0xFF00 + operand as u16;
        println!("LDH A,{:02X}", addr);
        self.a = self.mem.read_u8(addr);
        self.pc += 2;
    }

    fn ld_b_a(&mut self) {
        self.b = self.a;
        self.pc += 1;
        println!("LD B,A");
    }

    fn cb_res_0_a(&mut self) {
        self.a ^= 0b0000_0001u8;
        self.pc += 2;
        println!("RES 0,A");
    }

    fn jr_nz_r8(&mut self) {
        let operand = self.mem.read_u8(self.pc+1) as i8;
        println!("JR NZ,{:02X}", operand);
        if ! self.is_f_zero() {
            if operand > 0 {
                self.pc += operand as u16;
            } else {
                self.pc -= (operand * -1) as u16;
            }
        } else {
            self.pc += 2;
        }
        if self.pc == 105 {
            panic!();
        }
    }
}
