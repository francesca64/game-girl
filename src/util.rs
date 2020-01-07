pub fn get_u8(bytes: &[u8], offset: usize) -> u8 {
    bytes[offset]
}

#[allow(identity_op)]
pub fn get_u16(bytes: &[u8], offset: usize) -> u16 {
    u16_from_2u8s((bytes[offset+0], bytes[offset+1]))
}

#[allow(identity_op)]
pub fn get_u32(bytes: &[u8], offset: usize) -> u32 {
      (bytes[offset+3] as u32) << (8*3)
    | (bytes[offset+2] as u32) << (8*2)
    | (bytes[offset+1] as u32) << (8*1)
    | (bytes[offset+0] as u32) << (8*0)
}

#[allow(identity_op)]
pub fn get_u64(bytes: &[u8], offset: usize) -> u64 {
      (bytes[offset+7] as u64) << (8*7)
    | (bytes[offset+6] as u64) << (8*6)
    | (bytes[offset+5] as u64) << (8*5)
    | (bytes[offset+4] as u64) << (8*4)
    | (bytes[offset+3] as u64) << (8*3)
    | (bytes[offset+2] as u64) << (8*2)
    | (bytes[offset+1] as u64) << (8*1)
    | (bytes[offset+0] as u64) << (8*0)
}

#[allow(identity_op)]
pub fn switch_u16(value: u16) -> u16 {
    let b = u16_to_2u8s(value);
    u16_from_2u8s((b.0, b.1))
}

#[allow(identity_op)]
pub fn u16_to_2u8s(n: u16) -> (u8, u8) {
    (
        (n >> (8*1)) as u8,
        (n >> (8*0)) as u8
    )
}

#[allow(identity_op)]
pub fn u16_from_2u8s(nn: (u8, u8)) -> u16 {
      (nn.1 as u16) << (8*1)
    | (nn.0 as u16) << (8*0)
}

#[allow(identity_op)]
pub fn u32_to_4u8s(n: u32) -> (u8, u8, u8, u8) {
    (
        (n >> (8*3)) as u8,
        (n >> (8*2)) as u8,
        (n >> (8*1)) as u8,
        (n >> (8*0)) as u8
    )
}

#[allow(identity_op)]
pub fn u64_to_8u8s(n: u64) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    (
        (n >> (8*7)) as u8,
        (n >> (8*6)) as u8,
        (n >> (8*5)) as u8,
        (n >> (8*4)) as u8,
        (n >> (8*3)) as u8,
        (n >> (8*2)) as u8,
        (n >> (8*1)) as u8,
        (n >> (8*0)) as u8
    )
}

pub fn hexdump_slice(bytes: &[u8]) -> String {
    let mut x = 0;
    let mut y = 0;
    let strs: Vec<String> = bytes.iter()
        .map(|b| {
            if x != 15 || y == 2 {
                x += 1;
                format!("{:02X}", b)
            } else {
                x = 0; y += 1;
                format!("{:02X}\n       ", b)
            }
        })
        .collect();
    strs.join(" ")
}
