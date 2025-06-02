#![no_std]
#![no_main]

//mod newport;

const LED: u8 = 0b1000_0000; // PA7

const FREQ: u32 = 8_000_000;

use avr_device::attiny402::{self as pac, vporta, Peripherals};
//use avr_device::avr128db28::{self as pac, vporta, Peripherals};

use heapless::String;
use ufmt::uwrite;

//use avr_hal_generic::prelude::*;

//use panic_halt as _;
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();

    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an unsafe
    // operation - but because no other code can run after the panic handler was called,
    // we know it is okay.
    let dp = unsafe { pac::Peripherals::steal() };
    //serial_str(&dp, &_info.message().as_str().unwrap());
    loop {
        set_high(&dp.VPORTA, LED);
        delay_ms(5);
        set_low(&dp.VPORTA, LED);
        delay_ms(100);
    }
}

pub fn init_clock(dp: &Peripherals) {
    dp.CPU.ccp().write(|w| w.ccp().ioreg()); // remove protection
    //assert!(CoreClock::FREQ == 8_000_000);
    dp.CLKCTRL
        .mclkctrlb()
        .write(|w| w.pen().set_bit().pdiv()._2x()); // change frequency divider from 6 to 2, so we get 16/2 = 8 Mhz
}

pub fn delay_ms(ms: u32) {
    //avr_device::asm::delay_cycles(CoreClock::FREQ / 1000 * ms);
    avr_device::asm::delay_cycles(FREQ / 1000 * ms);
}

// pub struct Serial<'a> {
//   usart: &'a avr_device::attiny402::Peripherals,
// }

// impl Serial<'a> {
//     pub fn new(usart: &avr_device::attiny402::Peripherals) -> Self {
//         Self { usart }
//     }
// }

pub fn init_serial(dp: &Peripherals) {
    dp.PORTA.out().write(|w| w.pa6().set_bit());
    dp.PORTA.dirset().write(|w| w.pa6().set_bit());
    dp.USART0.ctrlc().write(|w| w.chsize()._8bit());
    unsafe { dp.USART0.baud().write(|w| w.bits(833)); } // 38400 baud
    dp.USART0.ctrlb().write(|w| w.txen().set_bit());
}

pub fn serial_c(dp: &Peripherals, b: u8) {
    while dp.USART0.status().read().dreif() == false {} // Wait for empty transmit buffer
    unsafe { dp.USART0.txdatal().write(|w| w.bits(b)); }
}

pub fn serial_ba(dp: &Peripherals, s: &[u8]) {
    for b in s {
        serial_c(dp, *b);
    }
}

pub fn serial_int(dp: &Peripherals, i: u16) {
    if i > 9 {
        serial_int(dp, i / 10);
        serial_int(dp, i % 10);
    } else {
        serial_c(dp, b'0' + i as u8);
    }
}

pub fn serial_str(dp: &Peripherals, s: &str) {
    serial_ba(dp, s.as_bytes());
}

pub fn set_high(r: &vporta::RegisterBlock, b: u8) {
    unsafe { r.out().modify(|r, w| w.bits(r.bits() | b)); }
}
pub fn set_low(r: &vporta::RegisterBlock, b: u8) {
    unsafe { r.out().modify(|r, w| w.bits(r.bits() & !b)); }
}
pub fn set(r: &vporta::RegisterBlock, b: u8, v: bool) {
    unsafe { r.out()
        .modify(|r, w| w.bits(if v { r.bits() | b } else { r.bits() & !b })); }
}

//#[arduino_hal::entry]
#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    init_clock(&dp);
    init_serial(&dp);

    unsafe { dp.VPORTA.dir().modify(|r, w| w.bits(r.bits() | LED)); }

    let mut counter: u16 = 0;

    loop {
        counter += 1;
        //let mut s: String<7> = String::new();
        //uwrite!(s, "{:?}\r\n", counter).unwrap();
        //serial_str(&dp, &s);
        serial_int(&dp, counter.into());
        serial_str(&dp, "\r\n");

        set_high(&dp.VPORTA, LED);

        delay_ms(5);

        //set_low(&dp.VPORTA, LED); // or:
        set(&dp.VPORTA, LED, false);

        delay_ms(990);
    }
}
