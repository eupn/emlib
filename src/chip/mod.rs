#[link(name = "emlib")]
extern {
    pub fn CHIP_Init();
}

pub fn init() {
    unsafe { CHIP_Init(); }
}
