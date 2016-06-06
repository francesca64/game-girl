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
    A, //F,
    B, C,
    D, E,
    H, L
}

enum Condition {
    NZ,
    Z,
    NC,
    C
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
    Condition // cc
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
            //&Reg8Name::F => "F",
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
            //Reg8Name::F => self.f,
            Reg8Name::H => self.h,
            Reg8Name::L => self.l
        }
    }

    fn reg8_write(&mut self, reg_name: Reg8Name, value: u8) {
        let reg = match reg_name {
            Reg8Name::A => &mut self.a,
            Reg8Name::B => &mut self.b,
            Reg8Name::C => &mut self.c,
            Reg8Name::D => &mut self.d,
            Reg8Name::E => &mut self.e,
            //Reg8Name::F => &mut self.f,
            Reg8Name::H => &mut self.h,
            Reg8Name::L => &mut self.l
        };
        *reg = value;
    }

    fn cond_string(&self, cond: &Condition) -> &str {
        match cond {
            &Condition::NZ => "NZ",
            &Condition::Z => "Z",
            &Condition::NC => "NC",
            &Condition::C => "C"
        }
    }

    fn cond_eval(&mut self, cond: Condition) -> bool {
        match cond {
            Condition::NZ => ! self.is_f_zero(),
            Condition::Z => self.is_f_zero(),
            Condition::NC => ! self.is_f_carry(),
            Condition::C => self.is_f_carry()
        }
    }

    fn opcode_exec(&mut self, opcode: u8) {
        //println!("PC {}", self.pc);
        print!("{:02X} ", opcode);
        match opcode {
            0x00 => self.nop(),

            // LDs
            0x7F => self.ld(Operand::Reg8(Reg8Name::A), Operand::Reg8(Reg8Name::A)),
            0x78 => self.ld(Operand::Reg8(Reg8Name::A), Operand::Reg8(Reg8Name::B)),
            0x79 => self.ld(Operand::Reg8(Reg8Name::A), Operand::Reg8(Reg8Name::C)),
            0x7A => self.ld(Operand::Reg8(Reg8Name::A), Operand::Reg8(Reg8Name::D)),
            0x7B => self.ld(Operand::Reg8(Reg8Name::A), Operand::Reg8(Reg8Name::E)),
            0x7C => self.ld(Operand::Reg8(Reg8Name::A), Operand::Reg8(Reg8Name::H)),
            0x7D => self.ld(Operand::Reg8(Reg8Name::A), Operand::Reg8(Reg8Name::L)),
            0x7E => self.ld(Operand::Reg8(Reg8Name::A), Operand::HLAddr),
            0x40 => self.ld(Operand::Reg8(Reg8Name::B), Operand::Reg8(Reg8Name::B)),
            0x41 => self.ld(Operand::Reg8(Reg8Name::B), Operand::Reg8(Reg8Name::C)),
            0x42 => self.ld(Operand::Reg8(Reg8Name::B), Operand::Reg8(Reg8Name::D)),
            0x43 => self.ld(Operand::Reg8(Reg8Name::B), Operand::Reg8(Reg8Name::E)),
            0x44 => self.ld(Operand::Reg8(Reg8Name::B), Operand::Reg8(Reg8Name::H)),
            0x45 => self.ld(Operand::Reg8(Reg8Name::B), Operand::Reg8(Reg8Name::L)),
            0x46 => self.ld(Operand::Reg8(Reg8Name::B), Operand::HLAddr),
            0x48 => self.ld(Operand::Reg8(Reg8Name::C), Operand::Reg8(Reg8Name::B)),
            0x49 => self.ld(Operand::Reg8(Reg8Name::C), Operand::Reg8(Reg8Name::C)),
            0x4A => self.ld(Operand::Reg8(Reg8Name::C), Operand::Reg8(Reg8Name::D)),
            0x4B => self.ld(Operand::Reg8(Reg8Name::C), Operand::Reg8(Reg8Name::E)),
            0x4C => self.ld(Operand::Reg8(Reg8Name::C), Operand::Reg8(Reg8Name::H)),
            0x4D => self.ld(Operand::Reg8(Reg8Name::C), Operand::Reg8(Reg8Name::L)),
            0x4E => self.ld(Operand::Reg8(Reg8Name::C), Operand::HLAddr),
            0x50 => self.ld(Operand::Reg8(Reg8Name::D), Operand::Reg8(Reg8Name::B)),
            0x51 => self.ld(Operand::Reg8(Reg8Name::D), Operand::Reg8(Reg8Name::C)),
            0x52 => self.ld(Operand::Reg8(Reg8Name::D), Operand::Reg8(Reg8Name::D)),
            0x53 => self.ld(Operand::Reg8(Reg8Name::D), Operand::Reg8(Reg8Name::E)),
            0x54 => self.ld(Operand::Reg8(Reg8Name::D), Operand::Reg8(Reg8Name::H)),
            0x55 => self.ld(Operand::Reg8(Reg8Name::D), Operand::Reg8(Reg8Name::L)),
            0x56 => self.ld(Operand::Reg8(Reg8Name::D), Operand::HLAddr),
            0x58 => self.ld(Operand::Reg8(Reg8Name::E), Operand::Reg8(Reg8Name::B)),
            0x59 => self.ld(Operand::Reg8(Reg8Name::E), Operand::Reg8(Reg8Name::C)),
            0x5A => self.ld(Operand::Reg8(Reg8Name::E), Operand::Reg8(Reg8Name::D)),
            0x5B => self.ld(Operand::Reg8(Reg8Name::E), Operand::Reg8(Reg8Name::E)),
            0x5C => self.ld(Operand::Reg8(Reg8Name::E), Operand::Reg8(Reg8Name::H)),
            0x5D => self.ld(Operand::Reg8(Reg8Name::E), Operand::Reg8(Reg8Name::L)),
            0x5E => self.ld(Operand::Reg8(Reg8Name::E), Operand::HLAddr),
            0x60 => self.ld(Operand::Reg8(Reg8Name::H), Operand::Reg8(Reg8Name::B)),
            0x61 => self.ld(Operand::Reg8(Reg8Name::H), Operand::Reg8(Reg8Name::C)),
            0x62 => self.ld(Operand::Reg8(Reg8Name::H), Operand::Reg8(Reg8Name::D)),
            0x63 => self.ld(Operand::Reg8(Reg8Name::H), Operand::Reg8(Reg8Name::E)),
            0x64 => self.ld(Operand::Reg8(Reg8Name::H), Operand::Reg8(Reg8Name::H)),
            0x65 => self.ld(Operand::Reg8(Reg8Name::H), Operand::Reg8(Reg8Name::L)),
            0x66 => self.ld(Operand::Reg8(Reg8Name::H), Operand::HLAddr),
            0x68 => self.ld(Operand::Reg8(Reg8Name::L), Operand::Reg8(Reg8Name::B)),
            0x69 => self.ld(Operand::Reg8(Reg8Name::L), Operand::Reg8(Reg8Name::C)),
            0x6A => self.ld(Operand::Reg8(Reg8Name::L), Operand::Reg8(Reg8Name::D)),
            0x6B => self.ld(Operand::Reg8(Reg8Name::L), Operand::Reg8(Reg8Name::E)),
            0x6C => self.ld(Operand::Reg8(Reg8Name::L), Operand::Reg8(Reg8Name::H)),
            0x6D => self.ld(Operand::Reg8(Reg8Name::L), Operand::Reg8(Reg8Name::L)),
            0x6E => self.ld(Operand::Reg8(Reg8Name::L), Operand::HLAddr),
            0x70 => self.ld(Operand::HLAddr, Operand::Reg8(Reg8Name::B)),
            0x71 => self.ld(Operand::HLAddr, Operand::Reg8(Reg8Name::C)),
            0x72 => self.ld(Operand::HLAddr, Operand::Reg8(Reg8Name::D)),
            0x73 => self.ld(Operand::HLAddr, Operand::Reg8(Reg8Name::E)),
            0x74 => self.ld(Operand::HLAddr, Operand::Reg8(Reg8Name::H)),
            0x75 => self.ld(Operand::HLAddr, Operand::Reg8(Reg8Name::L)),
            0x36 => self.ld(Operand::HLAddr, Operand::Immediate8),

            // JRs
            0x18 => self.jr(None, Operand::Immediate8),
            0x20 => self.jr(Some(Condition::NZ), Operand::Immediate8),
            0x28 => self.jr(Some(Condition::Z), Operand::Immediate8),
            0x30 => self.jr(Some(Condition::NC), Operand::Immediate8),
            0x38 => self.jr(Some(Condition::C), Operand::Immediate8),

            // ADDs
            0x80 => self.add(Operand::Reg8(Reg8Name::B)),
            0x81 => self.add(Operand::Reg8(Reg8Name::C)),
            0x82 => self.add(Operand::Reg8(Reg8Name::D)),
            0x83 => self.add(Operand::Reg8(Reg8Name::E)),
            0x84 => self.add(Operand::Reg8(Reg8Name::H)),
            0x85 => self.add(Operand::Reg8(Reg8Name::L)),
            0x86 => self.add(Operand::HLAddr),
            0x87 => self.add(Operand::Reg8(Reg8Name::A)),
            0xC6 => self.add(Operand::Immediate8),

            // SUBs
            0x90 => self.sub(Operand::Reg8(Reg8Name::B)),
            0x91 => self.sub(Operand::Reg8(Reg8Name::C)),
            0x92 => self.sub(Operand::Reg8(Reg8Name::D)),
            0x93 => self.sub(Operand::Reg8(Reg8Name::E)),
            0x94 => self.sub(Operand::Reg8(Reg8Name::H)),
            0x95 => self.sub(Operand::Reg8(Reg8Name::L)),
            0x96 => self.sub(Operand::HLAddr),
            0x97 => self.sub(Operand::Reg8(Reg8Name::A)),
            0xD6 => self.sub(Operand::Immediate8),

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

            // XORs
            0xA8 => self.xor(Operand::Reg8(Reg8Name::B)),
            0xA9 => self.xor(Operand::Reg8(Reg8Name::C)),
            0xAA => self.xor(Operand::Reg8(Reg8Name::D)),
            0xAB => self.xor(Operand::Reg8(Reg8Name::E)),
            0xAC => self.xor(Operand::Reg8(Reg8Name::H)),
            0xAD => self.xor(Operand::Reg8(Reg8Name::L)),
            0xAE => self.xor(Operand::HLAddr),
            0xAF => self.xor(Operand::Reg8(Reg8Name::A)),
            0xEE => self.xor(Operand::Immediate8),

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

            // CPs
            0xB8 => self.cp(Operand::Reg8(Reg8Name::B)),
            0xB9 => self.cp(Operand::Reg8(Reg8Name::C)),
            0xBA => self.cp(Operand::Reg8(Reg8Name::D)),
            0xBB => self.cp(Operand::Reg8(Reg8Name::E)),
            0xBC => self.cp(Operand::Reg8(Reg8Name::H)),
            0xBD => self.cp(Operand::Reg8(Reg8Name::L)),
            0xBE => self.cp(Operand::HLAddr),
            0xBF => self.cp(Operand::Reg8(Reg8Name::A)),
            0xFE => self.cp(Operand::Immediate8),

            0xC3 => self.jump_nn(),
            0x02 => self.ld_bc_a(),
            0x3E => self.ld_a_d8(),
            0xEA => self.ld_a16_a(),
            0xF3 => self.di(),
            0x0F => self.rrca(),
            0xE0 => self.ldh_a8_a(),
            0xCD => self.call(),
            0xF0 => self.ldh_a_a8(),
            0x47 => self.ld_b_a(),
            0xC9 => self.ret(),
            0x31 => self.ld_sp_d16(),
            0x21 => self.ld_hl_d16(),
            0x01 => self.ld_bc_d16(),
            0x23 => self.inc_hl(),
            0x0B => self.dec_bc(),
            0x26 => self.ld_h_d8(),
            0x04 => self.inc_b(),
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

    fn ld(&mut self, r1: Operand, r2: Operand) {
        match r1 {
            Operand::Reg8(reg_name) => {
                match r2 {
                    Operand::Reg8(second_reg_name) => {
                        println!("LD {},{}",
                            self.reg8_string(&reg_name), self.reg8_string(&second_reg_name));
                        let value = self.reg8_read(second_reg_name);
                        self.reg8_write(reg_name, value);
                    },
                    Operand::HLAddr => {
                        println!("LD {},(HL)", self.reg8_string(&reg_name));
                        let value = self.read_hladdr_u8();
                        self.reg8_write(reg_name, value);
                    },
                    _ => unreachable!("LD {} only supports Reg8 and HLAddr.",
                        self.reg8_string(&reg_name))
                }
            },
            Operand::HLAddr => {
                match r2 {
                    Operand::Reg8(reg_name) => {
                        println!("LD (HL),{}", self.reg8_string(&reg_name));
                        let value = self.reg8_read(reg_name);
                        self.write_hladdr_u8(value);
                    },
                    Operand::Immediate8 => {
                        let value = self.mem.read_u8(self.pc+1);
                        println!("LD (HL),{:02X}", value);
                        self.write_hladdr_u8(value);
                        self.pc += 2;
                    },
                    _ => unreachable!("LD (HL) only supports Reg8 and Immediate8.")
                }
            },
            _ => unreachable!("LD only supports Reg8 and HLAddr.")
        };
        self.pc += 1;
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

    fn jr(&mut self, condition: Option<Condition>, operand: Operand) {
        match operand {
            Operand::Immediate8 => {
                let value = self.mem.read_u8(self.pc+1) as i8;
                let satisfied = match condition {
                    Some(cond) => {
                        println!("JR {},{:02X}", self.cond_string(&cond), value);
                        self.cond_eval(cond)
                    }
                    None => {
                        println!("JR {:02X}", value);
                        true
                    }
                };

                if satisfied {
                    if value > 0 {
                        self.pc += value as u16;
                    } else {
                        self.pc -= (value * -1) as u16;
                    }
                } else {
                    self.pc += 2;
                }
            },
            _ => unreachable!("JR only supports Immediate8.")
        };

        // Temp hack that prevents endless looping while waiting for vblank
        if self.pc == 105 {
            panic!();
        }
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

    fn add(&mut self, operand: Operand) {
        let orig = self.a;
        let value = match operand {
            Operand::Reg8(reg_name) => {
                println!("ADD A,{}", self.reg8_string(&reg_name));
                let value = self.reg8_read(reg_name);
                self.a += value;
                self.pc += 1;
                value
            },
            Operand::HLAddr => {
                println!("ADD A,(HL)");
                let value = self.read_hladdr_u8();
                self.a += value;
                self.pc += 1;
                value
            },
            Operand::Immediate8 => {
                let value = self.mem.read_u8(self.pc+1);
                println!("ADD A,{:02X}", value);
                self.a += value;
                self.pc += 2;
                value
            },
            _ => unreachable!("ADD only supports Reg8, HLAddr, and Immediate8.")
        };

        self.f = 0;
        if self.a == 0 {
            self.set_f_zero();
        }
        let half_carry = ((self.a as i8 & 0xF) + (value as i8 & 0xF)) & 0x10;
        if half_carry == 0x10 {
            self.set_f_halfcarry();
        }
        if self.a < orig {
            self.set_f_carry();
        }
    }

    fn sub(&mut self, operand: Operand) {
        let value = match operand {
            Operand::Reg8(reg_name) => {
                println!("SUB A,{}", self.reg8_string(&reg_name));
                let value = self.reg8_read(reg_name);
                self.a -= value;
                self.pc += 1;
                value
            },
            Operand::HLAddr => {
                println!("SUB A,(HL)");
                let value = self.read_hladdr_u8();
                self.a -= value;
                self.pc += 1;
                value
            },
            Operand::Immediate8 => {
                let value = self.mem.read_u8(self.pc+1);
                println!("SUB A,{:02X}", value);
                self.a -= value;
                self.pc += 2;
                value
            },
            _ => unreachable!("SUB only supports Reg8, HLAddr, and Immediate8.")
        };

        self.f = 0b01000_000u8;
        if value == self.a {
            self.set_f_zero();
        } else if self.a < value {
            self.set_f_carry();
        }
        let half_carry = ((self.a as i8 & 0xF) - (value as i8 & 0xF)) & 0x10;
        if half_carry == 0x10 {
            self.set_f_halfcarry();
        }
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
        };
        self.f = 0b0010_0000u8;
        if self.a == 0 {
            self.set_f_zero();
        }
    }

    fn xor(&mut self, operand: Operand) {
        match operand {
            Operand::Reg8(reg_name) => {
                println!("XOR {}", self.reg8_string(&reg_name));
                self.a ^= self.reg8_read(reg_name);
                self.pc += 1;
            },
            Operand::HLAddr => {
                println!("XOR (HL)");
                let value = self.read_hladdr_u8();
                self.a ^= value;
                self.pc += 1;
            },
            Operand::Immediate8 => {
                let value = self.mem.read_u8(self.pc+1);
                println!("XOR {:02X}", value);
                self.a ^= value;
                self.pc += 2;
            },
            _ => unreachable!("XOR only supports Reg8, HLAddr, and Immediate8.")
        };
        self.f = 0;
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
        };
        self.f = 0;
        if self.a == 0 {
            self.set_f_zero();
        }
    }

    fn cp(&mut self, operand: Operand) {
        let value = match operand {
            Operand::Reg8(reg_name) => {
                println!("CP {}", self.reg8_string(&reg_name));
                let value = self.reg8_read(reg_name);
                self.pc += 1;
                value
            },
            Operand::HLAddr => {
                println!("CP (HL)");
                let value = self.read_hladdr_u8();
                self.pc += 1;
                value
            },
            Operand::Immediate8 => {
                let value = self.mem.read_u8(self.pc+1);
                println!("CP {:02X}", value);
                self.pc += 2;
                value
            },
            _ => unreachable!("CP only supports Reg8, HLAddr, and Immediate8.")
        };

        // Remove later!
        // 0x91 = 145 = first line of vblank period.
        let hack = self.pc == 111 && value == 0x91;

        self.f = 0b01000_000u8;
        if value == self.a || hack {
            self.set_f_zero();
        } else if self.a < value {
            self.set_f_carry();
        }
        let half_carry = ((self.a as i8 & 0xF) - (value as i8 & 0xF)) & 0x10;
        if half_carry == 0x10 {
            self.set_f_halfcarry();
        }
    }
}
