use util::*;
use std::io::{Write, Seek, SeekFrom};
use std::fs::File;

pub struct Mem {
    fixed_rom_bank: Vec<u8>,
    switchable_rom_bank: Vec<u8>,
    vram: Vec<u8>,
    switchable_ram: Vec<u8>,
    internal_ram_8kb: Vec<u8>,
    io_ports: Vec<u8>,
    high_ram: Vec<u8>
}

impl Default for Mem {
    fn default() -> Self {
        Mem {
            fixed_rom_bank: Vec::with_capacity(0x4000),
            switchable_rom_bank: Vec::with_capacity(0x4000),
            vram: vec![0x0; 0x2000+1],
            switchable_ram: vec![0x0; 0x2000+1],
            internal_ram_8kb: vec![0x0; 0x2000+1],
            io_ports: vec![0x0; 0x4C+1],
            high_ram: vec![0x0; 0x7F+1]
        }
    }
}

impl Mem {
    pub fn new() -> Self {
        Mem::default()
    }

    pub fn load_fixed_rom_bank(&mut self, contents: &[u8]) -> () {
        self.fixed_rom_bank.extend_from_slice(contents)
    }

    fn memory_map(&self, addr: u16) -> (&[u8], usize) {
        if addr <= 0x4000 {
            (&self.fixed_rom_bank, addr as usize)
        } else if addr <= 0x8000 {
            (&self.switchable_rom_bank, (addr-0x4000) as usize)
        } else if addr <= 0xA000 {
            (&self.vram, (addr-0x8000) as usize)
        } else if addr <= 0xC000 {
            (&self.switchable_ram, (addr-0xA000) as usize)
        } else if addr <= 0xE000 {
            (&self.internal_ram_8kb, (addr-0xC000) as usize)
        } else if addr <= 0xFE00 {
            panic!("Memory address in echo, which is unimplemented. {}", addr);
        } else if addr <= 0xFEA0 {
            panic!("Memory address in OAM, which is unimplemented. {}", addr);
        } else if addr <= 0xFF00 {
            panic!("Memory address in empty and unusable #1, which is unimplemented. {}", addr);
        } else if addr <= 0xFF4C {
            //println!("IO Read {} ({})", hexdump(addr), addr-0xFF00);
            (&self.io_ports, (addr-0xFF00) as usize)
        } else if addr <= 0xFF80 {
            panic!("Memory address in empty and unusable #2, which is unimplemented. {}", addr);
        } else {
            (&self.high_ram, (addr-0xFF80) as usize)
        }
    }

    fn memory_map_mut(&mut self, addr: u16) -> (&mut [u8], usize) {
        if addr <= 0x4000 {
            (&mut self.fixed_rom_bank, addr as usize)
        } else if addr <= 0x8000 {
            (&mut self.switchable_rom_bank, (addr-0x4000) as usize)
        } else if addr <= 0xA000 {
            (&mut self.vram, (addr-0x8000) as usize)
        } else if addr <= 0xC000 {
            (&mut self.switchable_ram, (addr-0xA000) as usize)
        } else if addr <= 0xE000 {
            (&mut self.internal_ram_8kb, (addr-0xC000) as usize)
        } else if addr <= 0xFE00 {
            panic!("Memory address in echo, which is unimplemented. {}", addr);
        } else if addr <= 0xFEA0 {
            panic!("Memory address in OAM, which is unimplemented. {}", addr);
        } else if addr <= 0xFF00 {
            panic!("Memory address in empty and unusable #1, which is unimplemented. {}", addr);
        } else if addr <= 0xFF4C {
            //println!("IO Write {} ({})", hexdump(addr), addr-0xFF00);
            (&mut self.io_ports, (addr-0xFF00) as usize)
        } else if addr <= 0xFF80 {
            panic!("Memory address in empty and unusable #2, which is unimplemented. {}", addr);
        } else {
            (&mut self.high_ram, (addr-0xFF80) as usize)
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        let pair = self.memory_map(addr);
        pair.0[pair.1]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        let pair = self.memory_map(addr);
        get_u16(pair.0, pair.1)
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        let pair = self.memory_map_mut(addr);
        pair.0[pair.1] = value;
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        let pair = self.memory_map_mut(addr);
        let values = u16_to_2u8s(value);
        pair.0[pair.1] = values.0;
        pair.0[pair.1+1] = values.1;
    }

    pub fn dump(&self) {
        let mut f = File::create("memdump.bin").unwrap();
        f.write_all(&self.fixed_rom_bank).unwrap();
        f.seek(SeekFrom::Start(0x4000)).unwrap();
        f.write_all(&self.switchable_rom_bank).unwrap();
        f.seek(SeekFrom::Start(0x8000)).unwrap();
        f.write_all(&self.vram).unwrap();
        f.seek(SeekFrom::Start(0xA000)).unwrap();
        f.write_all(&self.switchable_ram).unwrap();
        f.seek(SeekFrom::Start(0xE000)).unwrap();
        f.write_all(&self.internal_ram_8kb).unwrap();
        f.seek(SeekFrom::Start(0xFF4C)).unwrap();
        f.write_all(&self.io_ports).unwrap();
        f.seek(SeekFrom::Start(0xFF80)).unwrap();
        f.write_all(&self.high_ram).unwrap();
    }
}
