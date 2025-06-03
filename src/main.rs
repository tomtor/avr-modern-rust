#![no_std]
#![no_main]

const LED: u8 = 0b1000_0000; // PA7

const FREQ: u32 = 8_000_000; // Must be 8 Mhz, this is hard wired in init_clock()

use core::convert::Infallible;

use avr_device::attiny402::{self as pac, vporta, Peripherals};
//use avr_device::avr128db28::{self as pac, vporta, Peripherals};

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
        set_high(&dp.VPORTA, LED);
        delay_ms(5);
        set_low(&dp.VPORTA, LED);
        delay_ms(100);
    }
}

pub fn init_clock(dp: &Peripherals) {
    dp.CPU.ccp().write(|w| w.ccp().ioreg()); // remove protection
    dp.CLKCTRL
        .mclkctrlb()
        .write(|w| w.pen().set_bit().pdiv()._2x()); // change frequency divider from 6 to 2, so we get 16/2 = 8 Mhz
}

pub fn delay_ms(ms: u32) {
    avr_device::asm::delay_cycles(FREQ / 1000 * ms);
}

struct Serial<'a> {
    p: &'a Peripherals,
}

impl<'a> Serial<'a> {
    pub fn new(dp: &'a Peripherals) -> Serial<'a> {
        dp.PORTA.out().write(|w| w.pa6().set_bit());
        dp.PORTA.dirset().write(|w| w.pa6().set_bit());
        dp.USART0.ctrlc().write(|w| w.chsize()._8bit());
        //unsafe { dp.USART0.baud().write(|w| w.bits(833)); } // 38400 baud
        unsafe {
            dp.USART0.baud().write(|w| w.bits(278));
        } // 115200 baud
        dp.USART0.ctrlb().write(|w| w.txen().set_bit());

        Serial { p: dp }
    }

    pub fn write_c(&self, b: u8) {
        while self.p.USART0.status().read().dreif() == false {} // Wait for empty transmit buffer
        unsafe {
            self.p.USART0.txdatal().write(|w| w.bits(b));
        }
    }

    pub fn write_ba(&self, s: &[u8]) {
        for b in s {
            self.write_c(*b);
        }
    }

    pub fn write_int(&self, i: u16) {
        if i > 9 {
            self.write_int(i / 10);
            self.write_int(i % 10);
        } else {
            self.write_c(b'0' + i as u8);
        }
    }
}

impl<'a> ufmt::uWrite for Serial<'a> {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.write_ba(s.as_bytes());
        Ok(())
    }
    
    fn write_char(&mut self, c: char) -> Result<(), Self::Error> {
        let mut buf: [u8; 4] = [0;4];
        self.write_str(c.encode_utf8(&mut buf)).unwrap();
        //self.write_c(c as u8);
        Ok(())
    }
}

impl<'a> embedded_io::ErrorType for Serial<'a> {
    type Error = Infallible;
}

impl<'a> embedded_io::Write for Serial<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.write_ba(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub fn set_high(r: &vporta::RegisterBlock, b: u8) {
    unsafe {
        r.out().modify(|r, w| w.bits(r.bits() | b));
    }
}
pub fn set_low(r: &vporta::RegisterBlock, b: u8) {
    unsafe {
        r.out().modify(|r, w| w.bits(r.bits() & !b));
    }
}
pub fn set(r: &vporta::RegisterBlock, b: u8, v: bool) {
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
        dp.VPORTA.dir().modify(|r, w| w.bits(r.bits() | LED));
    }
    assert!(FREQ == 8_000_000); // init_clock only works for 8Mhz. We check here so panic() can at least blink the LED

    let mut counter: u16 = 0;
    let mut f: f32 = 1.0;
    const SCALE: f32 = 1.01;

    loop {
        if f >= 65535.0 / 1000.0 {
            f = 1.0;
            counter = 0;
        }
        counter += 1;
        f *= SCALE;

        // write!(serial, "Counter: {:?} f: {:?} ", counter, f).unwrap();
        //write!(serial, "Counter: {:?} ", counter).unwrap();
        uwrite!(serial, "Counter: {:?} ", counter).unwrap();

        // serial.write_int(counter.into());
        //serial.write_str(" ");
        serial.write_char('€').unwrap();
        uwrite!(serial, "μ€ ").unwrap();
        serial.write_int((f * 1000.0) as u16);
        serial.write(b"\r\n").unwrap();

        set_high(&dp.VPORTA, LED);

        delay_ms(5);

        //set_low(&dp.VPORTA, LED); // or:
        set(&dp.VPORTA, LED, false);

        delay_ms(990);
    }
}
