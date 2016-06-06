mod util;
mod gameboy;
mod cpu;
mod mem;

use util::*;
use gameboy::*;

fn main() {
    let mut gameboy = GameBoy::new();
    let load_result = gameboy.load_rom("/home/teruko/blue.gb");
    println!("== {} ==", gameboy.game_title);
    if let Err(e) = load_result {
        match e {
            CartridgeValidationError::InvalidNintendoLogo => {
                println!(
r#"[Warning 00] Nintendo logo (0x104 .. 0x133) did not match expected value.
    Expected:
        CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
        00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
        BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E
    Found:
        {}
    This ROM will not run on a real Game Boy."#,
                    hexdump_slice(&gameboy.rom[0x104 .. 0x133+1])
                );
            },
            _ => println!("Unhandled error event.")
        }
    };
    gameboy.run()
}
