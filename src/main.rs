#![no_std]
#![no_main]

//mod newport;

const LED: u8 = 0b1000_0000; // PA7

type CoreClock = avr_hal_generic::clock::MHz8;

use avr_device::attiny402::{self as pac, vporta, Peripherals};

use avr_hal_generic::clock::Clock;

pub use avr_hal_generic::port::{mode, PinMode, PinOps};

//use avr_hal_generic::prelude::*;

use panic_halt as _;

//use heapless:String;

pub fn init_clock(dp: &Peripherals) {
    dp.CPU.ccp.write(|w| w.ccp().ioreg()); // remove protection
    assert!(CoreClock::FREQ == 8_000_000);
    dp.CLKCTRL.mclkctrlb.write(|w| w.pen().bit(true)); // change frequency divider from 6 to 2, so we get 16/2 = 8 Mhz
}

pub fn delay_ms(ms: u32) {
    avr_device::asm::delay_cycles(CoreClock::FREQ / 1000 * ms);
}

pub fn init_serial(dp: &Peripherals) {
    const TX: u8 = 0b0100_0000; // PA6

    set_high(&dp.VPORTA, TX);
    dp.VPORTA.dir.modify(|r, w| w.bits(r.bits() | TX));

    unsafe {
        dp.USART0.ctrlc.write(|w| w.bits(0x3)); // 8 bit data
    }
    dp.USART0.baud.write(|w| w.bits(833)); // 38400
    unsafe {
        dp.USART0
            .ctrlb
            .modify(|r, w| w.bits(r.bits() | 0b0100_0000)); // enable TX
    }
}

pub fn serial_write(dp: &Peripherals, b: u8) {
    while (dp.USART0.status.read().bits() & 0b0010_0000) == 0 {} // Wait for empty tansmit buffer
    dp.USART0.txdatal.write(|w| w.bits(b));
}

pub fn set_high(r: &vporta::RegisterBlock, b: u8) {
    r.out.modify(|r, w| w.bits(r.bits() | b));
}
pub fn set_low(r: &vporta::RegisterBlock, b: u8) {
    r.out.modify(|r, w| w.bits(r.bits() & !b));
}
pub fn set(r: &vporta::RegisterBlock, b: u8, v: bool) {
    r.out
        .modify(|r, w| w.bits(if v { r.bits() | b } else { r.bits() & !b }));
}

//#[arduino_hal::entry]
#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    init_clock(&dp);
    init_serial(&dp);

    dp.VPORTA.dir.modify(|r, w| w.bits(r.bits() | LED));

    let mut counter: u8 = 0;

    loop {
        counter += 1;
        serial_write(&dp, 0x30 + (counter % 10));
        serial_write(&dp, b'\r');
        serial_write(&dp, b'\n');

        set_high(&dp.VPORTA, LED);

        delay_ms(5);

        //set_low(&dp.VPORTA, LED); // or:
        set(&dp.VPORTA, LED, false);

        delay_ms(990);
    }
}
