#[repr(u32)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum AllocResult {
    Ok = 0,
    Empty = 1 << 0,
    InvalidAlignment = 1 << 1,
}

impl From<u32> for AllocResult {
    fn from(value: u32) -> Self {
        match value {
            0 => AllocResult::Ok,
            1 => AllocResult::Empty,
            2 => AllocResult::InvalidAlignment,
            _ => panic!(),
        }
    }
}
