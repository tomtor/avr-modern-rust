#![no_std]
#![no_main]

#![feature(abi_avr_interrupt)]
//#![feature(core_intrinsics)]
// #![feature(asm_experimental_arch)]

const LED: u8 = 0b1000_0000; // PA7

const FREQ: u32 = 8_000_000; // Must be 8 Mhz, this is hard wired in init_clock()

mod delay;
mod serial;

use avr_device::{attiny402::{self as pac, porta, Peripherals}};

use crate::serial::Serial;

use embedded_io::{self, Write};

//use heapless::String;

use ufmt::{uWrite, uwrite};

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
    //write_str(&dp, &_info.message().as_str().unwrap());
    loop {
        set_high(&dp.PORTA, LED);
        delay::delay_ms(5);
        set_low(&dp.PORTA, LED);
        delay::delay_ms(100);
    }
}

pub fn init_clock(dp: &Peripherals) {
    dp.CPU.ccp().write(|w| w.ccp().ioreg()); // remove protection
    dp.CLKCTRL
        .mclkctrlb()
        .write(|w| w.pen().set_bit().pdiv()._2x()); // change frequency divider from 6 to 2, so we get 16/2 = 8 Mhz
}


pub fn set_high(r: &porta::RegisterBlock, b: u8) {
    unsafe {
        r.out().modify(|r, w| w.bits(r.bits() | b));
    }
}
pub fn set_low(r: &porta::RegisterBlock, b: u8) {
    unsafe {
        r.out().modify(|r, w| w.bits(r.bits() & !b));
    }
}
pub fn set(r: &porta::RegisterBlock, b: u8, v: bool) {
    unsafe {
        r.out()
            .modify(|r, w| w.bits(if v { r.bits() | b } else { r.bits() & !b }));
    }
}


#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    init_clock(&dp);

    let mut serial = Serial::new(&dp);

    unsafe {
        dp.PORTA.dir().modify(|r, w| w.bits(r.bits() | LED));
    }
    assert!(FREQ == 8_000_000); // init_clock only works for 8Mhz. We check here so panic() can at least blink the LED

    let mut counter: u16 = 0;
    // let mut f: f32 = 1.0;
    // const SCALE: f32 = 1.01;

    loop {
        // if f >= 65535.0 / 1000.0 {
        //     f = 1.0;
        //     counter = 0;
        // }
        counter += 1;
        // f *= SCALE;

        // write!(serial, "Counter: {:?} f: {:?} ", counter, f).unwrap();
        // write!(serial, "Counter: {:?} ", counter).unwrap();
        uwrite!(serial, "Counter: {:?} ", counter).unwrap();

        serial.write_int(2 * counter);
        serial.write_str(" ").unwrap();
        serial.write_char('€').unwrap();
        uwrite!(serial, "μ€ ").unwrap();
        //serial.write_int((f * 1000.0) as u16);
        serial.write(b"\r\n").unwrap();

        set_high(&dp.PORTA, LED);

        // delay::delay_ms(5);
        // delay::sleep_delay(3);
        delay::sleep_delay((counter & 0xf) + 1);

        //set_low(&dp.PORTA, LED); // or:
        set(&dp.PORTA, LED, false);

        //delay::delay_ms(990);
        delay::sleep_delay(390);
    }
}
