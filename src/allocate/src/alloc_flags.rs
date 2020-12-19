bitflags! {
    pub struct AllocFlags: u32 {
        const NONE = 0;
        const HUGE_PAGES = 1 << 0;
    }
}
