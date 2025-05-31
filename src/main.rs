#![no_std]
#![no_main]

use avr_device::attiny402 as device;

//type CoreClock = device::clock::MHz8;

// use arduino_hal as hal;

use arduino_hal::clock::Clock;
use embedded_hal::delay::DelayNs;

//use arduino_hal::prelude::*;
//use avr_hal_generic::prelude::*;

use panic_halt as _;

//pub type Delay = avr_hal_generic::delay::Delay<freq>;

pub fn delay_ms(ms: u32) {
    //Delay::new().delay_ms(ms);
    avr_device::asm::delay_cycles(8_000_000/1000 * ms);
}

//#[arduino_hal::entry]
#[avr_device::entry]
fn main() -> ! {

    let dp= avr_device::attiny402::Peripherals::take().unwrap();

    unsafe {
        dp.CPU.ccp.write(|w| w.bits(avr_device::attiny402::cpu::ccp::CCP_A::IOREG.into()));
        dp.CLKCTRL.mclkctrlb.write(|w| w.bits(0x1)); // change frequency divider from 6 to 2, so we get 16/2 = 8 Mhz
    }

    //let pins = dp.USART0.baud = ;

    dp.VPORTA.dir.write(|w| w.bits(128));

    loop {
            dp.VPORTA.out.write(|w| w.bits(128));

            delay_ms(10);

            dp.VPORTA.out.write(|w| w.bits(0));

            delay_ms(990);
    }
}
