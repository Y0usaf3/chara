use surrealism::*;

#[surrealism]
pub fn and(x: u32, y: u32) -> u32 {
    x & y
}

#[surrealism]
pub fn or(x: u32, y: u32) -> u32 {
    x | y
}

#[surrealism]
pub fn xor(x: u32, y: u32) -> u32 {
    x ^ y
}

#[surrealism]
pub fn not(x: u32) -> u32 {
    !x
}

#[surrealism]
pub fn shl(x: u32, shift: u32) -> u32 {
    x << shift
}

#[surrealism]
pub fn shr(x: u32, shift: u32) -> u32 {
    x >> shift
}

#[surrealism]
pub fn can(mask: u32, flag: u32) -> bool {
    (mask & 1) == 1 || (mask & flag) == flag
}
