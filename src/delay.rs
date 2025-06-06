use avr_device::attiny402::{self as pac};
use avr_device::{self, interrupt};

use embedded_hal::delay::DelayNs;

//use core::intrinsics;

//use crate::serial::Serial;

pub struct Delay {}

impl DelayNs for Delay {
    fn delay_ns(&mut self, ns: u32) {
        avr_device::asm::delay_cycles(ns / (1_000_000_000 / crate::FREQ));
    }
}

pub fn delay_ms(ms: u32) {
    let mut d = Delay {};
    d.delay_ms(ms)
}

static mut SLEEP_CNT: u8 = 0; //AtomicU8 = AtomicU8::new(0);

/// Delays in sleep mode, using the low power RTC counter at 32768 Hz
pub fn sleep_delay(ms: u16) {
    // serial.write_ba(b"Start...\r\n");
    if ms == 0
       { return; }
    let dp = unsafe { pac::Peripherals::steal() };

    const RTC_PERIOD: u16 = 0x8000; // Overflow at 32768 ticks from RTC
    static mut MUST_INIT: bool = true;
    unsafe {
        if MUST_INIT == true {
            //serial.write_ba(b"Do init...\r\n");

            while dp.RTC.status().read().bits() > 0 {} /* Wait for all register to be synchronized */
            dp.RTC.per().write(|w| w.set(RTC_PERIOD - 1)); /* 1 sec overflow */
            dp.RTC.clksel().write(|w| w.bits(0)); /* 32.768kHz Internal Ultra-Low-Power Oscillator (OSCULP32K) */
            dp.RTC
                .ctrla()
                .write(|w| w.rtcen().set_bit().runstdby().set_bit());
            dp.SLPCTRL.ctrla().write(|w|w.smode().stdby());

            MUST_INIT = false;
        }
    }
    dp.ADC0.ctrla().modify(|_, w| w.enable().clear_bit()); // important on many AVR devices

    while dp.RTC.status().read().bits() > 0 {} // Wait for settings to sync

    interrupt::disable();

    let cnt: u16 = dp.RTC.cnt().read().bits();
    let tdelay = (ms as u32 * 32) + ((ms / 4 * 3) as u16 as u32);
    let cmp = (cnt + tdelay as u16) & (RTC_PERIOD - 1);

    dp.RTC.cmp().write(|w| w.set(cmp));
    // With this calculation every multiple of 4ms is exact!
    // Note 20011 ms = 655360 ticks = 20 overflows exactly, so CMP will be set to the current CNT and this works OK
    // 9005 will set CMP to CNT+1

    let mut adjust: u8 = 1;
    if dp.RTC.cnt().read().bits() == dp.RTC.cmp().read().bits() {
        adjust = 0;
    } else if ((dp.RTC.cmp().read().bits() - cnt) & (RTC_PERIOD - 1)) <= 2 {
        // overflow is/was near, 4Mhz clock or faster
        while dp.RTC.cnt().read().bits() != dp.RTC.cmp().read().bits() {} // Wait for it...
        adjust = 0;
    }

    unsafe {
            SLEEP_CNT = (tdelay / (RTC_PERIOD as u32)) as u8 + adjust; // Calculate number of wrap arounds (overflows)
    }

    dp.RTC.intflags().write(|w| w.cmp().set_bit());
    dp.RTC.intctrl().modify(|_, w| w.cmp().set_bit());

    unsafe {
        interrupt::enable();
    }

    unsafe {
        while SLEEP_CNT != 0 { // This works, but the following is better? The Atomic type is overkill for u8.
        // while intrinsics::volatile_load(&raw const SLEEP_CNT as *const u8) != 0 {
            avr_device::asm::sleep();
        }
    }
    dp.RTC.intctrl().modify(|_, w| w.cmp().clear_bit());
    dp.ADC0.ctrla().modify(|_, w| w.enable().set_bit()); // important on many AVR devices

    // serial.write_ba(b"The End!");
}

#[avr_device::interrupt(attiny402)]
fn RTC_CNT() {
    unsafe {
        SLEEP_CNT -= 1;
    }
    let dp = unsafe { pac::Peripherals::steal() };
    dp.RTC.intflags().write(|w| w.cmp().set_bit());
}
