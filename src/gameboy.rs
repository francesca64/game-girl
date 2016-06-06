use std::io::Read;
use std::fs::File;
use std::str;
use util::*;
use cpu::*;

pub enum CartridgeValidationError {
    InvalidNintendoLogo,
    InvalidGameTitle(str::Utf8Error),
    /*InvalidSGBIndicator,
    InvalidCartridgeType,
    InvalidROMSize,
    InvalidRAMSize,
    InvalidLicenseeCode,
    FailedComplementCheck,
    FailedChecksumCheck*/
}

pub type CartridgeValidationResult<T> = Result<T, CartridgeValidationError>;

fn validate_nintendo_logo(logo: &[u8]) -> CartridgeValidationResult<()> {
    let expected: &[u8] = &[
        0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
        0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
        0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E
    ];
    if logo == expected {
        Ok(())
    } else {
        Err(CartridgeValidationError::InvalidNintendoLogo)
    }
}
/*
pub enum GameBoyError {

}

pub type GameBoyResult<T> = Result<T, GameBoyError>;
*/
pub struct GameBoy {
    pub rom: Vec<u8>,

    pub game_title: String,

    cartridge_type: u8,

    cpu: Cpu
}

impl Default for GameBoy {
    fn default() -> Self {
        GameBoy {
            rom: Vec::new(),

            game_title: String::new(),

            cartridge_type: 0x0,

            cpu: Cpu::new()
        }
    }
}

impl GameBoy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_rom(&mut self, rom_path: &str) -> CartridgeValidationResult<()> {
        self.rom = {
            let mut f = File::open(rom_path).expect("ROM file not found.");
            let mut v = Vec::new();
            f.read_to_end(&mut v).expect("ROM reading failed.");
            v
        };

        self.game_title = try!(str::from_utf8(&self.rom[0x134 .. 0x142+1])
            .map_err(CartridgeValidationError::InvalidGameTitle)).to_string();

        try!(validate_nintendo_logo(&self.rom[0x104 .. 0x133+1]));

        self.cartridge_type = get_u8(&self.rom, 0x147);

        Ok(())
    }

    pub fn run(&mut self) {
        self.cpu.run(&self.rom);
    }
}
