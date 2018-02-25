#[repr(C)]
#[derive(Copy, Clone)]
pub enum Clock {
    AES         = 0x40300,
    ACMP0       = 152064,
    ADC0        = 172544,
    LFA         = 0x60002,
    LFB         = 0x120003,
    LFC         = 0x1E0006,
    I2C0        = 0x2b200,
    CORE        = 0x40020,
    DMA         = 0x41300,
    HFPER       = 0x28110,
    GPIO        = 0x28200,
    CORELE      = 0x42300,
    HF          = 0x51,
    TIMER0      = 0x20200,
    TIMER1      = 0x21200,
    TIMER2      = 0x22200,
    USART0      = 0x23200,
    USART1      = 0x24200,
    PRS         = 0x26200,
    RTC         = 0x80430,
    LEUART0     = 0x140540,
    DBG         = 0x180004,
    AUX         = 0x1A0000,
    IDAC0       = 0x27200,
    VCMP        = 0x29200,
    PCNT0       = 0x60600,
}

/** High frequency RC bands. */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum HFRCOBand {
    _1MHz  = 0x0000_0000,
    _7MHz  = 0x0000_0001,
    _11MHz = 0x0000_0002,
    _14MHz = 0x0000_0003,
    _21MHz = 0x0000_0004,
}

/** Oscillator types. */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Osc {
    LFXO,
    LFRCO,
    HFXO,
    HFRCO,
    AUXHFRCO,
    ULFRCO,
}

/** Selectable clock sources. */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Select {
    Error,
    Disabled,
    LFXO,
    LFRCO,
    HFXO,
    HFRCO,
    CORELEDIV2,
    AUXHFRCO,
    HFCLK,
    ULFRCO,
}

#[link(name = "emlib")]
extern {
    pub fn CMU_ClockEnable(clock: Clock, enable: bool);
    pub fn CMU_ClockFreqGet(clock: Clock) -> u32;
    pub fn CMU_ClockSelectGet(clock: Clock) -> Select;
    pub fn CMU_ClockSelectSet(clock: Clock, reference: Select);
    pub fn CMU_ClockDivSet(clock: Clock, div: u32);
    pub fn CMU_HFRCOBandSet(band: HFRCOBand);
    pub fn CMU_OscillatorEnable(osc: Osc, enable: bool, wait: bool);
}

pub fn clock_enable(clock: Clock, enable: bool) {
    unsafe { CMU_ClockEnable(clock, enable) }
}

pub fn clock_freq_get(clock: Clock) -> u32 {
    unsafe { CMU_ClockFreqGet(clock) }
}

pub fn clock_select_get(clock: Clock) -> Select {
    unsafe { CMU_ClockSelectGet(clock) }
}

pub fn clock_select_set(clock: Clock, reference: Select) {
    unsafe { CMU_ClockSelectSet(clock, reference) }
}

pub fn clock_div_set(clock: Clock, div: u32) {
    unsafe { CMU_ClockDivSet(clock, div) }
}

pub fn hfrco_band_set(band: HFRCOBand) {
    unsafe { CMU_HFRCOBandSet(band) }
}

pub fn oscillator_enable(osc: Osc, enable: bool, wait: bool) {
    unsafe { CMU_OscillatorEnable(osc, enable, wait) }
}
