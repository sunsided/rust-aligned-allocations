#[repr(u32)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum AllocResult {
    Ok = 0,
    Empty = 1 << 0,
    InvalidAlignment = 1 << 1,
}
