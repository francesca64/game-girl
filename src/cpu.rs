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

enum Reg8Name {
    A, F,
    B, C,
    D, E,
    H, L
}

enum Operand<'a> {
    Reg8(Reg8Name),
    Reg16(&'a mut u16),
    Immediate8, // d8
    Immediate16, // d16
    Addr8, // a8
    Addr16, // a16
    Signed8, // r8
    HLAddr, // (HL)
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

    fn reg8_string(&self, reg_name: &Reg8Name) -> &str {
        match reg_name {
            &Reg8Name::A => "A",
            &Reg8Name::B => "B",
            &Reg8Name::C => "C",
            &Reg8Name::D => "D",
            &Reg8Name::E => "E",
            &Reg8Name::F => "F",
            &Reg8Name::H => "H",
            &Reg8Name::L => "L"
        }
    }

    fn reg8_read(&mut self, reg_name: Reg8Name) -> u8 {
        match reg_name {
            Reg8Name::A => self.a,
            Reg8Name::B => self.b,
            Reg8Name::C => self.c,
            Reg8Name::D => self.d,
            Reg8Name::E => self.e,
            Reg8Name::F => self.f,
            Reg8Name::H => self.h,
            Reg8Name::L => self.l
        }
    }

    fn reg8_write(&mut self, reg_name: Reg8Name, value: u8) {
        let mut reg = match reg_name {
            Reg8Name::A => self.a,
            Reg8Name::B => self.b,
            Reg8Name::C => self.c,
            Reg8Name::D => self.d,
            Reg8Name::E => self.e,
            Reg8Name::F => self.f,
            Reg8Name::H => self.h,
            Reg8Name::L => self.l
        };
        reg = value;
    }

    fn opcode_exec(&mut self, opcode: u8) {
        //println!("PC {}", self.pc);
        print!("{:02X} ", opcode);
        match opcode {
            0x00 => self.nop(),

            // ANDs
            0xA0 => self.and(Operand::Reg8(Reg8Name::B)),
            0xA1 => self.and(Operand::Reg8(Reg8Name::C)),
            0xA2 => self.and(Operand::Reg8(Reg8Name::D)),
            0xA3 => self.and(Operand::Reg8(Reg8Name::E)),
            0xA4 => self.and(Operand::Reg8(Reg8Name::H)),
            0xA5 => self.and(Operand::Reg8(Reg8Name::L)),
            0xA6 => self.and(Operand::HLAddr),
            0xA7 => self.and(Operand::Reg8(Reg8Name::A)),
            0xE6 => self.and(Operand::Immediate8),

            // ORs
            0xB0 => self.or(Operand::Reg8(Reg8Name::B)),
            0xB1 => self.or(Operand::Reg8(Reg8Name::C)),
            0xB2 => self.or(Operand::Reg8(Reg8Name::D)),
            0xB3 => self.or(Operand::Reg8(Reg8Name::E)),
            0xB4 => self.or(Operand::Reg8(Reg8Name::H)),
            0xB5 => self.or(Operand::Reg8(Reg8Name::L)),
            0xB6 => self.or(Operand::HLAddr),
            0xB7 => self.or(Operand::Reg8(Reg8Name::A)),
            0xF6 => self.or(Operand::Immediate8),

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
            0x78 => self.ld_a_b(),
            0xC9 => self.ret(),
            0x31 => self.ld_sp_d16(),
            0x21 => self.ld_hl_d16(),
            0x01 => self.ld_bc_d16(),
            0x36 => self.ld_hl_d8(),
            0x23 => self.inc_hl(),
            0x0B => self.dec_bc(),
            0x4A => self.ld_c_d(),
            0x26 => self.ld_h_d8(),
            0x04 => self.inc_b(),
            0x6B => self.ld_l_e(),
            0x22 => self.ldi_hl_a(),
            0x1D => self.dec_e(),
            0x15 => self.dec_d(),
            0x3D => self.dec_a(),
            0xE5 => self.push_hl(),
            0xD5 => self.push_de(),
            0xC5 => self.push_bc(),
            0xFA => self.ld_a_a16(),
            0xCB => {
                let opcode = self.mem.read_u8(self.pc+1);
                print!("{:02X} ", opcode);
                match opcode {
                    0x87 => self.cb_res_0_a(),
                    _ => {
                        println!("Unimplemented");
                        self.mem.dump();
                        panic!()
                    }
                }
            },
            _ => {
                println!("Unimplemented");
                self.mem.dump();
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
        let popped = self.mem.read_u8(self.sp);
        self.sp += 1;
        popped
    }

    fn pop_stack_u16(&mut self) -> u16 {
        let popped = switch_u16(self.mem.read_u16(self.sp));
        self.sp += 2;
        popped
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
        // 0x91 = 145 = first line of vblank period.
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



    fn ld_a_b(&mut self) {
        println!("LD A,B");
        self.a = self.b;
        self.pc += 1;
    }

    fn ret(&mut self) {
        println!("RET");
        self.pc = self.pop_stack_u16();
    }

    fn ld_sp_d16(&mut self) {
        let operand = self.mem.read_u16(self.pc+1);
        println!("LD SP,{:02X}", operand);
        self.sp = operand;
        self.pc += 3;
    }

    fn ld_hl_d16(&mut self) {
        let operand = self.mem.read_u16(self.pc+1);
        println!("LD HL,{:02X}", operand);
        let bytes = u16_to_2u8s(operand);
        self.h = bytes.0;
        self.l = bytes.1;
        self.pc += 3;
    }

    fn ld_bc_d16(&mut self) {
        let operand = self.mem.read_u16(self.pc+1);
        println!("LD BC,{:02X}", operand);
        let bytes = u16_to_2u8s(operand);
        self.b = bytes.0;
        self.c = bytes.1;
        self.pc += 3;
    }

    fn ld_hl_d8(&mut self) {
        let operand = self.mem.read_u8(self.pc+1);
        println!("LD (HL),{:02X}", operand);
        self.l = operand;
        self.pc += 2;
    }

    fn inc_hl(&mut self) {
        println!("INC HL");
        let value = u16_from_2u8s((self.l, self.h)) + 1;
        let bytes = u16_to_2u8s(value);
        self.h = bytes.0;
        self.l = bytes.1;
        self.pc += 1;
    }

    fn dec_bc(&mut self) {
        println!("DEC BC");
        let value = u16_from_2u8s((self.c, self.b)) - 1;
        let bytes = u16_to_2u8s(value);
        self.b = bytes.0;
        self.c = bytes.1;
        self.pc += 1;
    }

    fn ld_c_d(&mut self) {
        println!("LD C,D");
        self.c = self.d;
        self.pc += 1;
    }

    fn ld_h_d8(&mut self) {
        let operand = self.mem.read_u8(self.pc+1);
        println!("LD H,{:02X}", operand);
        self.h = operand;
        self.pc += 2;
    }

    fn inc_b(&mut self) {
        println!("INC B");
        self.b.saturating_add(1);
        if self.b == 0 {
            self.set_f_zero();
            // This might not be the correct interpretation of "Set if carry from bit 3."
            self.set_f_halfcarry();
        }
        self.reset_f_subtraction();
        self.pc += 1;
    }

    fn ld_l_e(&mut self) {
        println!("LD L,E");
        self.l = self.e;
        self.pc += 1;
    }

    fn read_hladdr_u8(&self) -> u8 {
        let addr = u16_from_2u8s((self.l, self.h));
        self.mem.read_u8(addr)
    }

    fn write_hladdr_u8(&mut self, value: u8) {
        let addr = u16_from_2u8s((self.l, self.h));
        self.mem.write_u8(addr, value);
    }

    fn inc_hl_(&mut self) {
        let value = u16_from_2u8s((self.l, self.h)) + 1;
        let bytes = u16_to_2u8s(value);
        self.h = bytes.0;
        self.l = bytes.1;
    }

    fn ldi_hl_a(&mut self) {
        println!("LDI (HL),A");
        let value = self.a;
        self.write_hladdr_u8(value);
        self.inc_hl_();
        self.pc += 1;
    }

    fn dec_e(&mut self) {
        println!("DEC E");
        self.e.saturating_sub(1);
        if self.e == 0 {
            self.set_f_zero();
            // I'm not sure how to interpret "Set if no borrow from bit 4."
            self.set_f_halfcarry();
        }
        self.set_f_subtraction();
        self.pc += 1;
    }

    fn dec_d(&mut self) {
        println!("DEC D");
        self.d.saturating_sub(1);
        if self.d == 0 {
            self.set_f_zero();
            self.set_f_halfcarry();
        }
        self.set_f_subtraction();
        self.pc += 1;
    }

    fn dec_a(&mut self) {
        println!("DEC A");
        self.a.saturating_sub(1);
        if self.a == 0 {
            self.set_f_zero();
            self.set_f_halfcarry();
        }
        self.set_f_subtraction();
        self.pc += 1;
    }

    fn push_hl(&mut self) {
        println!("PUSH HL");
        let value = u16_from_2u8s((self.l, self.h));
        self.push_stack_u16(value);
        self.pc += 1;
    }

    fn push_de(&mut self) {
        println!("PUSH HL");
        let value = u16_from_2u8s((self.e, self.d));
        self.push_stack_u16(value);
        self.pc += 1;
    }

    fn push_bc(&mut self) {
        println!("PUSH HL");
        let value = u16_from_2u8s((self.c, self.b));
        self.push_stack_u16(value);
        self.pc += 1;
    }

    fn ld_a_a16(&mut self) {
        let addr = self.mem.read_u16(self.pc+1);
        println!("LD A,({:02X})", addr);
        self.a = self.mem.read_u8(addr);
        self.pc += 3;
    }

    fn and(&mut self, operand: Operand) {
        match operand {
            Operand::Reg8(reg_name) => {
                println!("AND {}", self.reg8_string(&reg_name));
                self.a &= self.reg8_read(reg_name);
                self.pc += 1;
            },
            Operand::HLAddr => {
                println!("AND (HL)");
                let value = self.read_hladdr_u8();
                self.a &= value;
                self.pc += 1;
            },
            Operand::Immediate8 => {
                let value = self.mem.read_u8(self.pc+1);
                println!("AND {:02X}", value);
                self.a &= value;
                self.pc += 2;
            },
            _ => unreachable!("AND only supports Reg8, HLAddr, and Immediate8.")
        }
        self.f = 0b0010_0000u8;
        if self.a == 0 {
            self.set_f_zero();
        }
    }

    fn or(&mut self, operand: Operand) {
        match operand {
            Operand::Reg8(reg_name) => {
                println!("OR {}", self.reg8_string(&reg_name));
                self.a |= self.reg8_read(reg_name);
                self.pc += 1;
            },
            Operand::HLAddr => {
                println!("OR (HL)");
                let value = self.read_hladdr_u8();
                self.a |= value;
                self.pc += 1;
            },
            Operand::Immediate8 => {
                let value = self.mem.read_u8(self.pc+1);
                println!("OR {:02X}", value);
                self.a |= value;
                self.pc += 2;
            },
            _ => unreachable!("OR only supports Reg8, HLAddr, and Immediate8.")
        }
        self.f = 0;
        if self.a == 0 {
            self.set_f_zero();
        }
    }
}
