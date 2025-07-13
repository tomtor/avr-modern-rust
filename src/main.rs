#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
//#![feature(core_intrinsics)]
// #![feature(asm_experimental_arch)]

const LED: u8 = 0b1000_0000; // PA7

const FREQ: u32 = 8_000_000; // Must be 8 Mhz, this is hard wired in init_clock()
                             // const FREQ: u32 = 4_000_000; // For AVR128DB28
                             // const FREQ: u32 = 12_000_000; // For AVR128DB28

mod delay;
mod io;
mod serial;

#[cfg(feature = "attiny1614")]
use avr_device::attiny1614::{self as pac, Peripherals};
#[cfg(feature = "attiny402")]
use avr_device::attiny402::{self as pac, Peripherals};
#[cfg(feature = "avr128db28")]
use avr_device::avr128db28::{self as pac, Peripherals};

use crate::io::{set, set_high, set_low};

use crate::serial::Serial;

use embedded_io::{self, Write};

//use heapless::String;

use ufmt::{uWrite, uwrite};

// use libm::{exp, floorf, sin, sqrtf};

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

static mut ID: u16 = 5;

#[avr_device::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    unsafe {
        #[cfg(feature = "avr128db28")]
        {
            dp.CPU.ccp().write(|w| w.ccp().ioreg()); // remove protection
            dp.NVMCTRL.ctrlb().write(|w| w.flmap().bits(0)); // Set the memory flash mapping for AVR128DB28
        }

        dp.PORTA.dir().modify(|r, w| w.bits(r.bits() | LED));
    }

    //assert!(FREQ == 8_000_000); // init_clock only works for 8Mhz. We check here so panic() can at least blink the LED

    init_clock(&dp);

    let mut serial = Serial::new(&dp);

    const NUMBERS: &[u16; 1000] = &[1; 1000];

    for ni in NUMBERS.iter() {
        serial.write_int(*ni);
        serial.write(b"\r\n").unwrap();
    }

    const SOME_STRING: &str = "This String wont ever change\r\n";
    serial.write_ba(SOME_STRING.as_bytes());

    let mut counter: u16 = 0;
    // let mut f: f32 = 1.0;
    // const SCALE: f32 = 1.01;

    loop {
        unsafe {
            ID += 1;
        }

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
        // serial.write_int((f * 1000.0) as u16);
        serial.write(b"\r\n").unwrap();
        // write!(serial, "f: {:?} sqrtf:  {:?}\r\n", f, sqrtf(f)).unwrap();
        // write!(serial, "f: {:?} floorf: {:?}\r\n", f, floorf(f)).unwrap();
        // write!(serial, "f: {:?} sin:    {:?}\r\n", f, sin(f as f64)).unwrap();
        // write!(serial, "f: {:?} exp:    {:?}\r\n", f, exp(f as f64)).unwrap();

        set_high(&dp.PORTA, LED);
        // The following line produces incorrect code: !!!
        //set_high_vp(&dp.VPORTA, LED);

        // delay::delay_ms(5);
        // delay::sleep_delay(3);
        delay::sleep_delay((counter & 0xf) + 1);

        //set_low(&dp.PORTA, LED); // or:
        set(&dp.PORTA, LED, false);

        // delay::delay_ms(990);
        delay::sleep_delay(390);
        // delay::sleep_delay(1990);
    }
}
