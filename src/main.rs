#![no_std]
#![no_main]

const LED: u8 = 0b1000_0000; // PA7

type CoreClock = avr_hal_generic::clock::MHz8;

use avr_device::attiny402::{self as device, vporta::RegisterBlock, Peripherals};

use avr_hal_generic::clock::Clock;

//use avr_hal_generic::prelude::*;

use panic_halt as _;

pub fn init_clock(dp: &Peripherals) {
    dp.CPU.ccp.write(|w| w.ccp().ioreg()); // remove protection
    assert!(CoreClock::FREQ == 8_000_000);
    dp.CLKCTRL.mclkctrlb.write(|w| w.pen().bit(true)); // change frequency divider from 6 to 2, so we get 16/2 = 8 Mhz
}

pub fn delay_ms(ms: u32) {
    avr_device::asm::delay_cycles(CoreClock::FREQ / 1000 * ms);
}

pub fn set_high(r: &RegisterBlock, b: u8) {
        r.out.modify(|r, w| w.bits(r.bits() | b));
}
pub fn set_low(r: &RegisterBlock, b: u8) {
        r.out.modify(|r, w| w.bits(r.bits() & !b));
}
pub fn set(r: &RegisterBlock, b: u8, v: bool) {
        r.out.modify(|r, w| w.bits(if v { r.bits() | b} else {r.bits() & !b}));
}

//#[arduino_hal::entry]
#[avr_device::entry]
fn main() -> ! {
    let dp = device::Peripherals::take().unwrap();

    init_clock(&dp);

    //let pins = dp.USART0.baud = ;

    dp.VPORTA.dir.modify(|r, w| w.bits(r.bits() | LED));

    loop {
        set_high(&dp.VPORTA, LED);

        delay_ms(10);

        //set_low(&dp.VPORTA, LED); // or:
        set(&dp.VPORTA, LED, false);

        delay_ms(990);
    }
}
